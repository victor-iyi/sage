use regex::Regex;
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, ErrorCode};
use crate::graph::*;

#[derive(Debug)]
pub struct TripleId(String);

impl FromStr for TripleId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage triple" unique ID pattern.
    let re = Regex::new(r"^sg:T\d+$").unwrap();

    if re.is_match(s) {
      Ok(TripleId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 49, 25))
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
}

impl Default for Triple {
  fn default() -> Self {
    // TODO(victor): Add a default implementation like you did with `NamespaceStore`.
    //  you might wanna create BlankNodes, Forward connection, and generic predicate.
    Self::new()
  }
}

// TODO(victor): impl std::fmt::Display for Triple.
impl fmt::Display for Triple {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} {} -- {} -> {} ({})",
      self.id, self.source, self.predicate, self.destination, self.connection
    )
  }
}
