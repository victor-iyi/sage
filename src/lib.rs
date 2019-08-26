#![allow(dead_code)]

extern crate dotenv;
// #[macro_use]
// extern crate log;
extern crate ntriple;
extern crate rdf;
extern crate serde_json;
extern crate uuid;

mod datastore;
pub mod error;
pub mod graph;
mod processor;
mod query;
pub mod schema;
pub mod types;
pub mod voc;

/// Sage crate `Result` type.
pub type SageResult<T> = std::result::Result<T, error::SageError>;

/// Re-exports important traits and types. Meant to be glob imported when using Sage.
pub mod prelude {
    // Sage Error handler functionalities.
    pub use crate::error::{Category, Error, ErrorCode, Result, SageError, SageResult};

    // Sage types & vocabularies.
    pub use crate::types::{IRI, URI};
    pub use crate::voc::{RdfVoc, RdfsVoc, SchemaVoc, Vocabulary};

    // Sage graph, nodes, connections & ...
    pub use crate::graph::{Connection, ConnectionType, Node, NodeStore, NodeType};

    // Sage namespace & namespace store.
    pub use crate::schema::{Namespace, Namespaces};
}

pub use prelude::*;
