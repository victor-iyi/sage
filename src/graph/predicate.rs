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

#![allow(dead_code)]

use std::fmt;
use std::str::FromStr;

use regex::Regex;

use crate::error::{Error, ErrorCode};
use crate::vocab::Namespace;

/// Predicate is the actual data contained when two `Node`s are connected through some `ConnectionType`.
pub trait Pred<T> {}

/// `PredicateId` is a unique identifier assigned to every node in the Knowledge Graph.
///
/// Each `PredicateId` comes in form of `"sg:N4286"`.
/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | PredicateId
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/// `PredicateId` is a unique identifier assigned to every node in the Knowledge Graph.
///
///`PredicateId` comes in form of `"sg:P8080"`.
#[derive(Debug)]
pub struct PredicateId(String);

impl FromStr for PredicateId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage predicate" unique ID pattern.
    let re = Regex::new(r"^sg:P\d+$").unwrap();

    if re.is_match(s) {
      Ok(PredicateId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 37, 8))
    }
  }
}

impl Iterator for PredicateId {
  type Item = PredicateId;

  /// The generates new `PredicateId` each time a new node is created.
  fn next(&mut self) -> Option<PredicateId> {
    let mut counter: u64 = 0;
    counter += 1;
    let ret = format!("{}{}", self.0, counter);
    Some(PredicateId::from_str(&ret).unwrap())
  }
}

impl fmt::Display for PredicateId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, PartialEq)]
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
    matches!(*self, Predicate::Literal(_))
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
    matches!(*self, Predicate::Uri(_))
  }

  /// Returns the `Predicate` variant.
  ///
  /// # Example
  /// ```rust
  /// use sage::graph::Predicate;
  /// use sage::vocab::Namespace;
  ///
  /// let pred : Predicate = Predicate::Uri(Namespace::default());
  /// assert_eq!(pred.get_type(), &Predicate::Uri(Namespace::default()));
  /// ```
  ///
  pub fn get_type(&self) -> &Predicate {
    &*self
  }
}

impl fmt::Display for Predicate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_type())
  }
}

struct PredicateImpl {
  id: PredicateId,
  pred_type: Predicate,
}

impl PredicateImpl {
  fn new(pred_type: Predicate) -> PredicateImpl {
    PredicateImpl {
      id: PredicateId("sg:P".to_string()).next().unwrap(),
      pred_type,
    }
  }
}
