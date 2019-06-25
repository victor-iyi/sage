#![allow(unused_variables, unused_imports)]

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use serde_json::Value;

use crate::error;

use super::schema::{Edge, Graph, Vertex};


#[derive(Debug)]
pub struct KnowledgeGraph {
  pub name: String,
  pub description: String,
}

impl KnowledgeGraph {
  pub fn new(name: &str, description: &str) -> KnowledgeGraph {
    KnowledgeGraph {
      name: name.to_owned(),
      description: description.to_owned(),
    }
  }
}

#[derive(Debug)]
pub struct MultiKnowledgeGraph {
  pub name: String,
  pub description: String,
  pub graphs: Vec<Graph>,
}


// MultiKnowledgeGraph methods.
impl MultiKnowledgeGraph {
  pub fn add_graph(&mut self, name: &str, description: &str) -> error::Result<bool> {
    unimplemented!()
  }

}

// MultiKnowledgeGraph related functions.
impl MultiKnowledgeGraph {

  pub fn new(name: &str, description: &str) -> MultiKnowledgeGraph {
    MultiKnowledgeGraph {
      name: name.to_owned(),
      description: description.to_owned(),
      graphs: vec![],
    }
  }

  pub fn from_jsonld(path: impl AsRef<Path>) -> error::Result<MultiKnowledgeGraph> {
    unimplemented!()
  }

  pub fn from_triples(path: impl AsRef<Path>) -> error::Result<MultiKnowledgeGraph> {
    unimplemented!()
  }

}

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

