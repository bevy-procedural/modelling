use super::VertexBasics;
use crate::mesh::MeshType;

/// Basic vertex iterators for a mesh
pub trait VertexIterators<T: MeshType>: VertexBasics<T> {
    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Vertex> + 'a;

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Face> + 'a
    where
        T: 'a;

    /// Iterates the ids of all neighbors of the vertex
    fn neighbor_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::V> + 'a {
        self.vertices(mesh).map(|v| v.id())
    }
}
