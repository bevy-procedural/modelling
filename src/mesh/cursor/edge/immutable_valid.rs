use crate::mesh::{cursor::*, EdgeBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType};

/// A `ValidEdgeCursor` behaves the same as an `EdgeCursor` but is guaranteed to point to a existing non-deleted edge.
///
/// It is created by calling `load` on a `EdgeCursor`.
/// You can convert it back to a `EdgeCursor` by calling `into_maybe` or any other method that moves the cursor.
///
/// Unlike `EdgeCursor`, `ValidEdgeCursor` has accessors to retrieve the id of the edge, its payload, etc...
#[derive(Clone, Eq)]
pub struct ValidEdgeCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    edge: &'a T::Edge,
}

impl<'a, T: MeshType> ValidEdgeCursor<'a, T> {
    /// Creates a new edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, edge: &'a T::Edge) -> Self {
        Self { mesh, edge }
    }

    /// Creates a new edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn load_new(mesh: &'a T::Mesh, edge: T::E) -> Self {
        Self::new(mesh, mesh.edge_ref(edge))
    }

    /// Returns a reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &'a T::EP {
        self.mesh.edge_payload(self.id_unchecked())
    }
}

impl_debug_eq_cursor!(ValidEdgeCursor, edge);

#[rustfmt::skip]
impl_specific_cursor_data!(
    EdgeCursorData, ValidEdgeCursor,
    FC, move_to_face, T::F,FaceCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, ImmutableCursor, ValidEdgeCursor, EdgeCursor, 
    edge, E, Edge, EP, 
    get_edge, get_edge_mut, has_edge,
    ImmutableEdgeCursor, ValidEdgeCursorBasics, EdgeCursorBasics, EdgeCursorHalfedgeBasics
);

impl<'a, T: MeshType + 'a> ValidEdgeCursorHalfedgeBasics<'a, T> for ValidEdgeCursor<'a, T> where
    T::Edge: HalfEdge<T>
{
}
