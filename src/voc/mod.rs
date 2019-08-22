//! `sage::voc` module implements an
//! [Resource Description Framework (RDF)](https://en.wikipedia.org/wiki/Resource_Description_Framework)
//!  namespace (or vocabulary) registry.

mod rdf;
mod rdfs;
mod schema;
mod voc;

pub use crate::voc::rdf::RdfVoc;
pub use crate::voc::rdfs::RdfsVoc;
pub use crate::voc::schema::SchemaVoc;
pub use crate::voc::voc::Vocabulary;
