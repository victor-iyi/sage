#![allow(unused_imports)]

extern crate dotenv;
#[macro_use]
extern crate log;
extern crate ntriple;
extern crate rdf;
extern crate serde_json;
extern crate uuid;

mod datastore;
mod error;
mod processor;
mod query;
pub mod schema;
pub mod types;
pub mod voc;

/// Sage crate `Result` type.
pub type SageResult<T> = std::result::Result<T, error::SageError>;

/// Re-exports important traits and types. Meant to be glob imported when using Sage.
pub mod prelude {
    pub use crate::error::SageError;

    pub use crate::types::{IRI, URI};
    pub use crate::voc::{RdfVoc, RdfsVoc, SchemaVoc, Vocabulary};

    pub use crate::schema::{Namespace, Namespaces};
}

pub use prelude::*;
