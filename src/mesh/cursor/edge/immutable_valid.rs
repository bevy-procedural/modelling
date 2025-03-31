use crate::mesh::{cursor::*, EdgeBasics, HalfEdge, MeshBasics, MeshType};

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

impl<'a, T: MeshType> std::fmt::Debug for ValidEdgeCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidEdgeCursor({:?})", self.try_id())
    }
}

impl<'a, T: MeshType> PartialEq for ValidEdgeCursor<'a, T> {
    /// same edge id and pointing to the same mesh instance
    fn eq(&self, other: &Self) -> bool {
        self.edge.id() == other.edge.id() && std::ptr::eq(self.mesh, other.mesh)
    }
}

impl<'a, T: MeshType> ImmutableCursor for ValidEdgeCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.edge)
    }
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
        self.mesh.edge_payload(self.try_id())
    }
}

impl<'a, T: MeshType> EdgeCursorData<'a, T> for ValidEdgeCursor<'a, T>
where
    T: 'a,
{
    type VC = VertexCursor<'a, T>;
    type FC = FaceCursor<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for ValidEdgeCursor<'a, T>
where
    T: 'a,
{
    type I = T::E;
    type S = T::Edge;
    type T = T;
    type Payload = T::EP;
    type Maybe = EdgeCursor<'a, T>;
    type Valid = Self;

    #[inline]
    fn try_id(&self) -> T::E {
        self.edge.id()
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> Self::Maybe {
        EdgeCursor::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        Some(self.edge)
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        EdgeCursor::new(self.mesh, self.edge.id())
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

impl<'a, T: MeshType> ValidCursor for ValidEdgeCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.edge.id()
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.edge
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.edge_payload(self.edge.id())
    }
}

impl<'a, T: MeshType> ValidEdgeCursorHalfedgeBasics<'a, T> for ValidEdgeCursor<'a, T>
where
    T: 'a,
    T::Edge: HalfEdge<T>,
{
}
impl<'a, T: MeshType> ValidEdgeCursorBasics<'a, T> for ValidEdgeCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> ImmutableEdgeCursor<'a, T> for ValidEdgeCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorBasics<'a, T> for ValidEdgeCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBasics<'a, T> for ValidEdgeCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
