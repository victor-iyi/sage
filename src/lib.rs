#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate dotenv;
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
