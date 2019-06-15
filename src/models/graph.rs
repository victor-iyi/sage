use diesel::prelude::*;

use crate::schema::graph;
use crate::schema::graph::dsl::graph as all_graph;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "graph"]
pub struct Graph {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Graph {
    pub fn all(conn: &PgConnection) -> Vec<Graph> {
        graph::table
            .load::<Graph>(conn)
            .expect("Error loading graphs.")
    }

    pub fn get_by_id(id: &String, conn: &PgConnection) -> Option<Graph> {
        Some(
            all_graph
                .find(id)
                .first::<Graph>(conn)
                .expect(&format!("Error loading graph with id: {}", id)),
        )
    }

    pub fn get_by_name(name: &String, conn: &PgConnection) -> Option<Graph> {
        Some(
            all_graph
                .filter(graph::name.eq(name))
                .first::<Graph>(conn)
                .expect(&format!("Could not find graph with name: {}", name)),
        )
    }

    // pub fn get_vertices(name: &String, conn: &PgConnection) -> Vec<Graph> {
    //   use super::vertex;
    //   if let Some(g) = Graph::get_by_name(name, conn) {
    //     vertex::Vertex::belonging_to(&g)
    //       .load::<Graph>(conn)
    //       .expect("Could not find graph vertices")
    //   } else {
    //     vec![]
    //   }
    // }
}
