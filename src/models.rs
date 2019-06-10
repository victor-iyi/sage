use std::collections::HashMap;


/// Vertex (or Node) - representing each Entity in Graph.
///
#[derive(Queryable)]
pub struct Vertex {
  /// Unique 8-bit token assigned to each Vertex.
  pub id: String,

  /// Vertex label.
  pub label: String,

  /// Vertex schema.
  pub schema: String,

  /// Payload which current vertex carries. Contains information about Vertex.
  pub payload: HashMap<String, String>,

  /// Connection of Vertex to other Vertex in the Graph.
  pub edges: Vec<Edge>,
}


/// Edge which is describes the connection between one Vertex & it's neighbors.
///
#[derive(Queryable)]
pub struct Edge {

  /// Table's primary key.
  pub id: u32,

  /// Vertex which the edge is connected to.
  pub vertex_id: String,

  /// Describing the connection `vertex` has with `vertex_id`.
  pub predicate: String,

  /// Source Vertex (`vertex`) is connected to `vertex_id`.
  pub vertex: Vertex,
}


/// Graph database Schema.
///
#[derive(Queryable)]
pub struct Graph {
  /// Unique integer primary key.
  pub id: u32,

  /// Graph name - DB Storage name.
  pub name: String,

  /// Vertex Id foreign key.
  pub vertex_id: String,

  /// List of all Vertex objects in Graph.
  pub vertices: Vec<Vertex>,
}