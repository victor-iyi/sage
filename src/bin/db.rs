
extern crate diesel;
extern crate dotenv;
extern crate sage;

use diesel::prelude::*;
use dotenv::dotenv;
use sage::models::graph::{Graph, NewGraph};

use std::env;

fn main() {
  dotenv().ok();

  let database_url: String = env::var("DATABASE_URL").expect("Make sure DATABASE_URL is set");
  let conn =
    PgConnection::establish(&database_url).expect("Could not establish database connection.");

  let g = NewGraph {
    name: String::from("New York"),
    description: String::from("Information about the state of New York."),
  };

  if Graph::insert(&conn, g) {
    println!("Successfully inserted!");
  } else {
    println!("Insert not successful!");
  }

  let results = Graph::all(&conn);
  for result in results.iter() {
    println!("{} - {}", result.name, result.description);
  }
}
