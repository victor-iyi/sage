#[macro_use]
extern crate diesel;

pub mod models;

use diesel::prelude::*;

/// Establishes an SQLite connection.
///
/// # Example
/// ```
/// #[allow(unused_variable)]
/// let conn: SqliteConnection = establish_connection("post.db");
/// ```
pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
    .expect(&format!("Error connecting to {}", database_url))
}