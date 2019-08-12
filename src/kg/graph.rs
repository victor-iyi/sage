#![allow(unused_variables)]

// Standard library.
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

// External crates.
use serde_json::Value;

// Custom crate.
use crate::{error, SageResult};
use super::schema::{Edge, Graph, Vertex};


/// KnowledgeGraph is the simplest version of sage Graph database,
/// and it's useful when creating simple relationships.
///
/// It doesn't manage multiple graphs and connects one entity in a
/// graph with entities in another graph. See `MultiKnowledgeGraph`
/// if this is the case.
#[derive(Debug)]
pub struct KnowledgeGraph {
  pub name: String,
  pub description: String,
}

//  KnowledgeGraph related functions.
impl KnowledgeGraph {

  /// Create a new knowledge graph.
  ///
  /// # Example
  /// ```rust
  /// use sage::kg::KnowledgeGraph;
  ///
  /// let name = "Hollywood";
  /// let description = "Contains information about hollywood movie industry";
  /// // Create a singly KnowledgeGraph.
  /// let graph = KnowledgeGraph::new(name, description);
  /// assert_eq!(graph.name, name.to_string());
  /// assert_eq!(graph.description, description.to_string());
  /// ```
  pub fn new(name: &str, description: &str) -> KnowledgeGraph {
    KnowledgeGraph {
      name: name.to_string(),
      description: description.to_string(),
    }
  }

}

/// MultiKnowledgeGraph manages multiple graph and relates entities in
/// one graph with entities in another graph.  It's a more complex
/// version of the singly `KnowledgeGraph`.
#[derive(Debug)]
pub struct MultiKnowledgeGraph {
  pub name: String,
  pub description: String,
  pub graphs: Vec<Graph>,
}


// MultiKnowledgeGraph methods.
impl MultiKnowledgeGraph {

  /// Adds a new graph to the Knowledge Graph.
  pub fn add_graph(&mut self, name: &str, description: &str) -> SageResult<bool> {
    unimplemented!()
  }

}

// MultiKnowledgeGraph related functions.
impl MultiKnowledgeGraph {

  /// Create a new multi knowledge graph.
  ///
  /// # Example
  /// ```rust
  /// use sage::kg::MultiKnowledgeGraph;
  ///
  /// let name = "Hollywood";
  /// let description = "Contains information about the Hollywood movie industry.";
  ///
  /// // Create a new multiple knowledge graph.
  /// let graph = MultiKnowledgeGraph::new(name, description);
  /// assert_eq!(format!("MultiKnowledgeGraph<{}>", name), format!("{}", graph));
  /// ```
  pub fn new(name: &str, description: &str) -> MultiKnowledgeGraph {
    MultiKnowledgeGraph {
      name: name.to_string(),
      description: description.to_string(),
      graphs: vec![],
    }
  }

  /// Load knowledge graph from a JSONLD (.jsonld or .json) file.
  pub fn from_jsonld(path: impl AsRef<Path>) -> SageResult<MultiKnowledgeGraph> {
    unimplemented!()
  }

  /// Load the knowledge graph from an n-triple (.nt) file.
  pub fn from_triples(path: impl AsRef<Path>) -> SageResult<MultiKnowledgeGraph> {
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

