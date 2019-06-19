#![allow(unused_variables, unused_imports)]

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::Value;

use crate::error::SageError;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {
  pub src: Vertex, // TODO: Might not be useful.
  pub predicate: String,
  pub dest: Vertex,
}

impl Edge {

  pub fn new(src: Vertex, predicate: &str, dest: Vertex) -> Edge {
    Edge {
      src,
      predicate: predicate.to_owned(),
      dest,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vertex {
  pub label: String,
  pub schema: String,
  pub payload: HashMap<String, String>,
  pub edges: Vec<Edge>,
}

impl Vertex {
  pub fn new(label: &str, schema: &str) -> Vertex {
    Vertex {
      label: label.to_owned(),
      schema: schema.to_owned(),
      payload: HashMap::new(),
      edges: vec![],
    }
  }
}

impl Vertex {

  pub fn add_payload(&mut self, key: &str, value: &str) {
    self
      .payload
      .insert(key.to_owned(), value.to_owned())
      .expect("Could insert payload.");
  }

  pub fn add_edges(&mut self, predicate: &str, dest: &Vertex)  {
    // Create a new edge.
    let edge = Edge::new(self.clone(), predicate, dest.clone());

    // If edge does not exist, add it.
    if !self.edges.contains(&edge) {
      self.edges.push(edge);
    }
  }
}

#[derive(Debug)]
pub struct Graph {
  pub name: String,
  pub description: String,
  pub vertices: Vec<Vertex>,
}


// Graph related functions.
impl Graph {

  pub fn new(name: &str, description: &str) -> Graph {
    Graph {
      name: name.to_owned(),
      description: description.to_owned(),
      vertices: vec![],
    }
  }

  pub fn from_file(description: &str, data_file: impl AsRef<Path>) -> Result<Graph, SageError> {
    // name = filename of data_file.
    let splits: Vec<&str> = data_file.as_ref().to_str().unwrap().split('.').collect();
    let name = *(splits.get(splits.len() - 2).unwrap());

    // Open file in read-only mode with buffer.
    let file = File::open(&data_file).expect("Could not open file.");
    let reader = BufReader::new(file);

    // vertices = Loaded from data_file.
    // Read the JSON contents as an instance of `serde_json::Value`.
    let data: Value = serde_json::from_reader(reader).expect("Could not parse JSON data.");

    // Create new graph with loaded data.
    Graph::from_data(name, description, data)
  }

  pub fn from_data(name: &str, description: &str, data: Value) -> Result<Graph, SageError> {
    unimplemented!()
  }

}

// Graph methods.
impl Graph {

  pub fn add_vertex(&mut self, label: &str, schema: &str) {
    let vertex = Vertex::new(label, schema);
    self.vertices.push(vertex);
  }

  pub fn load(&mut self, data: Value) {
    unimplemented!()
  }

  pub fn load_file(&mut self, path: impl AsRef<Path>) {
    // Open file in read-only mode with buffer.
    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);

    // Read the JSON contents as an instance of `serde_json::Value`.
    let data: Value = serde_json::from_reader(reader).expect("Could not parse JSON data.");

    // Load with data.
    self.load(data)
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
    write!(f, "Edge<{}, {}>", self.src, self.dest)
  }
}