use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
};

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
pub struct ValidVertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> ValidVertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }
}

impl_debug_eq_cursor!(ValidVertexCursorMut, vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, ValidVertexCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    FC, move_to_face, T::F, FaceCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   ValidCursor, MutableCursor, ValidVertexCursorMut, VertexCursorMut,
   vertex, V, Vertex, VP, 
   get_vertex, get_vertex_mut, has_vertex,
   ValidVertexCursorBasics, VertexCursorBasics, VertexCursorHalfedgeBasics, VertexCursorBuilder
);

impl<'a, T: MeshType> ValidVertexCursorMut<'a, T> {
    /// Updates the representative edge incident to the vertex in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E)
    where
        T::Edge: HalfEdge<T>,
        T::Vertex: HalfEdgeVertex<T>,
    {
        self.mesh.vertex_ref_mut(self.try_id()).set_edge(edge);
    }
}
