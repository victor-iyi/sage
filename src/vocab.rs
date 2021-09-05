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
