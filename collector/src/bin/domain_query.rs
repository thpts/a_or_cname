extern crate clap;
extern crate damp;
extern crate failure;

use clap::{App, Arg};
use damp::model::connect;
use damp::model::domain::Domain;
use damp::model::record::NewRecord;
use damp::{end_processing_marker, schema, start_processing_marker};
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

    /// Perform queries of all query types, returning a vector containing all
    /// the answers for the given domain.
    ///
    /// # Arguments
    /// * `domain` - The [Domain](crate::model::domain::Domain)
    pub fn query_domain(&self, domain: Domain) {
        let name: Name = Name::from_ascii(domain.fqdn).unwrap();
        for query_type in &self.query_types {
            // FIXME: Remove unwrap here
            let response: DnsResponse = self
                .dns_client
                .query(&name, DNSClass::IN, *query_type)
                .unwrap();
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
                    domain: &domain.rowid,
                    parent: None, // TODO
                    response_code: &(response.response_code() as i32),
                    record_type: Some(&record_type),
                    ttl: Some(&ttl),
                    address: address.as_ref().map(String::as_str),
                    asn: asn.as_ref(),
                    query_time: &0,
                };

                diesel::insert_into(schema::record::table)
                    .values(&record)
                    .execute(&self.sql_client);
            }
        }
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

    

    // TODO processing here

    // --------------------------
    //       End of processing
    // --------------------------
    end_processing_marker("Querying completed", start);

    Ok(())
}
