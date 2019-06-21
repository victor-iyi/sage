//! `sage.utils` contains useful helper functions either during using
//! `sage` or by your program (which may or may not have anything to
//! do with `sage`).
use diesel::prelude::*;
use dotenv::dotenv;

use std::env;
use std::io;

/// Getting input from stdin. Masking Python's `input` function.
///
/// ## Basic Usage.
/// Getting `String` input from console.
/// ```
/// let name: String = input("Enter your name:")
///                     .expect("Failed to get name.");
/// println!("name = {}", name);
/// ```
///
/// Getting other types aside `String`.
/// ```
/// let age: u8 = input("Enter your age: ")
///                .expect("Failed to get age.")
///                .parse::<u8>().expect("Invalid age.");
/// ```
pub fn input(msg: &str) -> io::Result<String> {
    use std::io::Write;
    // Print prompt to the console.
    print!("{}", msg);
    io::stdout().flush()?;

    // Accept input.
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

/// Connect to a postgreSQL database.
///
/// **NOTE:** Make sure `DATABASE_URL` is set as an
/// environment variable.
///
/// ## Basic Usage
/// ```rust
/// // Establish database connection.
/// let conn = establish_connection();
/// // Use database connection to query database.
/// let all_graphs = sage::models::Graph::all(&conn);
/// for graph in all_graphs {
///   println!("{} - {}", graph.name, graph.description);
/// }
/// ```
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("Make sure DATABASE_URL is set.");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Could not establish connection to {}.", database_url))
}
