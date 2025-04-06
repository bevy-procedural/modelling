use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
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

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    fn into_immutable(self) -> ValidVertexCursor<'a, T> {
        ValidVertexCursor::load_new(self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursorData<'a, T> for VertexCursorMut<'a, T>
where
    T: 'a,
{
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

    #[inline]
    fn destructure(self) -> (&'a T::Mesh, Self::I) {
        (self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> CursorData for VertexCursorMut<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;
    type Payload = T::VP;
    type Maybe = Self;
    type Valid = ValidVertexCursorMut<'a, T>;

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
    fn load(self) -> Option<Self::Valid> {
        if self.is_void() {
            None
        } else {
            Some(ValidVertexCursorMut::new(self.mesh, self.try_id()))
        }
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        self
    }

    #[inline]
    fn from_maybe(from: Self::Maybe) -> Self {
        from
    }

    #[inline]
    fn from_valid(from: Self::Valid) -> Self {
        from.maybe()
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }
}

impl<'a, T: MeshType> MaybeCursor for VertexCursorMut<'a, T> {}

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
