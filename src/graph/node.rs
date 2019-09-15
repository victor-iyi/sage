#![allow(dead_code)]

/// `NodeTypes` consists of various kinds of valid nodes that is supported by the `sage` engine.
///
/// **NOTE:** This is not to be misplaced for Data types or a `Node` itself.
///
/// TLDR; `NodeType` represent the different forms which `Node` can exist.
#[derive(Debug)]
pub enum NodeType<T> {
  /// `Blank` node containing node with empty or null data.
  Blank(T),

  /// `Schema` node is created from some type of data structure.
  /// Usually but not limited to `struct`s.
  Schema(T),

  /// `Http` node is used to represent data coming from an external/http source.
  /// And example of such [James Cameron](https://www.wikidata.org/wiki/Q42574)
  /// node gotten from [wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page).
  Http(T),

  /// `Literal` node is used to represent nodes with primitive types
  /// like `Strings`, `Numbers`, `Date`, `Time`, `DateTime` etc.
  /// which contains no extra data associated to this node.
  Literal(T),
}

impl<T> NodeType<T> {
  pub fn get_type(&self) -> NodeType<T> {
    unimplemented!()
  }
}

/// `NodeId` is a unique identifier assigned to every node in the Knowledge Graph.
///
/// Each `NodeId` comes in form of `"sg:N4286"`.
pub struct NodeId(String);

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

/// `Nodeable` trait should be implemented by every node variant.
pub trait Nodeable<T> {
  type ItemType;

  fn node_type(&self) -> Self::ItemType;
}

pub struct SchemaNode {}

pub struct Node {
  node: Box<NodeImpl>,
}

impl Node {}

struct NodeImpl {
  /// Node ID should be inform of "sg:N4236".
  id: String,
  // `NodeType` describes the variant of node the current node is.
  // node_type: NodeType<T>,
}

impl NodeImpl {
  fn new() -> NodeImpl {
    NodeImpl {
      id: NodeId("sg:N".to_string()).next().unwrap(),
      // node_type,
    }
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
