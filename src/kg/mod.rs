//! `sage::kg` is the Knowledge Graph sub-module responsible for
//! high level operation with the Knowledge Graph. This includes loading,
//! parsing, querying and creating new graph (amongst others).
//!
mod load;
mod graph;
mod schema;

pub use load::{from_jsonld, SUPPORTED_FORMATS};
pub use graph::{MultiKnowledgeGraph, KnowledgeGraph};
pub use schema::{Vertex, Graph, Edge};
