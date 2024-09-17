//! This module contains the builder functions for the mesh representation.

mod edge;
mod extrude;
mod face;
mod loft;
mod vertex;
mod subdivision;

use super::{Mesh, MeshType};
use crate::{math::Vector3D, representation::payload::HasPosition};
pub use subdivision::*;

impl<T: MeshType> Mesh<T>
where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Flips the y and z coordinates of all vertices.
    pub fn flip_yz(&mut self) -> &mut Self {
        self.vertices_mut().for_each(|v| {
            let pos = v.payload().pos().xzy();
            v.payload_mut().set_pos(pos)
        });
        self
    }
}
