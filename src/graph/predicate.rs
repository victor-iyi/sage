#![allow(dead_code)]

#[derive(Debug)]
pub enum PredicateType {
  /// *Literal predicate* describes the connection between two `Node`s
  /// in form of a string slice (`&str`) or `String` or `IRI`.
  Literal,

  /// *Uri predicate* describes the connection between two `Node`s in
  /// for of a `URI`.
  Uri,
}
