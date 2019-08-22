//! **Shared Connection** creates a two-way connection between two nodes.
//! Both nodes share the same connection with each other.
//!
//! For example: `Jane <- friend of -> John`. Here, "Jane" is a *"friend of"* "John",
//! so also is "John" the *"friend of*" "Jane"'s.
//!
//! **Note** that this relationship can also be modelled as two forward nodes but
//! in the opposite direction.
//!
//! For example: `Jane --friend of-> John` & `John --friend of -> Jane`.
//! However, this is quite inefficient as we are creating unnecessary connections
//! and it doesn't really tell much about how they are related to one another.
