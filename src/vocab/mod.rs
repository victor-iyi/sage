//! `sage::vocab` module implements an
//! [Resource Description Framework (RDF)](https://en.wikipedia.org/wiki/Resource_Description_Framework)
//!  namespace (or vocabulary) registry.

mod rdf;
mod rdfs;
mod schema;
mod vocabulary;

pub use crate::vocab::rdf::RdfVoc;
pub use crate::vocab::rdfs::RdfsVoc;
pub use crate::vocab::schema::SchemaVoc;
pub use crate::vocab::vocabulary::Vocabulary;
