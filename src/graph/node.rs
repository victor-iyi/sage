#![allow(dead_code)]
use std::fmt;
use std::str::FromStr;

use regex::Regex;

use crate::error::{Error, ErrorCode};

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Public Enums.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/// `NodeTypes` consists of various kinds of valid nodes that is supported by the `sage` engine.
///
/// **NOTE:** This is not to be misplaced for Data types or a `Node` itself.
///
/// TLDR; `NodeType` represent the different forms which `Node` can exist.
#[derive(Debug, PartialEq)]
pub enum NodeType {
  /// `Blank` node containing node with empty or null data.
  Blank,

  /// `Schema` node is created from some type of data structure.
  /// Usually but not limited to `struct`s.
  Schema,

  /// `Http` node is used to represent data coming from an external/http source.
  /// And example of such [James Cameron](https://www.wikidata.org/wiki/Q42574)
  /// node gotten from [wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page).
  Http,

  /// `Literal` node is used to represent nodes with primitive types
  /// like `Strings`, `Numbers`, `Date`, `Time`, `DateTime` etc.
  /// which contains no extra data associated to this node.
  Literal,
}

/// Implementation for `NodeType` enum.
impl NodeType {
  /// Check of `NodeType` is of type `NodeType::Blank`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeType;
  ///
  /// let node_type = NodeType::Blank;
  /// assert_eq!(node_type.is_blank(), true);
  ///
  /// assert_eq!(NodeType::Schema.is_blank(), false);
  /// assert_eq!(NodeType::Http.is_blank(), false);
  ///
  /// ```
  ///
  pub fn is_blank(&self) -> bool {
    match *self {
      NodeType::Blank => true,
      _ => false,
    }
  }

  /// Check if `NodeType` is of type `NodeType::Schema`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeType;
  ///
  /// let node_type = NodeType::Schema;
  /// assert_eq!(node_type.is_schema(), true);
  ///
  /// assert_eq!(NodeType::Literal.is_schema(), false);
  /// assert_eq!(NodeType::Http.is_schema(), false);
  ///
  /// ```
  ///
  pub fn is_schema(&self) -> bool {
    match *self {
      NodeType::Schema => true,
      _ => false,
    }
  }

  /// Check if `NodeType` is of type `NodeType::Http`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeType;
  ///
  /// let node_type = NodeType::Http;
  /// assert_eq!(node_type.is_http(), true);
  ///
  /// assert_eq!(NodeType::Blank.is_http(), false);
  /// assert_eq!(NodeType::Schema.is_http(), false);
  ///
  /// ```
  ///
  pub fn is_http(&self) -> bool {
    match *self {
      NodeType::Http => true,
      _ => false,
    }
  }

  /// Check if `NodeType` is of type `NodeType::Literal`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use sage::graph::NodeType;
  ///
  /// let node_type = NodeType::Literal;
  /// assert_eq!(node_type.is_literal(), true);
  ///
  /// assert_eq!(NodeType::Blank.is_literal(), false);
  /// assert_eq!(NodeType::Schema.is_literal(), false);
  ///
  /// ```
  ///
  pub fn is_literal(&self) -> bool {
    match *self {
      NodeType::Literal => true,
      _ => false,
    }
  }

  /// Returns the `NodeType` variant.
  ///
  /// # Example
  /// ```rust
  /// use sage::graph::NodeType;
  ///
  /// // Assume `NodeType::Literal` was gotten dynamically.
  /// let node_type: NodeType = NodeType::Literal;
  ///
  /// assert_eq!(node_type.get_type(), NodeType::Literal);
  /// assert_eq!(NodeType::Blank.get_type(), NodeType::Blank);
  /// ```
  ///
  pub fn get_type(&self) -> NodeType {
    match *self {
      NodeType::Blank => NodeType::Blank,
      NodeType::Schema => NodeType::Schema,
      NodeType::Literal => NodeType::Literal,
      NodeType::Http => NodeType::Http,
    }
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Public Struct(s).
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */

/// `NodeId` is a unique identifier assigned to every node in the Knowledge Graph.
///
///`NodeId` comes in form of `"sg:N4286"`.
pub struct NodeId(String);

impl FromStr for NodeId {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Match the "sage node" unique ID pattern.
    let re = Regex::new(r"^sg:N\d+$").unwrap();

    if re.is_match(s) {
      Ok(NodeId(String::from(s)))
    } else {
      Err(Error::syntax(ErrorCode::RegexParser, 49, 25))
    }
  }
}

impl Iterator for NodeId {
  type Item = String;

  /// The generates new `NodeId` each time a new node is created.
  fn next(&mut self) -> Option<String> {
    let mut counter: u64 = 0;
    counter += 1;
    let ret = format!("{}{}", self.0, counter);
    Some(ret)
  }
}

/// `Node` represents each *"entity"* or *"real world object"* in the Knowledge Graph.
/// There are different variants of nodes which can be found in
/// `NodeType` enum.
///
/// `Node` is the crux of a `sage` knowledge graph, in which every *"entity"*
/// in the Knowledge Graph is regarded as a `Node` in `sage`.
///
/// As for the implementation of a `Node`, it has a private node implementation
/// which is only exposed through the `Node::node` and it is boxed for memory
/// management purposes. Higher level APIs are provided to work with `Node` as
/// effectively and efficiently as possible.
#[derive(Debug, Default)]
pub struct Node {
  node: Box<NodeImpl>,
}

impl Node {
  pub fn new() -> Node {
    unimplemented!()
  }
}

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | Public Traits.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
 */
/// `Nodeable` trait should be implemented by every node variant.
pub trait Nodeable<T> {
  type ItemType;

  fn node_type(&self) -> Self::ItemType;
}
#[derive(Debug)]
struct NodeImpl {
  /// Node ID should be inform of "sg:N4236".
  id: String,
  // `NodeType` describes the variant of node the current node is.
  node_type: NodeType,
}

impl NodeImpl {
  fn new(node_type: NodeType) -> NodeImpl {
    NodeImpl {
      id: NodeId("sg:N".to_string()).next().unwrap(),
      node_type,
    }
  }

  fn id(&self) -> &str {
    &self.id
  }
  // fn get_data<T: Nodeable>(node_type: T) -> NodeImpl {
  //   match node_type {
  //     NodeType::Blank(_) => {}
  //     NodeType::Schema(val) => val,
  //     NodeType::Http(val) => val,
  //     NodeType::Literal(val) => val,
  //   }

  //   unimplemented!()
  // }
}

impl Default for NodeImpl {
  fn default() -> Self {
    Self::new(NodeType::Blank)
  }
}

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

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // TODO(victor): Proper display for node. `Node` should be replaced with either it's type or the lable of the node.
    write!(f, "Node({})", self.node.id)
  }
}
