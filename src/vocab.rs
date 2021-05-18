//! `sage::vocab` module implements an
//! [Resource Description Framework (RDF)](https://en.wikipedia.org/wiki/Resource_Description_Framework)
//!  namespace (or vocabulary) registry.

mod namespace;
mod rdf;
mod rdfs;
mod schema;
mod vocabulary;

// Ambiguous export.
pub use crate::vocab::rdf::RdfVocab;

// Unambiguous export.
pub use namespace::{Namespace, NamespaceStore, Namespaces, URI};
pub use rdfs::RdfsVocab;
pub use schema::SchemaVocab;
pub use vocabulary::Vocabulary;
