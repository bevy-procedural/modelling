//! Module containing structs and traits for cursors.
//! Cursors are used to traverse the mesh and access its elements.
//!
//! There are 3 x 2 x 2 types of cursors:
//!
//! 1) Vertex cursors, iterating vertices
//! 2) Edge cursors, iterating edges
//! 3) Face cursors, iterating faces
//!
//! Each cursor type has variants based on:
//!
//! **Validity:**
//!
//!    1) MaybeCursor (default): May or may not point to a valid mesh element (can be void).
//!    2) ValidCursor: Guaranteed to point to a valid, existing mesh element.
//!
//! **Mutability:**
//!
//!    1) ImmutableCursor: Can be cloned freely, suitable for read-only access.
//!    2) MutableCursor: Holds a mutable reference to the mesh, enabling modification.

#[macro_use]
mod macro_helper;

#[macro_use]
mod data;

mod cursor;
mod edge;
mod face;
mod vertex;

pub use cursor::*;
pub use data::*;
pub use edge::*;
pub use face::*;
pub use vertex::*;
