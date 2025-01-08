use super::{
    CursorData, EdgeCursor, FaceCursor, VertexCursorBasics, VertexCursorData,
    VertexCursorHalfedgeBasics,
};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
};
use std::fmt::Debug;

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone, Debug)]
pub struct VertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursor<'a, T> {
    /// Creates a new vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Returns an iterator of edge cursors pointing to the outgoing halfedges of the vertex.
    /// Panics if the vertex is void.
    /// TODO: would be nice to return an empty iterator if the vertex is void instead?
    /// See [VertexBasics::edges_out] for more information.
    pub fn edges_out(&'a self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.unwrap()
            .edges_out(self.mesh)
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }

    /// Returns an iterator of edge cursors pointing to the incoming halfedges of the vertex.
    /// Panics if the vertex is void.
    /// TODO: would be nice to return an empty iterator if the vertex is void instead?
    /// See [VertexBasics::edges_in] for more information.
    pub fn edges_in(&'a self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.unwrap()
            .edges_in(self.mesh)
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }

    /// Iterates over the neighbors of the vertex.
    /// Panics if the vertex is void.
    /// See [VertexBasics::neighbors] for more information.
    #[inline]
    #[must_use]
    pub fn neighbors(&'a self) -> impl Iterator<Item = VertexCursor<'a, T>> {
        self.unwrap()
            .neighbor_ids(self.mesh())
            .map(move |v| VertexCursor::new(self.mesh, v))
    }

    /// Returns a reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &T::VP {
        self.unwrap().payload()
    }
}

impl<'a, T: MeshType + 'a> VertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    //
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursor<'a, T> {
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

impl<'a, T: MeshType + 'a> CursorData for VertexCursor<'a, T> {
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
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

impl<'a, T: MeshType + 'a> VertexCursorBasics<'a, T> for VertexCursor<'a, T> {}
impl<'a, T: MeshType + 'a> VertexCursorHalfedgeBasics<'a, T> for VertexCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
}
