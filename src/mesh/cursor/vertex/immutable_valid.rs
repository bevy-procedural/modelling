use crate::mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone)]
pub struct ValidVertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: &'a T::Vertex,
}

impl<'a, T: MeshType> std::fmt::Debug for ValidVertexCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidVertexCursor({:?})", self.vertex)
    }
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

impl<'a, T: MeshType> ImmutableCursor for ValidVertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursorData<'a, T> for ValidVertexCursor<'a, T>
where
    T: 'a,
{
    type EC = EdgeCursor<'a, T>;
    type FC = FaceCursor<'a, T>;

    #[inline]
    fn move_to_face(self, id: T::F) -> Self::FC {
        FaceCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for ValidVertexCursor<'a, T>
where
    T: 'a,
{
    type I = T::V;
    type S = T::Vertex;
    type T = T;
    type Payload = T::VP;
    type Maybe = VertexCursor<'a, T>;
    type Valid = Self;

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex.id()
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        Some(self.vertex)
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        VertexCursor::new(self.mesh, self.vertex.id())
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
