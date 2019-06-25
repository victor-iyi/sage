extern crate sage;

use sage::models::{Graph, NewGraph};
use sage::utils::{establish_connection, input};

fn main() {
    let conn = establish_connection();
    let name = input("Enter graph name: ").expect("Could not read name.");
    let desc = input("Enter graph description: ").expect("Could not read description.");
    let graph = NewGraph::new(&name, &desc);

    if Graph::insert(graph, &conn) {
        println!("Successfully inserted!");
    } else {
        println!("Could not insert record.");
    }
    // List all graph data.
    let all = Graph::all(&conn);
    println!("Listing all graph entries: ({})", all.len());
    for g in &all {
        println!("({}) {} - {}", g.id, g.name, g.description);
    }
}
