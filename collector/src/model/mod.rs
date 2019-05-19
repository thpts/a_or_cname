pub mod domain;
pub mod record;

use diesel::prelude::*;

/// Open the SQLite database for use
///
/// # Arguments:
/// * `database_url` - Path to the SQLite file
///
/// # Example
/// ```
/// use model::connect;
///
/// let conn = connect(":memory:");
/// ```
pub fn connect(database_url: String) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
