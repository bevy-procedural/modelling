use super::{Mesh, MeshType};
use crate::{math::VectorIteratorExt, mesh::payload::HasPosition};

impl<T: MeshType> Mesh<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Returns the mean of all vertex positions.
    pub fn centroid(&self) -> T::Vec {
        self.vertices().map(|v| *v.pos()).stable_mean()
    }
}