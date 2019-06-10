#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
    .expect(&format!("Error connecting to {}", database_url))
}