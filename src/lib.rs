#![allow(unused_imports)]

extern crate dotenv;
#[macro_use]
extern crate log;
extern crate ntriple;
extern crate rdf;
extern crate serde_json;
extern crate uuid;

mod cli;
mod datastore;
mod error;
mod processor;
mod query;
mod schema;


/// sage crate `Result` type.
pub type SageResult<T> = std::result::Result<T, error::SageError>;
