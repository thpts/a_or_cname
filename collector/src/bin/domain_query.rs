extern crate clap;

extern crate damp;

use clap::{App, Arg};
use damp::dns::query::Query;
use damp::{end_processing_marker, start_processing_marker};

use std::error::Error;
use std::net::SocketAddr;

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

fn main() -> Result<(), Box<Error>> {
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
    let asn_db = matches.value_of("asn-db").unwrap().to_string();

    let client = Query::new(resolver, &asn_db);

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
