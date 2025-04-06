use crate::mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics};

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
pub struct ValidVertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> std::fmt::Debug for ValidVertexCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidVertexCursorMut({:?})", self.vertex)
    }
}

impl<'a, T: MeshType> ValidVertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }
}

impl<'a, T: MeshType> VertexCursorData<'a, T> for ValidVertexCursorMut<'a, T>
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

impl<'a, T: MeshType> CursorData for ValidVertexCursorMut<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;
    type Payload = T::VP;
    type Maybe = VertexCursorMut<'a, T>;
    type Valid = Self;

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
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        self.mesh.get_vertex(self.vertex)
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        VertexCursorMut::new(self.mesh, self.vertex)
    }

    #[inline]
    fn from_maybe(from: Self::Maybe) -> Self {
        from.load().unwrap()
    }

    #[inline]
    fn from_valid(from: Self::Valid) -> Self {
        from
    }

    #[inline]
    fn is_void(&self) -> bool {
        false
    }
}

impl<'a, T: MeshType> ValidCursor for ValidVertexCursorMut<'a, T> {
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

impl<'a, T: MeshType> MutableCursor for ValidVertexCursorMut<'a, T> {
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> ValidCursorMut for ValidVertexCursorMut<'a, T> {
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
