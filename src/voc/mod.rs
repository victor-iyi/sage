//! `sage::voc` module implements an RDF namespace (vocabulary) registry

mod rdf;
mod rdfs;
mod schema;

pub use crate::voc::rdf::RdfVoc;
pub use crate::voc::rdfs::RdfsVoc;
pub use crate::voc::schema::SchemaVoc;



pub trait Vocabulary {
  type Prefix;
  type Full;

  fn prefix() -> Self::Prefix;
  fn full() -> Self::Full;
}
