//!  ## ConnectionTypes
//!
//! - Forward connection
//! - Shared connection
//! - Relational connection
//! - Multiple connection
//!
//! These connection types all implements the `sage::graph::Connection` trait.
//!

/// `Connection` trait should be implemented by every connection type.
pub trait Connection {}

/// `sage` represents real world information in a graph-like structure.
/// It does this through building up connections among entities. This
/// connections are named `ConnectionType` abd consists of many variants.
/// Here are some possible connections that can occur among entities
/// (or nodes) in the graph.
#[derive(Debug)]
pub enum ConnectionType {
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
  /// For Example: `John --speaks-> [English, French, Spanish, Dutch].
  /// Here, "John" has a forward relationship with those languages however the
  /// languages doesn't necessarily share a relationship with "John".
  Multiple,
}
