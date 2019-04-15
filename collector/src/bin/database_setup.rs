extern crate clap;
#[macro_use]
extern crate diesel_migrations;

extern crate damp;

use clap::{App, Arg};
use damp::model::connect;
use diesel_migrations::embed_migrations;
use std::error::Error;

static QUERY_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static QUERY_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
static QUERY_ABOUT: &'static str = r#"
Generates a correct version of the database with the necessary schema.
"#;

embed_migrations!("./migrations");

fn main() -> Result<(), Box<Error>> {
    let matches = App::new("database_setup")
        .version(QUERY_VERSION)
        .author(QUERY_AUTHORS)
        .about(QUERY_ABOUT)
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("sqlite-db")
                .help("Path to SQLite database")
                .long("sqlite-db")
                .required(true)
                .value_name("FILE"),
        )
        .get_matches();
    let sqlite_db = matches.value_of("sqlite-db").unwrap();
    let conn = connect(sqlite_db.to_string());

    return match embedded_migrations::run_with_output(&conn, &mut std::io::stdout()) {
        Err(e) => Err(Box::new(e)),
        Ok(_) => Ok(()),
    };
}
