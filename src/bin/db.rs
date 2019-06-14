extern crate sage;

use sage::models::graph::{Graph, NewGraph};
use sage::utils::{input, establish_connection};

fn main() {
  let conn = establish_connection();

  let g = NewGraph {
    name: input("Enter name of graph: ").expect("Could not read graph name."),
    description: input("Enter description of graph: ").expect("Could not read description."),
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
