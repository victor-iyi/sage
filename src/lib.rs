#![allow(unused_imports)]

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate serde_json;
extern crate uuid;


mod cli;
pub mod error;

/// sage crate `Result` type.
pub type SageResult<T> = std::result::Result<T, error::SageError>;
