use crate::mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct ValidVertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: &'a T::Vertex,
}

impl<'a, T: MeshType> ValidVertexCursor<'a, T> {
    /// Creates a new vertex cursor pointing to the given vertex.
    #[must_use]
    #[inline]
    pub fn new(mesh: &'a T::Mesh, vertex: &'a T::Vertex) -> Self {
        Self { mesh, vertex }
    }

    /// Creates a new vertex cursor pointing to the given vertex.
    #[inline]
    #[must_use]
    pub fn load_new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self::new(mesh, mesh.vertex_ref(vertex))
    }
}

impl_debug_eq_cursor!(ValidVertexCursor, vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, ValidVertexCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    FC, move_to_face, T::F, FaceCursor
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, ImmutableCursor, ValidVertexCursor, VertexCursor,
    vertex, V, Vertex, VP, 
    get_vertex, get_vertex_mut, has_vertex,
    ImmutableVertexCursor, ValidVertexCursorBasics, VertexCursorBasics, VertexCursorHalfedgeBasics
);
