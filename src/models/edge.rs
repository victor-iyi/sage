use diesel::prelude::*;

use super::vertex::Vertex;
use crate::schema::edge;
use crate::schema::edge::dsl::edge as all_edges;

#[derive(Identifiable, Queryable, Associations, PartialEq, Clone, Debug)]
// #[belongs_to(Vertex, foreign_key = "src_vertex_id")]
#[table_name = "edge"]
pub struct Edge {
    pub id: i32,
    pub src_vertex_id: String,
    pub predicate: String,
    pub dest_vertex_id: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "edge"]
pub struct NewEdge {
    pub src_vertex_id: String,
    pub predicate: String,
    pub dest_vertex_id: String,
}

impl Edge {}
