//! Module containing structs and traits for cursors.
//! Cursors are used to traverse the mesh and access its elements.
//!
//! There are 3 x 2 x 2 types of cursors:
//!
//! 1) Vertex cursors, iterating vertices
//! 2) Edge cursors, iterating edges
//! 3) Face cursors, iterating faces
//!
//! Each of them is available in different variants:
//!
//! 1) Valid cursors, which are known to point to an existing element
//! 2) Maybe cursors (default), which may point to an existing element or be void
//!
//! Which, each, can be mutable or immutable, i.e., hold a mutable or immutable 
//! reference to the mesh to either allow modifying the mesh or cloning the cursor.

mod cursor;
mod edge;
mod face;
mod vertex;

pub use cursor::*;
pub use edge::*;
pub use face::*;
pub use vertex::*;
