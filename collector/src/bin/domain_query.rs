extern crate clap;
extern crate damp;
extern crate failure;

use clap::{App, Arg};
use damp::model::connect;
use damp::model::domain::Domain;
use damp::model::record::{NewRecord, Record};
use damp::{end_processing_marker, schema, start_processing_marker, unix_time};
use diesel::prelude::*;
use diesel::QueryDsl;
use failure::Error;
use maxminddb::geoip2::Isp;
use maxminddb::Reader;
use std::net::SocketAddr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::DnsResponse;
use trust_dns::rr::{DNSClass, Name, RData, RecordType};
use trust_dns::udp::UdpClientConnection;

static QUERY_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static QUERY_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
static QUERY_ABOUT: &'static str = r#"
With this binary we take a list of domains in a SQLite database loaded by the
domain_loader(8) binary and for each run a set of queries against the provided
resolver:
* A
* AAAA
* NS

It is not a requirement that the machine have IPv6 present in order to complete
the AAAA queries, however the name server in use must answer correctly in spite
of our network potentially not being IPv6 capable.

For the sakes of keeping the code complexity low, this process is both
synchronous and blocking in nature and any delays by one query will slow down or
stop subsequent requests.
"#;

struct DnsQuery {
    dns_client: SyncClient<UdpClientConnection>,
    sql_client: SqliteConnection,
    maxmind: Reader<Vec<u8>>,
    query_types: Vec<RecordType>,
}

impl DnsQuery {
    /// Returns a struct that handles all the various network and DB calls.
    ///
    /// # Arguments:
    /// * `dns_resolver`: The socket address to a Do53 service
    /// * `sql_db`: Path to SQLite database
    /// * `mmdb`: Path to Maxmind GeoLite2 ASN database
    ///
    /// # Example
    /// ```
    /// use std::net::SocketAddr;
    ///
    /// let addr = "127.0.0.1:53".parse().unwrap();
    /// let path = "tests/fixtures/GeoLite2-ASN-Test.mmdb";
    /// let sql = ":memory:".to_string();
    /// let d_q = Query::new(addr, sql, path);
    /// ```
    pub fn new(dns_resolver: SocketAddr, sql_db: &str, mmdb: &str) -> Result<DnsQuery, Error> {
        let dns_conn = UdpClientConnection::new(dns_resolver).expect("Unable to use DNS client!");
        let mmdb = Reader::open_readfile(mmdb)?;
        Ok(DnsQuery {
            dns_client: SyncClient::new(dns_conn),
            sql_client: connect(sql_db.to_string()),
            maxmind: mmdb,
            query_types: vec![RecordType::A, RecordType::AAAA, RecordType::NS],
        })
    }

    /// Perform queries of all query types.
    ///
    /// # Arguments
    /// * `domain` - The [Domain](crate::model::domain::Domain)
    /// * `query_type` - The DNS RecordType to query
    /// * `is_www` - If set true, query against 'www.' of the domain
    pub fn query_domain(&self, domain: &Domain, query_type: RecordType, is_www: bool) {
        let query: String = match is_www {
            true => format!("www.{}", domain.clone().fqdn),
            false => domain.clone().fqdn.clone(),
        };
        let name: Name = Name::from_ascii(query).unwrap();
        // FIXME: Remove unwrap here
        let mut query_time = unix_time();
        let response: DnsResponse = self
            .dns_client
            .query(&name, DNSClass::IN, query_type)
            .unwrap();

        self.insert_record(&response, &domain.rowid, &is_www, &query_time, None);

        let parent_rowid = self.get_last_row();

        // Process NS records and convert the host name returned into A/AAAA records
        if query_type == RecordType::NS {
            for answer in response.answers().iter() {
                query_time = unix_time();
                let answer_name = self.parse_address(answer.rdata()).unwrap();
                let ns_name: Name = Name::from_ascii(answer_name).unwrap();

                let a_res: DnsResponse = self
                    .dns_client
                    .query(&ns_name, DNSClass::IN, RecordType::A)
                    .unwrap();

                self.insert_record(&a_res, &domain.rowid, &false, &query_time, parent_rowid.as_ref(),);

                let aaaa_res: DnsResponse = self
                    .dns_client
                    .query(&ns_name, DNSClass::IN, RecordType::AAAA)
                    .unwrap();
                self.insert_record(&aaaa_res, &domain.rowid, &false, &query_time, parent_rowid.as_ref());
            }
        }
    }

