// Copyright 2021 Victor I. Afolabi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `Sage is an open source [Knowledge Graph] used to represent linked-data.
//! It comprises of varieties of features that makes it stand out amongst other
//! (open source) Knowledge Graphs.
//!
//! [Knowledge Graph]: https://en.wikipedia.org/wiki/Knowledge_Graph

mod datastore;
pub mod error;
pub mod graph;
#[macro_use]
mod macros;
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

  // Export macros.
  pub use crate::macros::*;
}

pub use prelude::*;
