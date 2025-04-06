use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, MeshTypeHalfEdge},
};

/// An edge cursor pointing to an edge of a mesh with a mutable reference to the mesh.
pub struct EdgeCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
    // TODO: Integrate the path builder into the edge cursor mut! This should now include setting the start etc.
}

impl<'a, T: MeshType> EdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> EdgeCursor<'a, T> {
        EdgeCursor::new(&*self.mesh, self.edge)
    }
}

impl_debug_eq_cursor!(EdgeCursorMut, edge);

#[rustfmt::skip]
impl_specific_cursor_data!(
    EdgeCursorData, EdgeCursorMut,
    FC, move_to_face, T::F, FaceCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, MutableCursor, EdgeCursorMut, ValidEdgeCursorMut, 
   edge, new, E, Edge, EP, 
   get_edge, has_edge,
   MutableCursor, EdgeCursorBasics, EdgeCursorHalfedgeBasics
);

impl<'a, T: MeshType + 'a> EdgeCursorBuilder<'a, T> for EdgeCursorMut<'a, T> {}
impl<'a, T: MeshType + 'a> EdgeCursorHalfedgeBuilder<'a, T> for EdgeCursorMut<'a, T> where
    T: MeshTypeHalfEdge
{
}
