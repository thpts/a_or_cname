extern crate chrono;
extern crate clap;
extern crate failure;
#[macro_use]
extern crate diesel_migrations;

extern crate damp;

use clap::{App, Arg};
use damp::model::connect;
use damp::{end_processing_marker, start_processing_marker};
use diesel_migrations::embed_migrations;
use diesel_migrations::RunMigrationsError;

static QUERY_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static QUERY_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
static QUERY_ABOUT: &'static str = r#"
Generates a correct version of the database with the necessary schema.
"#;

embed_migrations!("./migrations");

fn main() -> Result<(), RunMigrationsError> {
    let matches = App::new("database_setup")
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
        .get_matches();

    let sqlite_db = matches.value_of("sqlite-db").unwrap();
    let start =
        start_processing_marker(format!("Loading of schema into {}", sqlite_db.to_string()));
    let conn = connect(sqlite_db.to_string());

    return match embedded_migrations::run_with_output(&conn, &mut std::io::stdout()) {
        Err(e) => Err(e),
        Ok(_) => {
            end_processing_marker("Migrations completed", start);
            Ok(())
        }
    };
}
