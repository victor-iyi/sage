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
