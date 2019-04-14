use super::schema::domain;

use diesel::prelude::*;

/// Open the SQLite database for use
///
/// Arguments:
/// * `database_url` - Path to the SQLite file
pub fn connect(database_url: String) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct Domain {
    pub rank: i32,
    pub fqdn: String,
    pub sub: String,
    pub root: String,
    pub suffix: String,
}

#[derive(Insertable)]
#[table_name = "domain"]
pub struct NewDomain<'a> {
    pub rank: &'a i32,
    pub fqdn: &'a str,
    pub sub: &'a str,
    pub root: &'a str,
    pub suffix: &'a str,
}
