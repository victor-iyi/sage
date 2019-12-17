#![allow(dead_code)]

extern crate dotenv;
// #[macro_use]
// extern crate log;
extern crate ntriple;
extern crate rand;
extern crate rdf;
extern crate regex;
extern crate serde_json;
extern crate uuid;

mod datastore;
pub mod error;
pub mod graph;
mod processor;
mod query;
pub mod schema;
pub mod types;
pub mod vocab;

/// Sage crate `Result` type.
pub type SageResult<T> = std::result::Result<T, error::SageError>;

/// Re-exports important traits and types. Meant to be glob imported when using Sage.
pub mod prelude {
  // Sage Error handler functionalities.
  pub use crate::error::*;

  // Sage types & vocabularies.
  pub use crate::types::*;
  pub use crate::vocab::*;

  // Sage graph, nodes, connections & ...
  pub use crate::graph::*;

  // Sage namespace & namespace store.
  pub use crate::schema::*;
}

pub use prelude::*;
