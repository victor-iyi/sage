use crate::graph::*;

struct Triple {
  source: Node,
  connection: Connection,
  predicate: Predicate,
  destination: Node,
}

// TODO(victor): impl std::fmt::Display for Triple.