    fn insert_record(
        &self,response: &DnsResponse, row_id: &i64, is_www: &bool, query_time: &i64, parent: Option<&i64>) {
        let mut parent_record: Option<i64> = parent.cloned();
        for answer in response.answers().iter() {
            let asn: Option<i32> = match answer.rdata().to_ip_addr() {
                Some(ip) => match self.maxmind.lookup::<Isp>(ip) {
                    Ok(res) => res.autonomous_system_number.map(|m| m as i32),
                    Err(_) => None,
                },
                None => None,
            };
            let address = self.parse_address(answer.rdata());
            let record_type = answer.record_type().to_string();
            let ttl = answer.ttl() as i32;
            let record = NewRecord {
                domain: row_id,
                is_www,
                parent: parent_record.as_ref(),
                response_code: &(response.response_code() as i32),
                record_type: Some(&record_type),
                ttl: Some(&ttl),
                address: address.as_ref().map(String::as_str),
                asn: asn.as_ref(),
                query_time,
            };

            match diesel::insert_into(schema::record::table)
                .values(&record)
                .execute(&self.sql_client)
            {
                Ok(_) => {
                    // Set the parent to the value of the row just inserted. As we're
                    // presently single-threaded, the 'highest' rowid should be the one.
                    // It's worth noting that the authors of diesel explicitly oppose to
                    // the use of `sqlite3_last_insert_rowid`.
                    //
                    // See Also: https://github.com/diesel-rs/diesel/issues/376
                    match parent_record {
                        None => {
                            parent_record = self.get_last_row();
                        }
                        Some(_) => {}
                    }
                }
                Err(e) => println!("Unable to insert record - {}", e.to_string()),
            };
        }
    }

    fn get_last_row(&self) -> Option<i64> {
        use damp::schema::record::dsl::*;
        let parent_rowid = record
            .order(rowid.desc())
            .first::<Record>(&self.sql_client)
            .unwrap()
            .rowid;
        return Some(parent_rowid);
    }

    /// Process all domains and get all records for the given query types.
    /// The structure of the queries we'll do looks like:
    /// ```text
    ///     apex ─┬──  A
    ///           ├──  AAAA
    ///           ├──  NS
    ///           │    ├── A
    ///           │    └── AAAA
    ///           └──  www
    ///                ├── A
    ///                └── AAAA
    /// ```
    /// There is no need for performing NS queries against www as we assume that nobody is
    /// (arguably mis-)configuring their DNS hierarchy to put www as apex in a delegate zone.
    /// Also, to aid better identification of the name servers, we perform an A and AAAA query
    /// against any hosts in the NS set.
    pub fn process_all(&self) {
        use damp::schema::domain::dsl::*;
        let domains = domain.load::<Domain>(&self.sql_client).unwrap();

        for d in &domains {
            for query_type in &self.query_types {
                self.query_domain(d, *query_type, false);
                // Perform www queries
                match query_type {
                    RecordType::A | RecordType::AAAA => self.query_domain(d, *query_type, true),
                    _ => {}
                }
            }
        }
    }

    /// Return total number of domains available to query in our dataset
    /// TODO: Consider pagination or breaking up queries
    pub fn total_domains(&self) -> i64 {
        use damp::schema::domain::dsl::*;
        let count = domain.count().load(&self.sql_client).unwrap();
        return count[0];
    }

    /// Using the type of record, convert the RData into a String
    ///
    /// # Arguments
    /// * `answer` - The DNS Answer containing the record details
    pub fn parse_address(&self, answer: &RData) -> Option<String> {
        return match answer {
            // TODO: DNAME?
            RData::CNAME(name) | RData::NS(name) => Some(name.to_ascii()),
            RData::A(ip) => Some(ip.to_string()),
            RData::AAAA(ip) => Some(ip.to_string()),
            _ => None,
        };
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new("domain_query")
        .version(QUERY_VERSION)
        .author(QUERY_AUTHORS)
        .about(QUERY_ABOUT)
        .arg(
            Arg::with_name("sqlite-db")
                .help("Path to SQLite database")
                .long("sqlite-db")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("asn-db")
                .help("Path to MaxMind GeoLite2 ASN Database")
                .long("asn-db")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("resolver")
                .help("IP address of DNS resolver to query, including port.")
                .long("resolver")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let resolver: SocketAddr = matches.value_of("resolver").unwrap().parse().unwrap();
    let sqlite_db = matches.value_of("sqlite-db").unwrap();
    let asn_db = matches.value_of("asn-db").unwrap();

    let dns_query = DnsQuery::new(resolver, sqlite_db, asn_db)?;

    // --------------------------
    //     Start of processing
    // --------------------------
    let start = start_processing_marker(format!(
        "Querying domains using resolver {} into {}",
        resolver.to_string(),
        sqlite_db
    ));

    let total_domains = dns_query.total_domains();
    println!("Processing {} domains", total_domains);

    dns_query.process_all();

    // --------------------------
    //       End of processing
    // --------------------------
    end_processing_marker("Querying completed", start);

    Ok(())
}
