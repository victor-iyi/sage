#![allow(unused_variables)]

use std::collections::HashMap;
use std::path::Path;

// use super::models::NewGraph;
use super::error::SageError;

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
  pub vertices: Vec<Vertex>
}

#[derive(Debug)]
pub struct KnowledgeGraph {
  pub name: String,
  pub description: String,
  pub graphs: Vec<Graph>,
}

// KnowledgeGraph methods.
impl KnowledgeGraph {

  pub fn add_graph(&mut self, name: &String, description: &String) -> Result<bool, SageError> {
    unimplemented!()
  }

}

// KnowledgeGraph related functions.
impl KnowledgeGraph {

  pub fn from_jsonld(path: impl AsRef<Path>) -> Result<KnowledgeGraph, SageError> {
    unimplemented!()
  }

  pub fn from_triples(path: impl AsRef<Path>) -> Result<KnowledgeGraph, SageError> {
    unimplemented!()
  }

}

// Graph methods.
impl Graph {
  pub fn add_vertex(&mut self, label: &String, schema: &String) -> Result<bool, SageError> {
    unimplemented!()
  }

  pub fn add_payload(&mut self, key: &String, value: &String) -> Result<bool, SageError> {
    unimplemented!()
  }

  pub fn add_triples(&mut self, subject: &String, predicate: &String, object: &String) -> Result<bool, SageError> {
    unimplemented!()
  }
}


// Graph related functions.
impl Graph {

  pub fn from(name: &String, description: &String) -> Graph {
    unimplemented!()
  }

}