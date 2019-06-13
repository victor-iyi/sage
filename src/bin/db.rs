extern crate sage;

use sage::connect::*;

fn main() {
  let _conn = establish_connection();
  println!("Database test.");
}