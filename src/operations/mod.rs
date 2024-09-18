//! This module contains the builder functions for the mesh representation.

mod dual;
mod edge;
mod extrude;
mod face;
mod halfedge;
mod loft;
mod subdivision;
mod vertex;

use super::{Mesh, MeshType};
use crate::{math::Vector3D, mesh::payload::HasPosition};
pub use subdivision::*;
