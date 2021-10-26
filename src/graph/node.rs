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

use regex::Regex;

use crate::{
  dtype::{DType, URI},
  error::{Error, ErrorCode},
};

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
  Http(URI),

  /// `Literal` node is used to represent nodes with primitive types
  /// supported by `sage::DType`. For example: `Numbers`, `String`, `Array`,
  /// `Object`, `Boolean`, `DateTime` & `Null`.
  ///
  /// For example [James Cameron] - [directed] -> [Avatar, Titanic, Terminator].
  Literal(DType),

  /// `Multiple` node is created from
  Multiple(Vec<Node>),
}

/// Implementation for `Node` enum.
impl Node {
  /// Check of `Node` is of type `Node::Blank`.
  ///
  /// ```rust
  /// # use sage::{dtype::URI, graph::Node};
  /// #
  /// let node_type = Node::Blank;
  /// assert!(node_type.is_blank());
  ///
  /// # assert!(!Node::Schema.is_blank());
  /// # assert!(!Node::Http(URI::from("https://schema.org/Person")).is_blank());
  /// ```
  ///
  pub fn is_blank(&self) -> bool {
    matches!(*self, Node::Blank)
  }

  /// Check if `Node` is of type `Node::Schema`.
  ///
  /// ```rust
  /// # use sage::{dtype::URI, graph::Node};
  /// #
  /// let node_type = Node::Schema;
  /// assert!(node_type.is_schema());
  ///
  /// #
  /// # assert!(!Node::Http(URI::from("https://schema.org/Person")).is_schema());
  /// ```
  ///
  pub fn is_schema(&self) -> bool {
    matches!(*self, Node::Schema)
  }

  /// Check if `Node` is of type `Node::Http`.
  ///
  /// ```rust
  /// # use sage::{dtype::URI, graph::Node};
  /// #
  /// let node_type = Node::Http(URI::from("https://schema.org/Person"));
  /// assert!(node_type.is_http());
  /// #
  /// # assert!(!Node::Blank.is_http());
  /// # assert!(!Node::Schema.is_http());
  /// ```
  ///
  pub fn is_http(&self) -> bool {
    matches!(*self, Node::Http(_))
  }

  /// Check if `Node` is of type `Node::Literal`.
  ///
  /// ```rust
  /// # use sage::{dtype::DType, graph::Node};
  /// #
  /// let node_type = Node::Literal(DType::String("John Doe".to_string()));
  /// assert!(node_type.is_literal());
  ///
  /// #
  /// # assert!(!Node::Blank.is_literal());
  /// # assert!(!Node::Schema.is_literal());
  /// ```
  ///
  pub fn is_literal(&self) -> bool {
    matches!(*self, Node::Literal(_))
  }

  /// Check if `Node` is of type `Node::Multiple`.
  ///
  /// ```rust
  /// # use sage::graph::Node;
  ///
  /// let node_type = Node::Multiple(vec![
  ///   Node::Literal("Rust".into()),
  ///   Node::Literal("Java".into()),
  ///   Node::Literal("Python".into()),
  /// ]);
  ///
  /// assert!(node_type.is_multiple());
  /// ```
  ///
  pub fn is_multiple(&self) -> bool {
    matches!(*self, Node::Multiple(_))
  }

  /// Returns the `Node` variant.
  ///
  /// ```rust
  /// # use sage::graph::Node;
  /// # use sage::dtype::DType;
  /// #
  /// let john : Node = Node::Literal(DType::String("John Doe".to_string()));
  /// assert_eq!(john.get_type(), &john);
  ///
  /// assert_eq!(Node::Blank.get_type(), &Node::Blank);
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
