use diesel::prelude::*;

use crate::schema::vertex;
use crate::schema::vertex::dsl::vertex as all_vertices;

#[derive(Queryable, Debug)]
pub struct Vertex {
  pub id: i32,
  pub label: String,
  pub schema: String,
}

#[derive(Insertable, Debug)]
#[table_name = "vertex"]
pub struct NewVertex {
  pub label: String,
  pub schema: String,
}


impl Vertex {
  pub fn all(conn: &PgConnection) -> Vec<Vertex> {
    all_vertices
      .order(vertex::id)
      .load::<Vertex>(conn)
      .expect("Could not load vertex data.")
  }

  pub fn all_with_limit(limit: i64, conn: &PgConnection) -> Vec<Vertex> {
    all_vertices
      .order(vertex::id.desc())
      .limit(limit)
      .load::<Vertex>(conn)
      .expect("Could not load vertex data.")
  }

  pub fn get_by_id(id: i32, conn: &PgConnection) -> Vec<Vertex> {
    all_vertices
      .find(id)
      .load::<Vertex>(conn)
      .expect("Could not get item in vertex.")
  }

  pub fn get_by_label(label: &String, schema: &String, conn: &PgConnection) -> Vec<Vertex> {
    all_vertices
      .filter(vertex::label.eq(label))
      .filter(vertex::schema.eq(schema))
      .get_results::<Vertex>(conn)
      .expect(&format!("Problem finding vertex label: {}", label))
  }

  pub fn insert(conn: &PgConnection, v: NewVertex) -> bool {
    // If item's label already exist. Don't insert!
    if Vertex::get_by_label(&v.label, &v.schema, conn).len() > 0 {
      return false;
    }

    diesel::insert_into(vertex::table)
      .values(v)
      .get_result::<Vertex>(conn)
      .is_ok()
  }

  pub fn update(id: i32, conn: &PgConnection, v: NewVertex) -> Option<Vertex> {
    // If id isn't in db, return None.
    if Vertex::get_by_id(id, conn).is_empty() {
      return None;
    }

    let updated_vertex: Vertex = diesel::update(all_vertices.find(id))
      .set((vertex::label.eq(v.label), vertex::schema.eq(v.schema)))
      .get_result::<Vertex>(conn)
      .expect("Failed to update record.");

    Some(updated_vertex)
  }
}