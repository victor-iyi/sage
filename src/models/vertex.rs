use diesel::associations::*;
use diesel::prelude::*;

use crate::schema::vertex;
use crate::schema::vertex::dsl::vertex as all_vertices;

// use super::graph::Graph;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
// #[belongs_to(Graph)]
#[table_name = "vertex"]
pub struct Vertex {
  pub id: String,
  pub label: String,
  pub schema: String,
  pub graph_id: String,
}

impl Vertex {

  pub fn all(conn: &PgConnection) -> Vec<Vertex> {
    vertex::table
      .load::<Vertex>(conn)
      .expect("Error loading vertices.")
  }

  pub fn get_by_id(id: &String, conn: &PgConnection) -> Option<Vertex> {
    Some(
      all_vertices
        .find(id)
        .first::<Vertex>(conn)
        .expect(&format!("Error loading vertex with id: {}", id)),
    )
  }

  pub fn get_by_label(label: &String, schema: &String, conn: &PgConnection) -> Option<Vertex> {
    Some(
      all_vertices
        .filter(vertex::label.eq(label))
        .filter(vertex::schema.eq(schema))
        .first::<Vertex>(conn)
        .expect(&format!("Could not find vertex with name: {}", label)),
    )
  }
}