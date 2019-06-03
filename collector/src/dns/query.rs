use trust_dns::client::{Client, SyncClient};
use trust_dns::udp::UdpClientConnection;

use std::net::SocketAddr;
use std::str::FromStr;
use trust_dns::rr::{RecordType, DNSClass, Name};
use crate::model::record::NewRecord;
use maxminddb::Reader;
use trust_dns::op::DnsResponse;
use crate::model::domain::Domain;

pub struct Query {
    /// DNS Client
    client: SyncClient<UdpClientConnection>,

    /// Maxmind DB client
    maxmind: Reader<Vec<u8>>,

    /// Query types to perform in [query_all](Query::query_all)
    query_types: Vec<RecordType>,
}

impl Query {
    /// Returns a wrapper around the `trust_dns` synchronous client to facilitate
    /// helper methods.
    ///
    /// # Arguments:
    /// * `resolver` - host name to query, including the port number. For IPv6
    ///                hosts, please wrap the IP address with square brackets.
    /// * `maxminddb` - path to the Maxmind GeoIP2 ASN database.
    ///
    /// # Example
    /// ```
    /// use query::Query;
    /// use std::net::SocketAddr;
    ///
    /// let addr = "127.0.0.1:53";
    /// let path = "/tmp/fake.mmdb";
    /// let client = Query::new(addr, &path);
    /// ```
    pub fn new(resolver: SocketAddr, maxminddb: &str) -> Self {
        let conn = UdpClientConnection::new(resolver).unwrap();

        Query {
            client: SyncClient::new(conn),
            maxmind: maxminddb::Reader::open_readfile(maxminddb).unwrap(),
            query_types: vec![RecordType::A, RecordType::AAAA, RecordType::NS],
        }
    }

    /// Perform queries of all query types, returning a vector containing all
    /// the answers for the given domain.
    ///
    /// # Arguments
    /// * `domain` - The [Domain](crate::model::domain::Domain)
    pub fn query_all(&self, domain: Domain) -> Option<Vec<NewRecord>> {
        let name: Name = match Name::from_str(domain.fqdn.as_ref()) {
            Ok(n) => n,
            Err(_) => {
                return None
            }
        };
        let mut records = Vec::new();
        for query_type in &self.query_types {

            // FIXME: Remove unwrap here
            let response: DnsResponse = self.client.query(&name, DNSClass::IN, *query_type).unwrap();
            for answer in response.answers().iter() {

                // TODO: ASN lookup here

                // TODO correct record details
                records.push(NewRecord {
                    domain: &domain.rowid,
                    parent: &0,
                    response_code: answer.response_code().into(),
                    record_type: answer.record_type().to_string() ,
                    ttl: answer.ttl().into(),
                    address: "",
                    asn: &0,
                    query_time: &0
                });
            }

        }

        if records.len() == 0 {
            return None
        }
        Some(records)
    }
}
