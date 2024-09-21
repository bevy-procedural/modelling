//! This module contains the builder functions for the mesh representation.

mod edge;
mod extrude;
mod loft;
mod subdivision;

use super::{Mesh, MeshType};
use crate::{math::Vector3D, mesh::payload::HasPosition};
pub use subdivision::*;
