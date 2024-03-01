//! This module contains the representation of the mesh. It defines the vertex, edge, face and mesh structs.

mod deletable;
mod edge;
mod face;
mod index_type;
mod mesh;
mod vertex;

pub use deletable::*;
pub use edge::*;
pub use face::*;
pub use index_type::*;
pub use mesh::*;
pub use vertex::*;
