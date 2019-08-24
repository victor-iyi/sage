//! Sage represents real world information in a graph-like structure
//! it does this through building up connections among nodes/entities.
//! Here are some default connection supported by `sage` by default.
//!
//!  ## ConnectionTypes
//!
//! - Forward connection
//! - Shared connection
//! - Relational connection
//! - multiple connection
//!
//! These connection types all implements the `sage::graph::Connection` trait.

mod connection_impl;
mod predicate;
mod types;

pub use connection_impl::Connection;
pub use types::ConnectionType;
