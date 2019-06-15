mod load;
mod graph;

pub use load::from_jsonld;
pub use graph::{KnowledgeGraph, Vertex, Graph, Edge};
