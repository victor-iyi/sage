#![allow(unused_imports, unused_variables)]

use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::graph;
use crate::schema::graph::dsl::graph as all_graph;


#[derive(Identifiable, Queryable, PartialEq, Clone, Debug)]
#[table_name = "graph"]
pub struct Graph {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "graph"]
pub struct NewGraph {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl NewGraph {
    pub fn new(name: &str, description: &str) -> NewGraph {
        let id = Uuid::new_v4();
        NewGraph {
            id: id.to_simple().to_string(),
            name: name.to_owned(),
            description: description.to_owned(),
        }
    }
}

impl Graph {

    pub fn all(conn: &PgConnection) -> Vec<Graph> {
        graph::table
            .load::<Graph>(conn)
            .expect("Error loading graphs.")
    }

    pub fn get_by_id(id: &str, conn: &PgConnection) -> Option<Graph> {
        let result = all_graph.find(id).first::<Graph>(conn);

        match result {
            Ok(g) => Some(g),
            Err(_) => None, // Could not find item in graph.
        }
    }

    pub fn get_by_name(name: &str, conn: &PgConnection) -> Option<Graph> {
        let result = all_graph.filter(graph::name.eq(name)).first::<Graph>(conn);
        match result {
            Ok(g) => Some(g),
            Err(_) => None,
        }
    }

    pub fn insert(g: NewGraph, conn: &PgConnection) -> bool {
        match Graph::get_by_name(&g.name, conn) {
            Some(_) => false, // Graph name already taken.
            None => diesel::insert_into(graph::table)
                .values(&g)
                .execute(conn)
                .is_ok(),
        }
    }

    pub fn create_and_insert(name: &str, description: &str, conn: &PgConnection) -> Option<Graph> {
        match Graph::get_by_name(name, conn) {
            Some(g) => Some(g), // Return graph (already exists).
            None => {
                // Create a new graph to insert.
                let NewGraph {
                    id,
                    name,
                    description,
                } = NewGraph::new(name, description);

                // Build insert values.
                let values = (
                    graph::id.eq(id),
                    graph::name.eq(name),
                    graph::description.eq(description),
                );

                // Insert into db.
                match diesel::insert_into(graph::table)
                    .values(&values)
                    .get_result::<Graph>(conn)
                {
                    Ok(r) => Some(r), // Return inserted graph.
                    Err(_) => None,   // Could not insert into graph.
                }
            }
        }
    }

    // pub fn get_vertices(name: &str, conn: &PgConnection) -> Vec<Graph> {
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
