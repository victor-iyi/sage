use diesel::associations::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::vertex;
use crate::schema::vertex::dsl::vertex as all_vertices;

use super::graph::Graph;


#[derive(Insertable, Clone, Debug)]
#[table_name = "vertex"]
pub struct NewVertex {
    pub id: String,
    pub label: String,
    pub schema: String,
    pub graph_id: String,
}

impl NewVertex {

    pub fn new(label: &str, schema: &str, graph_id: &str) -> NewVertex {
        let id: Uuid = Uuid::new_v4();
        NewVertex {
            id: id.to_simple().to_string(),
            label: label.to_owned(),
            schema: schema.to_owned(),
            graph_id: graph_id.to_owned(),
        }
    }

}


#[derive(Identifiable, Queryable, PartialEq, Clone, Debug)]
// #[belongs_to(Graph, foreign_key = "graph_id")]
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

    pub fn get_by_id(id: &str, conn: &PgConnection) -> Option<Vertex> {
        let result = all_vertices.find(id).first::<Vertex>(conn);
        match result {
            Ok(v) => Some(v),
            Err(_) => None, // Could not find item.
        }
    }

    pub fn get_by_label(label: &str, schema: &str, conn: &PgConnection) -> Option<Vertex> {
        let result = all_vertices
            .filter(vertex::label.eq(label))
            .filter(vertex::schema.eq(schema))
            .first::<Vertex>(conn);

        match result {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
