mod connection;
mod node;
mod predicate;
mod triple;

pub use connection::{Connection, ConnectionType};
pub use node::{Node, NodeStore, NodeType};
pub use predicate::{Predicate, PredicateType};
pub use triple::Triple;

// TODO(victor): Generate unique ID for the  Knowledge `GraphScore`. Node ID will be inform of "sg:N4286" while predicate will be inform of "sg:P5245".
