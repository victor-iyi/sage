use diesel::prelude::*;

use crate::schema::edge;
use crate::schema::edge::dsl::edge as all_edges;


#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
// #[belongs_to(vertex::Vertex, foreign_key = "src_vertex_id")]
#[table_name = "edge"]
pub struct Edge {
  pub id: i32,
  pub src_vertex_id: String,
  pub predicate: String,
  pub dest_vertex_id: String,
}


impl Edge {

}