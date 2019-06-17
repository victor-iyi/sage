mod load;
mod graph;

pub use load::from_jsonld;
pub use graph::{MultiKnowledgeGraph, KnowledgeGraph, Vertex, Graph, Edge};
