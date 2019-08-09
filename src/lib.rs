#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate serde_json;
extern crate uuid;


pub mod error;
pub mod kg;
pub mod utils;


/// sage crate `Result` type.
///
/// ## Example
/// ```
/// use sage::{SageResult, kg};
///
/// /// Attempts to create Knowledge Graph from a file.
/// fn graph_from_file(path: impl AsRef<str>) -> SageResult<kg::KnowledgeGraph> {
///   unimplemented!()
/// }
/// ```
pub type SageResult<T> = std::result::Result<T, error::SageError>;
