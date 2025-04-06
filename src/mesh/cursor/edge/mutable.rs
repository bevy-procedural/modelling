use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshType},
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

impl_debug_cursor!(EdgeCursorMut<'a, T: MeshType>, id: edge);

#[rustfmt::skip]
impl_specific_cursor_data!(
    EdgeCursorData, EdgeCursorMut,
    FC, move_to_face, T::F,FaceCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, EdgeCursorMut, ValidEdgeCursorMut, 
   edge, new, E, Edge, EP, 
   get_edge, has_edge
);

impl<'a, T: MeshType> MutableCursor for EdgeCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> EdgeCursorBasics<'a, T> for EdgeCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> EdgeCursorBuilder<'a, T> for EdgeCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBuilder<'a, T> for EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
