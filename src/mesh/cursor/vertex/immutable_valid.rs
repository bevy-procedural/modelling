use crate::mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone)]
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

impl_debug_cursor!(ValidVertexCursor<'a, T: MeshType>, id: vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, ValidVertexCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    FC, move_to_face, T::F, FaceCursor
);

#[rustfmt::skip]
impl_cursor_data!(
   ValidCursor, ValidVertexCursor, VertexCursor,
   vertex, V, Vertex, VP, 
   get_vertex, has_vertex
);

impl<'a, T: MeshType> ImmutableCursor for ValidVertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> ValidCursor for ValidVertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.vertex.id()
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.vertex
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.vertex.payload()
    }
}

impl<'a, T: MeshType> ValidVertexCursorBasics<'a, T> for ValidVertexCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> ImmutableVertexCursor<'a, T> for ValidVertexCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorBasics<'a, T> for ValidVertexCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorHalfedgeBasics<'a, T> for ValidVertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
    T: 'a,
{
}
