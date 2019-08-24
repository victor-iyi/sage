/// `Nodeable` trait should be implemented by every node variant.
pub trait Nodeable {}

pub struct Node {}

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
