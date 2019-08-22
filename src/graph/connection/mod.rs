//! Sage represents real world information in a graph-like structure
//! it does this through building up connections among nodes/entities.
//! Here are some default connection supported by `sage` by default.
//!
//!  ## Connection Types
//!
//! - Forward connection
//! - Shared connection
//! - Relational connection
//! - multiple connection
//!
//! These connection types all implements the `sage::graph::Connection` trait.

mod connection;
mod forward;
mod multiple;
mod relational;
mod shared;

pub use connection::Connection;
