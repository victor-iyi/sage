mod datastore;
pub mod error;
pub mod graph;
mod processor;
mod query;
pub mod schema;
pub mod types;
pub mod vocab;

/// Sage `Result` type.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// Sage `Result` type.
pub type SageResult<T, E = error::Error> = Result<T, E>;

/// Sage `Error` type.
pub type SageError = Error;

/// Re-exports important traits and types. Meant to be glob imported when using Sage.
pub mod prelude {
  // Sage Error handler functionalities.
  pub use crate::error::*;

  // Sage types & vocabularies.
  pub use crate::types::*;
  pub use crate::vocab::*;

  // Sage graphs, nodes, connections, predicates & triples.
  pub use crate::graph::*;

  // Sage schemas. Files and data sage can work with.
  // Example: jsonld, rdf, wikidata, etc.
  pub use crate::schema::*;
}

pub use prelude::*;
