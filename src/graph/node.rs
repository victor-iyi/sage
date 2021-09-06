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
use crate::types::{DTypes, URI};

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Node
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */

/// `Node` consists of various kinds of valid nodes that is supported by the `sage` engine.
///
/// `Node` represents each *entity* or *real world object* in the Knowledge Graph.
/// There are different variants of nodes.
///
/// `Node` is the crux of a `sage` knowledge graph, in which every *entity*
/// in the Knowledge Graph is regarded as a `Node` in `sage`.
///
#[derive(Debug, PartialEq)]
pub enum Node {
  /// `Blank` node containing node with empty or null data.
  Blank,

  /// `Schema` node is created from some type of structured data.
  /// For example: wikidata, jsonld, rdf, ntriple or even structs.
  Schema,

  /// `Http` node is used to represent data coming from an external/http source.
  /// And example of such [James Cameron](https://www.wikidata.org/wiki/Q42574)
  /// node gotten from [wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page).
  Http { uri: URI },

  /// `Literal` node is used to represent nodes with primitive types
  /// like `Strings`, `Numbers`, `Date`, `Time`, `DateTime` etc.
  /// which contains no extra data associated to this node.
  Literal { literal: DTypes },
}

struct Literal<T> {
  data: T,
  dtype: Option<DTypes>,
}

/// Implementation for `Node` enum.
impl Node {
  /// Check of `Node` is of type `Node::Blank`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Node;
  /// use sage::types::URI;
  ///
  /// let node_type = Node::Blank;
  /// assert_eq!(node_type.is_blank(), true);
  ///
  /// assert_eq!(Node::Schema.is_blank(), false);
  /// assert_eq!(Node::Http{ uri: URI::from("https://schema.org/Person")}.is_blank(), false);
  ///
  /// ```
  ///
  pub fn is_blank(&self) -> bool {
    matches!(*self, Node::Blank)
  }

  /// Check if `Node` is of type `Node::Schema`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Node;
  /// use sage::types::URI;
  ///
  /// let node_type = Node::Schema;
  /// assert_eq!(node_type.is_schema(), true);
  ///
  /// assert_eq!(Node::Http{ uri: URI::from("https://schema.org/Person") }.is_schema(), false);
  ///
  /// ```
  ///
  pub fn is_schema(&self) -> bool {
    matches!(*self, Node::Schema)
  }

  /// Check if `Node` is of type `Node::Http`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Node;
  /// use sage::types::URI;
  ///
  /// let node_type = Node::Http{ uri: URI::from("https://schema.org/Person")};
  /// assert_eq!(node_type.is_http(), true);
  ///
  /// assert_eq!(Node::Blank.is_http(), false);
  /// assert_eq!(Node::Schema.is_http(), false);
  ///
  /// ```
  ///
  pub fn is_http(&self) -> bool {
    matches!(*self, Node::Http { .. })
  }

  /// Check if `Node` is of type `Node::Literal`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::Node;
  ///
  /// let node_type = Node::Literal{ literal: "John Doe".to_string(), language: None, dtype: None};
  /// assert_eq!(node_type.is_literal(), true);
  ///
  /// assert_eq!(Node::Blank.is_literal(), false);
  /// assert_eq!(Node::Schema.is_literal(), false);
  ///
  /// ```
  ///
  pub fn is_literal(&self) -> bool {
    matches!(*self, Node::Literal { .. })
  }

  /// Returns the `Node` variant.
  ///
  /// # Example
  /// ```rust
  /// use sage::graph::Node;
  /// use sage::types::DTypes;
  ///
  /// // Assume `Node::Literal` was gotten dynamically.
  ///
  /// assert_eq!(Node::Blank.get_type(), &Node::Blank);
  /// let john : Node = Node::Literal{ literal: "John Doe".to_string(), language: None, dtype: None };
  /// assert_eq!(john.get_type(), &john);
  ///
  /// ```
  ///
  pub fn get_type(&self) -> &Node {
    &*self
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_type())
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | NodeId
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/// `NodeId` is a unique identifier assigned to every node in the Knowledge Graph.
///
///`NodeId` comes in form of `"sg:N4286"`.
#[derive(Debug)]
pub struct NodeId(String);

impl FromStr for NodeId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage node" unique ID pattern.
    let re = Regex::new(r"^sg:N\d+$").unwrap();

    if re.is_match(s) {
      Ok(NodeId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 189, 9))
    }
  }
}

impl Iterator for NodeId {
  type Item = NodeId;

  /// The generates new `NodeId` each time a new node is created.
  fn next(&mut self) -> Option<Self::Item> {
    let mut counter: u64 = 0;
    counter += 1;
    let ret = format!("{}{}", self.0, counter);
    Some(NodeId::from_str(&ret).unwrap())
  }
}

impl fmt::Display for NodeId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | NodeStore
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/// `NodeStore` consist of List of node items.
#[derive(Default)]
pub struct NodeStore {
  nodes: Vec<Node>,
}

impl NodeStore {
  /// Creates an empty instance of a `NodeStore`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeStore;
  ///
  /// let nodes = NodeStore::new();
  /// assert_eq!(nodes.len(), 0);
  /// ```
  pub fn new() -> NodeStore {
    NodeStore { nodes: Vec::new() }
  }

  pub fn nodes(&self) -> &[Node] {
    &self.nodes
  }

  /// Checks if the `NodeStore` is empty.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeStore;
  ///
  /// let nodes = NodeStore::new();
  /// assert_eq!(nodes.is_empty(), true);
  /// ```
  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  /// Returns the length of the nodes in the store.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeStore;
  ///
  /// let nodes = NodeStore::new();
  /// assert_eq!(nodes.len(), 0);
  /// ```
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | NodeImpl
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
#[derive(Debug)]
struct NodeImpl {
  /// Node ID should be inform of "sg:N4236".
  id: NodeId,
  // `Node` describes the variant of node the current node is.
  node_type: Node,
}

impl NodeImpl {
  fn new(node_type: Node) -> NodeImpl {
    NodeImpl {
      id: NodeId("sg:N".to_string()).next().unwrap(),
      node_type,
    }
  }

  /// Return the id of the current `Node`.
  fn id(&self) -> &str {
    &self.id.0
  }
}

impl Default for NodeImpl {
  fn default() -> Self {
    Self::new(Node::Blank)
  }
}

/*
/// `Entity` represents each *real world object* in the Knowledge Graph.
/// There are different variants of nodes which can be found in
/// `Node` enum.
///
/// `Node` is the crux of a `sage` knowledge graph, in which every *"entity"*
/// in the Knowledge Graph is regarded as a `Node` in `sage`.
///
/// As for the implementation of a `Node`, it has a private node implementation
/// which is only exposed through the `Node::node` and it is boxed for memory
/// management purposes. Higher level APIs are provided to work with `Node` as
/// effectively and efficiently as possible.
#[derive(Debug, Default)]
pub struct Entity {
  node: Box<NodeImpl>,
}

impl Entity {
  pub fn new(dtype: Node) -> Entity {
    Entity {
      node: Box::new(NodeImpl::new(dtype)),
    }
  }
}
*/
