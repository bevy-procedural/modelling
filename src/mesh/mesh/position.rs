use super::{basics::MeshBasics, EuclideanMeshType};
use crate::{math::VectorIteratorExt, mesh::VertexBasics};

/// Methods for transforming meshes.
pub trait MeshPosition<const D: usize, T: EuclideanMeshType<D, Mesh = Self>>:
    MeshBasics<T>
{
    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec {
        self.vertices().map(|v| v.pos()).stable_mean()
    }
}
