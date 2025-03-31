use crate::{
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdge, MeshBasics, MeshType},
};

/// An edge cursor pointing to an edge of a mesh with a mutable reference to the mesh.
pub struct EdgeCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
    // TODO: Integrate the path builder into the edge cursor mut! This should now include setting the start etc.
}

impl<'a, T: MeshType> std::fmt::Debug for EdgeCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeCursorMut({:?})", self.edge)
    }
}

impl<'a, T: MeshType> EdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> EdgeCursor<'a, T> {
        EdgeCursor::new(&*self.mesh, self.edge)
    }
}

impl<'a, T: MeshType> EdgeCursorData<'a, T> for EdgeCursorMut<'a, T>
where
    T: 'a,
{
    type VC = VertexCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for EdgeCursorMut<'a, T> {
    type I = T::E;
    type S = T::Edge;
    type T = T;
    type Payload = T::EP;
    type Maybe = Self;
    type Valid = ValidEdgeCursorMut<'a, T>;

    #[inline]
    fn try_id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> EdgeCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    fn load(self) -> Option<Self::Valid> {
        if self.is_void() {
            None
        } else {
            Some(ValidEdgeCursorMut::new(self.mesh, self.edge))
        }
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b T::Edge> {
        self.mesh().get_edge(self.try_id())
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
        self.try_id() == IndexType::max() || !self.mesh().has_edge(self.try_id())
    }
}

impl<'a, T: MeshType> MutableCursor for EdgeCursorMut<'a, T> {
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> MaybeCursor for EdgeCursorMut<'a, T> {}
impl<'a, T: MeshType> EdgeCursorBasics<'a, T> for EdgeCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> EdgeCursorBuilder<'a, T> for EdgeCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBuilder<'a, T> for EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
