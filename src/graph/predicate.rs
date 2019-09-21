#![allow(dead_code)]

use crate::vocab::Namespace;

/// Predicate is the actual data contained when two `Node`s are connected through some `ConnectionType`.
pub trait Pred<T> {}

/// `PredicateId` is a unique identifier assigned to every node in the Knowledge Graph.
///
/// Each `PredicateId` comes in form of `"sg:N4286"`.
pub struct PredicateId(String);

impl Iterator for PredicateId {
  type Item = String;

  /// The generates new `PredicateId` each time a new node is created.
  fn next(&mut self) -> Option<String> {
    let mut counter: u64 = 0;
    counter += 1;
    let ret = format!("{}{}", self.0, counter);
    Some(ret)
  }
}

struct PredicateImpl {
  id: String,
  pred_type: PredicateType,
}

impl PredicateImpl {
  fn new(pred_type: PredicateType) -> PredicateImpl {
    PredicateImpl {
      id: PredicateId("sg:P".to_string()).next().unwrap(),
      pred_type,
    }
  }
}

pub struct Predicate {
  predicate: Box<PredicateImpl>,
}

#[derive(Debug)]
pub enum PredicateType {
  /// *Literal predicate* describes the connection between two `Node`s
  /// in form of a string slice (`&str`) or `String`.
  Literal,

  /// *Uri predicate* describes the connection between two `Node`s in
  /// for of a `Namespace` which could expand into a full `IRI` or
  /// a short `IRI`.
  Uri(Namespace),
}
