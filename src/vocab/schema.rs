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

use crate::types::IRI;
use crate::vocab::Vocabulary;

/// `SchemaVocab` contains constants of the <https://schema.org> vocabulary.
///
/// `SchemaVocab` implements the `Vocabulary` trait which provides
/// `SchemaVocab::prefix()` and `SchemaVocab::full()` by default.
///
/// ## Basic Usage
///
/// ```rust
/// use sage::types::IRI;
/// use sage::vocab::{Vocabulary, SchemaVocab};
///
/// assert_eq!(SchemaVocab::prefix(), IRI::from("schema:"));
/// assert_eq!(SchemaVocab::full(), IRI::from("https://schema.org/"));
/// ```
pub struct SchemaVocab;

impl Vocabulary for SchemaVocab {
  type Prefix = IRI;
  type Full = IRI;

  fn prefix() -> Self::Prefix {
    IRI::from("schema:")
  }

  fn full() -> Self::Full {
    IRI::from("https://schema.org/")
  }
}
