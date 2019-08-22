mod connection;
mod node;

pub use node::{Node, NodeStore, NodeTypes};
pub use connection::Connection;

// TODO(victor): Generate unique ID for the  Knowledge `GraphScore`. Node ID will be inform of "sg:N4286" while predicate will be inform of "sg:P5245".
