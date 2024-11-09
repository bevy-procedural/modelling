use super::{basics::MeshBasics, MeshType};
use crate::{
    math::{HasPosition, VectorIteratorExt},
    mesh::VertexBasics,
};

/// Methods for transforming meshes.
pub trait MeshPosition<T: MeshType<Mesh = Self>>: MeshBasics<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec {
        self.vertices().map(|v| v.pos()).stable_mean()
    }
}
