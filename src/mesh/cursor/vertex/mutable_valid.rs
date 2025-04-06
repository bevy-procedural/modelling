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

impl_debug_cursor!(ValidVertexCursorMut<'a, T: MeshType>, id: vertex);

#[rustfmt::skip]
impl_specific_cursor_data!(
    VertexCursorData, ValidVertexCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    FC, move_to_face, T::F, FaceCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   ValidCursor, ValidVertexCursorMut, VertexCursorMut,
   vertex, V, Vertex, VP, 
   get_vertex, has_vertex
);

impl<'a, T: MeshType> ValidCursor for ValidVertexCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.vertex
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.mesh.get_vertex(self.vertex).unwrap()
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.vertex_ref(self.try_id()).payload()
    }
}

impl<'a, T: MeshType> MutableCursor for ValidVertexCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> ValidCursorMut for ValidVertexCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload {
        self.mesh.vertex_ref_mut(self.try_id()).payload_mut()
    }

    #[inline]
    fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S {
        self.mesh.get_vertex_mut(self.vertex).unwrap()
    }
}

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

impl<'a, T: MeshType> ValidVertexCursorBasics<'a, T> for ValidVertexCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorBasics<'a, T> for ValidVertexCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> VertexCursorHalfedgeBasics<'a, T> for ValidVertexCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
    T: 'a,
{
}
