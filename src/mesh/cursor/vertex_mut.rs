use super::{
    CursorData, EdgeCursorMut, FaceCursorMut, VertexCursorBasics, VertexCursorData,
    VertexCursorHalfedgeBasics,
};
use crate::{
    math::IndexType,
    mesh::{HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
};

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
pub struct VertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> std::fmt::Debug for VertexCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexCursorMut({:?})", self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Creates a new void vertex cursor.
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a mut T::Mesh) -> Self {
        Self::new(mesh, IndexType::max())
    }

    /// Returns a mutable reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::VP {
        VertexBasics::payload_mut(self.mesh.vertex_ref_mut(self.try_id()))
    }
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursorMut<'a, T> {
    type EC = EdgeCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for VertexCursorMut<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

impl<'a, T: MeshType + 'a> VertexCursorMut<'a, T> {
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

impl<'a, T: MeshType + 'a> VertexCursorBasics<'a, T> for VertexCursorMut<'a, T> {}
impl<'a, T: MeshType + 'a> VertexCursorHalfedgeBasics<'a, T> for VertexCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
}
