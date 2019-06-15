extern crate sage;

use sage::models::{Graph, NewGraph};
use sage::utils::{establish_connection, input};

fn main() {
    let conn = establish_connection();
    let graph = NewGraph {
        name: input("Enter graph name: "),
        description: input("Enter graph description: "),
    };
    if Graph::insert(graph, &conn) {
        println!("Successfully inserted!");
    } else {
        println!("Could not insert record.");
    }
}
