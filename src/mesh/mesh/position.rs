use crate::{
    math::{Vector, VectorIteratorExt},
    mesh::{EuclideanMeshType, MeshBasics, VertexBasics},
};

/// Methods for transforming meshes.
pub trait MeshPosition<const D: usize, T: EuclideanMeshType<D, Mesh = Self>>:
    MeshBasics<T>
{
    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec {
        self.vertices().map(|v| v.pos()).stable_mean()
    }

    /// Returns the closest vertex to a given position.
    /// Without a spatial data structure, this takes O(n) time.
    fn closest_vertex<'a>(&'a self, pos: T::Vec) -> Option<&'a T::Vertex>
    where
        T: 'a,
    {
        self.vertices()
            .map(|v| (v, v.pos().distance_squared(&pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(v, _)| v)
    }
}
