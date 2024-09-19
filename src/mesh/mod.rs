//! This module contains the representation of the mesh. It defines the vertex, edge, face and mesh structs.

mod edge;
mod face;
mod mesh;
mod vertex;
mod triangulation;

pub use edge::*;
pub use face::*;
pub use mesh::*;
pub use vertex::*;
pub use triangulation::*;
