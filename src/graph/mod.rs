mod connection;
mod node;
mod predicate;
mod triple;

pub use connection::Connection;
pub use node::{Node, NodeStore};
pub use predicate::Predicate;
pub use triple::Triple;

// TODO(victor): Generate unique ID for the  Knowledge `GraphScore`. Node ID will be inform of "sg:N4286" while predicate will be inform of "sg:P5245".
