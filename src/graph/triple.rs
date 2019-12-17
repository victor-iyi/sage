use std::fmt;

use crate::graph::*;

struct TripleID;

pub struct Triple {
  id: TripleID,
  source: Node,
  connection: Connection,
  predicate: Predicate,
  destination: Node,
}

impl Triple {
  pub fn new() -> Triple {
    unimplemented!()
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
    write!(f, "Node -- predicate -> Node ({})", self.connection)
  }
}
