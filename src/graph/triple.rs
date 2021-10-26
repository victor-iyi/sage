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

use std::{fmt, str::FromStr};

use crate::{
  error::{Error, ErrorCode},
  graph::*,
};

use regex::Regex;

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Triple ID
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
#[derive(Debug, Eq)]
pub struct TripleId(String);

impl PartialEq for TripleId {
  fn eq(&self, other: &TripleId) -> bool {
    self.0 == other.0
  }
}

impl FromStr for TripleId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage triple" unique ID pattern.
    let re = Regex::new(r"^sg:T\d+$").unwrap();

    if re.is_match(s) {
      Ok(TripleId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 22, 27))
    }
  }
}

impl Iterator for TripleId {
  type Item = TripleId;

  /// The generates new `TripleId` each time a new node is created.
  fn next(&mut self) -> Option<TripleId> {
    let mut counter: u64 = 0;
    counter += 1;
    let ret = format!("{}{}", self.0, counter);
    Some(TripleId::from_str(&ret).unwrap())
  }
}

impl fmt::Display for TripleId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Triple
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
pub struct Triple {
  id: TripleId,
  source: Node,
  predicate: Predicate,
  destination: Node,
  connection: Connection,
}

impl Triple {
  pub fn new() -> Triple {
    Triple {
      id: TripleId("sg:T".to_string()).next().unwrap(),
      source: Node::Blank,
      predicate: Predicate::Literal("".to_string()),
      destination: Node::Blank,
      connection: Connection::Forward,
    }
  }

  #[doc(hidden)]
  pub fn id(&self) -> &TripleId {
    &self.id
  }

  #[doc(hidden)]
  pub fn connection(&self) -> &Connection {
    &self.connection
  }
}

impl PartialEq for Triple {
  fn eq(&self, other: &Triple) -> bool {
    self.id == other.id
  }
}

impl Default for Triple {
  fn default() -> Self {
    // TODO(victor): Add a default implementation like you did with `NamespaceStore`.
    //  you might wanna create BlankNodes, Forward connection, and generic predicate.
    Self::new()
  }
}

impl fmt::Display for Triple {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.connection() {
      Connection::Forward => write!(
        f,
        "{} \"{}\" -- {} -> \"{}\"",
        self.id, self.source, self.predicate, self.destination
      ),
      Connection::Multiple => write!(
        f,
        "{} \"{}\" -- {} -> {:?}",
        self.id, self.source, self.predicate, self.destination
      ),
      Connection::Relational => write!(
        f,
        "{} \"{}\" -- {} -> \"{}\"",
        self.id, self.source, self.predicate, self.destination
      ),
      Connection::Shared => write!(
        f,
        "{} \"{}\" <-- {} -> \"{}\"",
        self.id, self.source, self.predicate, self.destination
      ),
    }
    // write!(
    //   f,
    //   "{} {} -- {} -> {} ({})",
    //   self.id, self.source, self.predicate, self.destination, self.connection
    // )
  }
}
