#![allow(unused_variables, unused_imports)]

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use serde_json::Value;

use crate::error::SageError;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
  pub vertex: Vertex, // TODO: Might not be useful.
  pub predicate: String,
  pub dest: Vertex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
  pub label: String,
  pub schema: String,
  pub payload: HashMap<String, String>,
  pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Graph {
  pub name: String,
  pub description: String,
  pub vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct KnowledgeGraph {
  pub name: String,
  pub description: String,
}

#[derive(Debug)]
pub struct MultiKnowledgeGraph {
  pub name: String,
  pub description: String,
  pub graphs: Vec<Graph>,
}

// MultiKnowledgeGraph methods.
impl MultiKnowledgeGraph {

  pub fn add_graph(&mut self, name: &String, description: &String) -> Result<bool, SageError> {
    unimplemented!()
  }

}

// MultiKnowledgeGraph related functions.
impl MultiKnowledgeGraph {

  pub fn from_jsonld(path: impl AsRef<Path>) -> Result<MultiKnowledgeGraph, SageError> {
    unimplemented!()
  }

  pub fn from_triples(path: impl AsRef<Path>) -> Result<MultiKnowledgeGraph, SageError> {
    unimplemented!()
  }

}

// Graph methods.
impl Graph {

  pub fn add_vertex(&mut self, label: &str, schema: &str) -> Result<bool, SageError> {
    unimplemented!()
  }

  pub fn add_payload(&mut self, key: &str, value: &str) -> Result<bool, SageError> {
    unimplemented!()
  }

  pub fn load(&mut self, data: Value) {
    unimplemented!()
  }

}


// Graph related functions.
impl Graph {

  pub fn new(name: &str, description: &str) -> Graph {
    unimplemented!()
  }

  pub fn with_data(name: &str, description: &str, data: Value) -> Graph {
    unimplemented!()
  }

  pub fn from(description: &str, data_file: impl AsRef<Path>) -> Graph {
    unimplemented!()
  }

}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | |                          Display Traits                          + |
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */

impl fmt::Display for KnowledgeGraph {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "KnowledgeGraph<{}>", self.name)
  }
}

impl fmt::Display for MultiKnowledgeGraph {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MultiKnowledgeGraph<{}>", self.name)
  }
}

impl fmt::Display for Graph {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Graph<{}>", self.name)
  }
}

impl fmt::Display for Vertex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vertex<{}, {}>", self.label, self.schema)
  }
}

impl fmt::Display for Edge {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Edge<{}, {}>", self.vertex, self.dest)
  }
}
