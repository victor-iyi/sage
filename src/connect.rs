use std::env;

use dotenv;
use diesel::prelude::*;

/// Establishes an postgres connection.
///
/// # Example
/// ```
/// #[allow(unused_variable)]
/// let conn: PgConnection = establish_connection();
/// ```
pub fn establish_connection() -> PgConnection {
  dotenv.ok();

  let database_url: String = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set.");

  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}