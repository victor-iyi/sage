/// Vertex (or Node) - representing each Entity in Graph.
///
#[derive(Queryable)]
pub struct Vertex {
  /// Unique 8-bit token assigned to each Vertex.
  pub id: diesel::types::Uuid,

  /// Label given to Vertex (or Node).
  pub label: diesel::types::VarChar,

  /// Vertex schema. Using https://schema.org standards.
  /// Other forms of FOAF oncology will be supported in
  /// future.
  pub schema: diesel::types::VarChar,

  /// Payload which current vertex carries.
  /// Contains information about Vertex.
  pub payload: diesel::types::Array<Payload>,

  /// Connection of Vertex to other Vertex in the Graph.
  pub edges: diesel::types::Array<Edge>,
}


/// Payload containing detailed information about Vertex.
///
/// **Note:** The payload doesn't hold a value of another Vertex
/// object, otherwise, it contains data modeled by a certain
/// Vertex.
///
/// E.g: "name": "John Doe", "age": 22
///
#[derive(Queryable)]
pub struct Payload {
  /// Table's primary key.
  pub id: diesel::types::Integer,

  /// Key of payload. E.g "name", "age", etc...
  pub key: diesel::types::VarChar,

  /// Value of payload. E.g. "John", 23, etc...
  pub value: diesel::types::Text,

  /// Foreign key of Vertex which has this payload.
  pub vertex_id: diesel::types::Uuid,
}


/// Edge which is describes the connection between
/// one Vertex & it's neighbors.
///
#[derive(Queryable)]
pub struct Edge {

  /// Table's primary key.
  pub id: diesel::types::Uuid,

  /// Destination vertex Identifier is the vertex identifier
  ///  which the source vertex identifier is connected to.
  pub dest_vertex_id: diesel::types::Uuid,

  /// Destination vertex is the vertex which the source
  /// vertex is connected to.
  pub dest_vertex: Vertex,

  /// Describing the connection `src_vertex_id` has with
  /// `dest_vertex_id`.
  pub predicate: diesel::types::Text,

  /// Source vertex identifier is the identifier of current
  ///  vertex which is connected to `dest_vertex_id`.
  pub src_vertex_id: diesel::types::Uuid,

  /// Source vertex is the current vertex  which is connected
  /// to `dest_vertex_id`.
  pub src_vertex: Vertex,
}


/// A collection of related information on a particular
/// piece of linked data.
///
/// **Note:** Sage KnowledgeGraph can contain multiple Graph.
#[derive(Queryable)]
pub struct Graph {
  /// We can have multiple graphs in a Knowledge Graph.
  pub id: diesel::types::Uuid,

  /// A descriptive name given to the Graph. Name tells us
  /// which type of information is contained in the Graph.
  /// Example: "Medical Symptoms"
  pub name: diesel::types::VarChar,

  /// A detailed description of the Graph content.
  pub description: diesel::types::Text,

  /// List of all Vertex objects in Graph.
  pub vertices: diesel::types::Array<Vertex>,
}
