use super::{
    CursorData, EdgeCursor, FaceCursor, VertexCursorBasics, VertexCursorData,
    VertexCursorHalfedgeBasics,
};
use crate::{
    math::IndexType,
    mesh::{HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
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

    /// Clones the cursor.
    #[inline]
    #[must_use]
    pub fn fork(&self) -> Self {
        Self::new(self.mesh, self.vertex)
    }

    /// Creates a new void vertex cursor.
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self::new(mesh, IndexType::max())
    }

    /// Returns an iterator of edge cursors pointing to the outgoing halfedges of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_edges_out] for more information.
    #[must_use]
    #[inline]
    pub fn edges_out(self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.mesh
            .vertex_edges_out(self.try_id())
            .map(move |e| EdgeCursor::new(self.mesh, e))
    }

    /// Returns an iterator of edge ids pointing to the outgoing halfedges of the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_edges_out] for more information.
    #[must_use]
    #[inline]
    pub fn edges_out_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b {
        self.mesh.vertex_edges_out(self.id())
    }

    /// Returns an iterator of edge cursors pointing to the incoming halfedges of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_edges_in] for more information.
    #[must_use]
    #[inline]
    pub fn edges_in(self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.mesh
            .vertex_edges_in(self.try_id())
            .map(move |e| EdgeCursor::new(self.mesh, e))
    }

    /// Returns an iterator of edge ids pointing to the incoming halfedges of the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_edges_in] for more information.
    #[must_use]
    #[inline]
    pub fn edges_in_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b {
        self.mesh.vertex_edges_in(self.id())
    }

    /// Iterates over the neighbors of the vertex.
    /// Panics if the vertex is void.
    /// See [VertexBasics::neighbor_ids] for more information.
    #[inline]
    #[must_use]
    pub fn neighbors(self) -> impl Iterator<Item = VertexCursor<'a, T>> {
        self.mesh
            .vertex_neighbors(self.id())
            .map(move |v| VertexCursor::new(self.mesh, v))
    }

    /// Iterates over the neighbors' ids of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_neighbors] for more information.
    #[inline]
    #[must_use]
    pub fn neighbor_ids<'b>(&'b self) -> impl Iterator<Item = T::V> + 'b {
        self.mesh.vertex_neighbors(self.try_id())
    }

    /// Iterates over the faces adjacent to the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_faces] for more information.
    #[inline]
    #[must_use]
    pub fn faces(self) -> impl Iterator<Item = FaceCursor<'a, T>> {
        self.mesh
            .vertex_faces(self.id())
            .map(move |f| FaceCursor::new(self.mesh, f))
    }

    /// Iterates over the ids of the faces adjacent to the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_faces] for more information.
    #[inline]
    #[must_use]
    pub fn face_ids<'b>(&'b self) -> impl Iterator<Item = T::F> + 'b {
        self.mesh.vertex_faces(self.id())
    }

    /// Returns a reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &T::VP {
        self.unwrap().payload()
    }
}

impl<'a, T: MeshType > VertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    //
}

impl<'a, T: MeshType > VertexCursorData<'a, T> for VertexCursor<'a, T> {
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

impl<'a, T: MeshType > CursorData for VertexCursor<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;

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
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }

    #[inline]
    fn inner<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

impl<'a, T: MeshType > VertexCursorBasics<'a, T> for VertexCursor<'a, T> {}
impl<'a, T: MeshType > VertexCursorHalfedgeBasics<'a, T> for VertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
}
