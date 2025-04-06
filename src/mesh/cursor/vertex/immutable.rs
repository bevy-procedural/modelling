use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone)]
pub struct VertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursor<'a, T> {
    /// Creates a new vertex cursor pointing to the given vertex.
    #[must_use]
    #[inline]
    pub fn new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Creates a new vertex cursor pointing nowhere (void).
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self {
            mesh,
            vertex: IndexType::max(),
        }
    }
}

impl_debug_cursor!(VertexCursor<'a, T: MeshType>, id: vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, VertexCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    FC, move_to_face, T::F, FaceCursor
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, VertexCursor, ValidVertexCursor,
   vertex, load_new, V, Vertex, VP, 
   get_vertex, has_vertex
);

impl<'a, T: MeshType> ImmutableCursor for VertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> ImmutableVertexCursor<'a, T> for VertexCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorBasics<'a, T> for VertexCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorHalfedgeBasics<'a, T> for VertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
    T: 'a,
{
}
