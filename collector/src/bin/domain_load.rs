extern crate chrono;
extern crate clap;
extern crate csv;
extern crate diesel;
extern crate publicsuffix;

extern crate damp;

use chrono::prelude::Utc;
use clap::{App, Arg};
use damp::dns::get_sub_domain;
use damp::model::connect;
use damp::model::domain::NewDomain;
use damp::schema;
use diesel::prelude::*;
use publicsuffix::{Domain, List};
use std::error::Error;
use std::str::FromStr;
use std::time::Instant;

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

fn main() -> Result<(), Box<Error>> {
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
    let start = Instant::now();
    println!(
        "Import of domains from {} started at {}",
        csv_file,
        Utc::now().to_rfc3339()
    );

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
                    Ok(_)  => (),
                    Err(e) => eprintln!("Error inserting domain - {}", e)
                };
            }
            Err(e) => eprintln!("Error parsing domain - {}", e),
        }
    }

    // --------------------------
    //       End of processing
    // --------------------------
    let duration = start.elapsed();
    println!(
        "Import completed at {} taking {} seconds",
        Utc::now().to_rfc3339(),
        duration.as_secs()
    );

    Ok(())
}

/// Insert a domain from the spreadsheet into the SQLite database.
///
/// # Arguments
/// * `rank` - Domain ranking
/// * `domain` - Parsed Domain
fn insert_domain(rank: &i32, domain: &Domain, conn: &SqliteConnection) -> QueryResult<usize> {
    let sub_domain = get_sub_domain(&domain);

    let d = NewDomain {
        rank: &rank,
        fqdn: &domain.to_string(),
        sub: &sub_domain.unwrap_or("".to_string()),
        root: &domain.root().unwrap_or(""),
        suffix: &domain.suffix().unwrap_or(""),
    };

    return diesel::insert_into(schema::domain::table)
        .values(&d)
        .execute(conn);
}
