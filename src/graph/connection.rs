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

//!  ## Connections
//!
//! - Forward connection
//! - Shared connection
//! - Relational connection
//! - Multiple connection
//!
//! These connection types all implements the `sage::graph::Connection` trait.
//!

use std::fmt;

/*
/// `Connection` trait should be implemented by every connection type.
pub trait Connection {}
*/

/// `sage` represents real world information in a graph-like structure.
/// It does this through building up connections among entities. This
/// connections are named `Connection` abd consists of many variants.
/// Here are some possible connections that can occur among entities
/// (or nodes) in the graph.
#[derive(Debug)]
pub enum Connection {
  /// *Forward Connection* connects two nodes together at a time.
  /// This connection might occur multiple times.
  ///
  /// For example: `John --born in-> London` & `John --current location-> Fiji`.
  /// Here, "John" is connected to both "London" and "Fiji", however "London" &
  /// "Fiji" doesn't necessarily have a direct connection with "John".
  Forward,

  /// *Shared Connection* creates a two-way connection between two nodes.
  /// Both nodes share the same connection with each other.
  ///
  /// For example: `Jane <- friend of -> John`. Here, "Jane" is a *"friend of"* "John",
  /// so also is "John" the *"friend of*" "Jane"'s.
  ///
  /// **Note** that this relationship can also be modelled as two forward nodes but
  /// in the opposite direction.
  ///
  /// For example: `Jane --friend of-> John` & `John --friend of -> Jane`.
  /// However, this is quite inefficient as we are creating unnecessary connections
  /// and it doesn't really tell much about how they are related to one another.
  Shared,

  /// *Relational connection* represents relationship between two
  /// or more nodes such that the first node has a different relationship
  /// with the second and the second has a different relationship with the first.
  ///
  /// For example: `John --son-> Bob` & `Bob --father->John`. Here, "John" shares
  /// a "son" relationship with "Bob" and "Bob" shares a "father" relationship with "John".
  Relational,

  /// **Multiple connection** shares the same connection with many other nodes.
  /// Note that the target nodes does not necessarily share a relationship with
  /// the source node.
  ///
  /// For Example: `John --speaks-> [English, French, Spanish, Dutch]`.
  /// Here, "John" has a forward relationship with those languages however the
  /// languages doesn't necessarily share a relationship with "John".
  Multiple,
}

impl Connection {
  #[doc(hidden)]
  pub fn is_forward(&self) -> bool {
    matches!(*self, Connection::Forward)
  }

  #[doc(hidden)]
  pub fn is_shared(&self) -> bool {
    matches!(*self, Connection::Shared)
  }

  #[doc(hidden)]
  pub fn is_relational(&self) -> bool {
    matches!(*self, Connection::Relational)
  }

  #[doc(hidden)]
  pub fn is_multiple(&self) -> bool {
    matches!(*self, Connection::Multiple)
  }
}

impl fmt::Display for Connection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let conn_type: &str = match self {
      Connection::Forward => "Forward",
      Connection::Shared => "Shared",
      Connection::Relational => "Relational",
      Connection::Multiple => "Multiple",
    };
    f.write_str(conn_type)
  }
}
