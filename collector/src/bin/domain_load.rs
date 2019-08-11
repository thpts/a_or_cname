extern crate chrono;
extern crate clap;
extern crate csv;
extern crate diesel;
extern crate failure;
extern crate publicsuffix;

extern crate damp;

use clap::{App, Arg};
use damp::dns::{get_root_domain, get_sub_domain};
use damp::model::connect;
use damp::model::domain::NewDomain;
use damp::schema;
use damp::{end_processing_marker, start_processing_marker};
use diesel::prelude::*;
use failure::Error;
use publicsuffix::{Domain, List};
use std::str::FromStr;

static LOADER_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static LOADER_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
static LOADER_ABOUT: &'static str = r#"
Loader takes the domain ranking CSV, performs analysis on the
domain to determine name, TLD etc and persists in the SQLite table.

The spreadsheet must be formatted with values of ranking and domain,
containing no column header. This is a common format used by many
lists such as Alexa, Cisco Umbrella:

1,example.com
2,example.net
"#;

fn main() -> Result<(), Error> {
    let matches = App::new("domain_load")
        .version(LOADER_VERSION)
        .author(LOADER_AUTHORS)
        .about(LOADER_ABOUT)
        .arg(
            Arg::with_name("csv")
                .help("Path to CSV spreadsheet")
                .long("csv")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("sqlite-db")
                .help("Path to SQLite database")
                .long("sqlite-db")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("public-suffix-db")
                .help("Path to the Public Suffix List")
                .long("public-suffix-db")
                .required(true)
                .value_name("FILE"),
        )
        .get_matches();

    let csv_file = matches.value_of("csv").unwrap();
    let suffix_path = matches.value_of("public-suffix-db").unwrap();
    let sqlite_db = matches.value_of("sqlite-db").unwrap();

    // --------------------------
    //     Start of processing
    // --------------------------
    let start = start_processing_marker(format!(
        "Loading of domains from {} into {}",
        csv_file, sqlite_db
    ));

    let list = List::from_path(suffix_path.to_string()).unwrap();
    let conn = connect(sqlite_db.to_string());
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_file)?;

    for line in csv_reader.records() {
        // For record.get(), offset 0 is rank, 1 is domain
        let record = line?;
        let raw_domain = record.get(1);
        let rank = FromStr::from_str(record.get(0).unwrap()).unwrap_or(0);
        match list.parse_domain(raw_domain.unwrap()) {
            Ok(d) => {
                match insert_domain(&rank, &d, &conn) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error inserting domain - {}", e),
                };
            }
            Err(e) => eprintln!("Error parsing domain - {}", e),
        }
    }

    // --------------------------
    //       End of processing
    // --------------------------
    end_processing_marker("Import completed", start);

    Ok(())
}

/// Insert a domain from the spreadsheet into the SQLite database.
///
/// # Arguments
/// * `rank` - Domain ranking
/// * `domain` - Parsed Domain
/// * `conn` - SQLite connection
fn insert_domain(rank: &i32, domain: &Domain, conn: &SqliteConnection) -> QueryResult<usize> {
    let sub_domain = get_sub_domain(&domain);
    let root_domain = get_root_domain(&domain);

    let d = NewDomain {
        rank: &rank,
        fqdn: &domain.to_string(),
        sub: sub_domain.as_ref().map(|x| &**x),
        root: root_domain.as_ref().map(|x| &**x),
        suffix: domain.suffix(),
    };

    return diesel::insert_into(schema::domain::table)
        .values(&d)
        .execute(conn);
}
