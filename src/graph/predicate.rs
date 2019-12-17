#![allow(dead_code)]

use std::str::FromStr;

use regex::Regex;

use crate::error::{Error, ErrorCode};
use crate::vocab::Namespace;

/// Predicate is the actual data contained when two `Node`s are connected through some `ConnectionType`.
pub trait Pred<T> {}

/// `PredicateId` is a unique identifier assigned to every node in the Knowledge Graph.
///
/// Each `PredicateId` comes in form of `"sg:N4286"`.
pub struct PredicateId(String);

impl FromStr for PredicateId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage node" unique ID pattern.
    let re = Regex::new(r"^sg:P\d+$").unwrap();

    if re.is_match(s) {
      Ok(PredicateId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 49, 25))
    }
  }
}

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

#[derive(Debug)]
pub enum Predicate {
  /// *Literal predicate* describes the connection between two `Node`s
  /// in form of a string slice (`&str`) or `String`.
  Literal(String),

  /// *Uri predicate* describes the connection between two `Node`s in
  /// for of a `Namespace` which could expand into a full `IRI` or
  /// a short `IRI`.
  Uri(Namespace),
}

impl Predicate {
  /// Check if `Predicate` is of type `Predicate::Literal`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Predicate;
  /// use sage::vocab::Namespace;
  ///
  /// let pred = Predicate::Literal("John Doe".to_string());
  /// assert_eq!(pred.is_literal(), true);
  ///
  /// assert_eq!(Predicate::Literal("John Doe".to_string()).is_literal(), true);
  /// assert_eq!(Predicate::Uri(Namespace::default()).is_literal(), false);
  ///
  /// ```
  ///
  pub fn is_literal(&self) -> bool {
    match *self {
      Predicate::Literal(_) => true,
      _ => false,
    }
  }

  /// Check if `Predicate` is of type `Predicate::Uri`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Predicate;
  /// use sage::vocab::Namespace;
  ///
  /// let pred_type = Predicate::Uri(Namespace::default());
  /// assert_eq!(pred_type.is_uri(), true);
  ///
  /// assert_eq!(Predicate::Uri(Namespace::default()).is_uri(), true);
  /// assert_eq!(Predicate::Literal("John Doe".to_string()).is_uri(), false);
  ///
  /// ```
  ///
  pub fn is_uri(&self) -> bool {
    match *self {
      Predicate::Uri(_) => true,
      _ => false,
    }
  }
}

/*
struct PredicateImpl {
  id: String,
  pred_type: Predicate,
}

impl PredicateImpl {
  fn new(pred: Predicate) -> PredicateImpl {
    PredicateImpl {
      id: PredicateId("sg:P".to_string()).next().unwrap(),
      pred,
    }
  }
}

pub struct Predicate {
  predicate: Box<PredicateImpl>,
}
*/
