use crate::mesh::DefaultEdgePayload;

use super::{MeshBasics, MeshType};

/// Some basic operations to build meshes.
pub trait MeshBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Same as `add_isolated_edge` but with default edge payloads
    fn add_isolated_edge_default(&mut self, a: T::VP, b: T::VP) -> (T::V, T::V)
    where
        T::EP: DefaultEdgePayload;

    /// Generate a path from the finite iterator of positions and return the first and
    /// last edge resp. the arcs/halfedges pointing to the first and last vertex.
    fn insert_path(&mut self, vp: impl IntoIterator<Item = T::VP>) -> (T::E, T::E)
    where
        T::EP: DefaultEdgePayload;

    /// Same as `insert_path` but closes the path by connecting the last vertex with the first one.
    /// Also, returns only the first edge (outside the loop when constructed ccw).
    fn insert_loop(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E
    where
        T::EP: DefaultEdgePayload;
}
