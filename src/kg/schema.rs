#![allow(unused_variables, unused_imports)]

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::Value;

use crate::SageResult;

/// Edge describes the connection between a vertex and it's neighbors.
///
/// ```rust
/// james = Vertex::new("James Cameron", "Person");
/// avatar = Vertex::new("Avatar", "Movie");
/// // James ---director--> Avatar
/// let edge = Edge::new(&james, "director", &avatar);
///```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {

  // TODO: Might wanna use a Box<Vertex> to reduce size.
  /// Source Vertex has a directed connection to `dest` vertex.
  pub src: Vertex,

  /// Describes the connection `src` vertex has with `dest` vertex.
  pub predicate: String,

  // TODO: Might wanna use a Box<Vertex> to reduce size.
  /// Vertex which edge is connected to.
  pub dest: Vertex,
}

impl Edge {

  /// Creates a new edge connection between 2 vertices.
  ///
  /// ## Example
  /// ```rust
  /// let james = Vertex::new("James Cameron", "Person");
  /// let avatar = Vertex::new("Avatar", "Movie");
  /// // James --director--> Avatar
  /// let edge = Edge::new(&james, "director", &avatar);
  /// ```
  pub fn new(src: &Vertex, predicate: &str, dest: &Vertex) -> Edge {
    Edge {
      src: src.clone(),
      predicate: predicate.to_string(),
      dest: dest.clone(),
    }
  }
}

/// Vertex (or Node) is a representation of each entity in the Graph.
///
/// ## Example
/// Creates new entity called `James Cameron` of type <https://schema.org/Person>.
/// ```rust
/// let james = Vertex::new("James Cameron", "Person");
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vertex {
  pub label: String,
  pub schema: String,
  pub payload: HashMap<String, String>,
  pub edges: Vec<Edge>,
}

impl Vertex {

  /// Creates a new Vertex object from label and schema.
  ///
  ///  Vertices are created with empty payload and edges.
  ///
  /// ## Example
  /// Creates a new entity called `James Cameron` of type <https://schema.org/Person>
  /// ```rust
  /// let james = Vertex::new("James Cameron", "Person");
  /// ```
  ///
  /// Creates a new entity called `Avatar` of type <https://schema.org/Movie>
  /// ```rust
  /// let avatar = Vertex::new("Avatar", "Movie");
  /// ```
  pub fn new(label: &str, schema: &str) -> Vertex {
    Vertex {
      label: label.to_string(),
      schema: schema.to_string(),
      payload: HashMap::new(),
      edges: vec![],
    }
  }

}

impl Vertex {

  /// Adds a new payload to the vertex.
  ///
  /// If `key` already exist, it is overridden by the new `value`.
  ///
  /// ## Example
  /// ```rust
  /// // Creates a new vertex.
  /// let avatar = Vertex::new("Avatar", "Movie");
  /// // Adds `genre` to the vertex's payload.
  /// avatar.add_payload("genre", "Science Fiction");
  /// // Adds `trailer` to the vertex's payload.
  /// avatar.add_payload("trailer", "https://www.youtube.com/watch?v=6ziBFh3V1aM");
  /// ```
  pub fn add_payload(&mut self, key: &str, value: &str) {
    self
      .payload
      .insert(key.to_string(), value.to_string())
      .expect("Could insert payload.");
  }

  /// Adds new connection to current vertex object.
  ///
  /// ## Example
  /// `James Cameron` directed the movie `Avatar`, can be represented as:
  /// ```rust
  /// let mut james = Vertex::new("James Cameron", "Person");
  /// let avatar = Vertex::new("Avatar", "Movie");
  /// // Connects `avatar` to `james` as "director".
  /// james.add_edge("director", &avatar);
  /// ```
  pub fn add_edge(&mut self, predicate: &str, dest: &Vertex) {
    // Create a new edge.
    let edge = Edge::new(&self, predicate, &dest);

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


/// Graph related functions.
impl Graph {

  /// Creates an empty Graph with a name and description.
  ///
  /// ## Basic Usage
  /// ```rust
  /// // Creates an empty graph.
  /// let g = Graph::new("Hollywood", "Stores everything related to Hollywood.");
  /// ```
  pub fn new(name: &str, description: &str) -> Graph {
    Graph {
      name: name.to_string(),
      description: description.to_string(),
      vertices: vec![],
    }
  }

  /// Creates a new graph, populating it with loaded data from `data_file`.
  /// name is inferred from the filename of `data_file`.
  ///
  /// **Note:** `data_file` must be of supported formats.
  ///   See `sage::kg::SUPPORTED_FORMATS`.
  ///
  /// **P.S:** `name` can be updated by resetting the `name` property.
  ///
  /// ## Example
  /// ```rust
  /// // Create a new mutable graph.
  /// let mut g = Graph::from_file("Stores everything related to Hollywood",
  ///                              "resources/schema-org/movie.jsonld");
  /// // Override the inferred graph name.
  /// g.name = "Hollywood";
  /// ```
  pub fn from_file(description: &str, data_file: impl AsRef<Path>) -> SageResult<Graph> {
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

  /// Creates a new graph, and loading it with initial data.
  ///
  /// ## Example
  /// ```rust
  ///
  /// ```
  pub fn from_data(name: &str, description: &str, data: Value) -> SageResult<Graph> {
    unimplemented!()
  }

}

// Graph methods.
impl Graph {

  /// Creates and pushes a new vertex unto the Graph.
  ///
  /// ## Example
  ///
  /// ```rust
  /// // Create an empty Graph.
  /// let g = Graph::new("Hollywood", "Stores everything related to Hollywood.");
  /// // Create and add the 1st vertex.
  /// g.add_vertex("James Cameron", "Person");
  /// // Create and add the 2nd vertex.
  /// g.add_vertex("Johnny Depp", "Person");
  /// // Assert the vertices where pushed successfully.
  /// assert_eq!(g.vertices.len(), 2);
  /// ```
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

mod tests {

  use super::*;

  #[test]
  fn create_vertex() {
    let schema = String::from("Person");
    let label = String::from("James Cameron");
    let vertex: Vertex = Vertex::new(&label, &schema);

    assert_eq!(vertex.label, label);
    assert_eq!(vertex.schema, schema);
  }
}