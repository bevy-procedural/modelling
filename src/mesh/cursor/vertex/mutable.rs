use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
};

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
/// #[derive(DebugCursor)]
pub struct VertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    fn into_immutable(self) -> ValidVertexCursor<'a, T> {
        ValidVertexCursor::load_new(self.mesh, self.vertex)
    }
}

impl_debug_cursor!(VertexCursorMut<'a, T: MeshType>, id: vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, VertexCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    FC, move_to_face, T::F, FaceCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, VertexCursorMut, ValidVertexCursorMut, 
   vertex, new, V, Vertex, VP, 
   get_vertex, has_vertex
);

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
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

impl<'a, T: MeshType> VertexCursorBasics<'a, T> for VertexCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorHalfedgeBasics<'a, T> for VertexCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
    T: 'a,
{
}
