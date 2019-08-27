#![allow(dead_code)]

/// `NodeTypes` consists of various kinds of valid nodes that is supported by the `sage` engine.
///
/// **NOTE:** This is not to be misplaced for Data types or a `Node` itself.
///
/// TLDR; `NodeType` represent the different forms which `Node` can exist.
#[derive(Debug)]
pub enum NodeType {
  /// `BlankNode` containing node with empty or null data.
  BlankNode,

  /// `SchemaNode` is created from some type of data structure.
  /// Usually but not limited to `struct`s.
  SchemaNode,

  /// `HttpNode` is used to represent data coming from an external/http source.
  /// And example of such [James Cameron](https://www.wikidata.org/wiki/Q42574)
  /// node gotten from [wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page).
  HttpNode,

  /// `LiteralNode` is used to represent nodes with primitive types
  /// like `Strings`, `Numbers`, `Date`, `Time`, `DateTime` etc.
  /// which contains no extra data associated to this node.
  LiteralNode,
}

/// `Nodeable` trait should be implemented by every node variant.
pub trait Nodeable<T> {}

pub struct Node {
  node: Box<NodeImpl>,
}

struct NodeImpl {}

/// `NodeStore` consist of List of node items.
#[derive(Default)]
pub struct NodeStore {
  nodes: Vec<Node>,
}

impl NodeStore {
  /// Creates a new instance of a `NodeStore`.
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
