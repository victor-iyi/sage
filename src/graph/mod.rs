//! Sage KnowledgeGraph interface _(front-end)_.
//!
//! Author: Victor I. Afolabi
//! Email: javafolabi@gmail.com
//! License: MIT

// Standard libraries.
use std::collections::HashMap;
use std::fmt;

/// Vertex (or Node) - representing each Entity in Graph.
///
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Edge {
  /// Vertex which the edge is connected to.
  pub vertex_id: String,

  /// Describing the connection `vertex` has with `vertex_id`.
  pub predicate: String,

  /// Source Vertex (`vertex`) is connected to `vertex_id`.
  pub vertex: Vertex,
}


/// Graph database Schema.
///
#[derive(Debug)]
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

/// KnowledgeGraph is the interface for interacting with sage.
#[derive(Debug)]
pub struct KnowledgeGraph {
  pub name: String,
  pub vertices: Vec<Vertex>,
}

/// KnowledgeGraph related functions.
impl KnowledgeGraph {}

/// Display trait implementation for KnowledgeGraph
impl fmt::Display for KnowledgeGraph {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "KnowledgeGraph<{}>", self.name)
  }
}

/// Display trait implementation for Vertex
impl fmt::Display for Vertex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vertex<{}, {}>", self.label, self.schema)
  }
}

/// Display trait implementation for Edge
impl fmt::Display for Edge {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Edge<{}, {}>", self.vertex_id, self.predicate)
  }
}