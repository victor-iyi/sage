#![allow(unused_imports, dead_code, unused_variables)]

use diesel::prelude::*;

use crate::schema::graph;
use crate::schema::graph::dsl::graph as all_graph;


#[derive(Queryable)]
pub struct Graph {
  pub id: i32,
  pub name: String,
  pub description: String,
}

#[derive(Insertable)]
#[table_name = "graph"]
pub struct NewGraph {
  pub name: String,
  pub description: String,
}

impl Graph {
  pub fn all(conn: &PgConnection) -> Vec<Graph> {
    all_graph
      .order(graph::id)
      .load::<Graph>(conn)
      .expect("Could not load graph data.")
  }

  pub fn all_with_limit(limit: i64, conn: &PgConnection) -> Vec<Graph> {
    all_graph
      .order(graph::id)
      .limit(limit)
      .load::<Graph>(conn)
      .expect("Could not load graph data.")
  }

  pub fn get_by_id(id: i32, conn: &PgConnection) -> Vec<Graph> {
    all_graph
      .find(id)
      .load::<Graph>(conn)
      .expect("Could not get item in graph.")
  }

  pub fn get_by_name(name: &String, conn: &PgConnection) -> Vec<Graph> {
    all_graph
      .filter(graph::name.eq(name))
      .get_results::<Graph>(conn)
      .expect(&format!("Problem finding graph named: {}", name))
  }

  pub fn insert(conn: &PgConnection, g: NewGraph) -> bool {
    // If item's name already exist. Don't insert!
    if Graph::get_by_name(&g.name, conn).len() > 0 {
      return false;
    }

    diesel::insert_into(graph::table)
      .values(g)
      .get_result::<Graph>(conn)
      .is_ok()
  }

  pub fn update(id: i32, conn: &PgConnection, g: NewGraph) -> Option<Graph> {
    // If id isn't in db, return None.
    if Graph::get_by_id(id, conn).is_empty() {
      return None;
    }

    let updated_graph: Graph = diesel::update(all_graph.find(id))
      .set((graph::name.eq(g.name), graph::description.eq(g.description)))
      .get_result::<Graph>(conn)
      .expect("Failed to update record.");

    Some(updated_graph)
  }
}
