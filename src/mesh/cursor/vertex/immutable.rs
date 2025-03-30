use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone)]
pub struct VertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> std::fmt::Debug for VertexCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexCursor({:?})", self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursor<'a, T> {
    /// Creates a new vertex cursor pointing to the given vertex.
    #[must_use]
    #[inline]
    pub fn new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self {
            mesh,
            vertex: IndexType::max(),
        }
    }
}

impl<'a, T: MeshType> ImmutableCursor for VertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursorData<'a, T> for VertexCursor<'a, T>
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

impl<'a, T: MeshType> CursorData for VertexCursor<'a, T>
where
    T: 'a,
{
    type I = T::V;
    type S = T::Vertex;
    type T = T;
    type Maybe = Self;
    type Valid = ValidVertexCursor<'a, T>;

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        if self.is_valid() {
            // PERF: avoid checking twice
            Some(ValidVertexCursor::load_new(self.mesh, self.vertex))
        } else {
            None
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
}

impl<'a, T: MeshType> MaybeCursor for VertexCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
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
