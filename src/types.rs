//! `sage::types` contains all/most types used by the `sage` engine. Many types wrap
//!  native Rust types. Although it's highly advised to use these types rather than
//! native rust because they include extended functionalities and can also be dereferenced
//!  back and forth into native Rust types  like [Strings](https://doc.rust-lang.org/stable/alloc/string/struct.String.html) and sage types.
//!

/// `IRI` stands for International Resource Identifer. (ex: <name>).
pub type IRI = String;

/// `URI` is used to represent any URL-like `IRI`.
pub type URI = String;

#[derive(Debug, Clone, PartialEq)]
pub enum DTypes {
  Boolean(bool),
  Text(String),
  Number,
  Time,
  DateTime,
}

#[derive(Debug)]
enum Number {
  Float(f64),
  Integer(isize),
}
