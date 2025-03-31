use crate::{
    math::{HasPosition, IndexType, Scalar, Vector},
    mesh::{cursor::*, EdgeBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, VertexBasics},
};

pub trait ImmutableVertexCursor<'a, T: MeshType>:
    CursorData<T = T, I = T::V, S = T::Vertex> + VertexCursorData<'a, T>
where
    T: 'a,
{
    /// Returns an iterator of edge cursors pointing to the outgoing halfedges of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_edges_out] for more information.
    #[must_use]
    #[inline]
    fn edges_out(self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>> {
        self.mesh()
            .vertex_edges_out(self.try_id())
            .map(move |e| ValidEdgeCursor::load_new(self.mesh(), e))
    }

    /// Returns an iterator of edge ids pointing to the outgoing halfedges of the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_edges_out] for more information.
    #[must_use]
    #[inline]
    fn edges_out_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b
    where
        T: 'b,
    {
        self.mesh().vertex_edges_out(self.try_id())
    }

    /// Returns an iterator of edge cursors pointing to the incoming halfedges of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_edges_in] for more information.
    #[must_use]
    #[inline]
    fn edges_in(self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>> {
        self.mesh()
            .vertex_edges_in(self.try_id())
            .map(move |e| ValidEdgeCursor::load_new(self.mesh(), e))
    }

    /// Returns an iterator of edge ids pointing to the incoming halfedges of the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_edges_in] for more information.
    #[must_use]
    #[inline]
    fn edges_in_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b
    where
        T: 'b,
    {
        self.mesh().vertex_edges_in(self.try_id())
    }

    /// Iterates over the neighbors of the vertex.
    /// Panics if the vertex is void.
    /// See [VertexBasics::neighbor_ids] for more information.
    #[inline]
    #[must_use]
    fn neighbors(self) -> impl Iterator<Item = ValidVertexCursor<'a, T>> {
        self.mesh()
            .vertex_neighbors(self.try_id())
            .map(move |v| ValidVertexCursor::load_new(self.mesh(), v))
    }

    /// Iterates over the neighbors' ids of the vertex.
    /// Returns an empty iterator if the vertex is void.
    /// See [MeshBasics::vertex_neighbors] for more information.
    #[inline]
    #[must_use]
    fn neighbor_ids<'b>(&'b self) -> impl Iterator<Item = T::V> + 'b
    where
        T: 'b,
    {
        self.mesh().vertex_neighbors(self.try_id())
    }

    /// Iterates over the faces adjacent to the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_faces] for more information.
    #[inline]
    #[must_use]
    fn faces(self) -> impl Iterator<Item = ValidFaceCursor<'a, T>> {
        self.mesh()
            .vertex_faces(self.try_id())
            .map(move |f| ValidFaceCursor::load_new(self.mesh(), f))
    }

    /// Iterates over the ids of the faces adjacent to the vertex.
    /// Panics if the vertex is void.
    /// See [MeshBasics::vertex_faces] for more information.
    #[inline]
    #[must_use]
    fn face_ids<'b>(&'b self) -> impl Iterator<Item = T::F> + 'b
    where
        T: 'b,
    {
        self.mesh().vertex_faces(self.try_id())
    }
}

/// This trait implements some basic functionality for vertex cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait VertexCursorBasics<'a, T: MeshType>: VertexCursorData<'a, T> {
    /// Returns an edge cursor pointing to a representative edge incident to the vertex.
    #[inline]
    #[must_use]
    fn edge(self) -> Self::EC
    where
        Self::Valid: ValidVertexCursorBasics<'a, T> + VertexCursorData<'a, T, EC = Self::EC>,
    {
        self.load_or_else::<Self::EC, _, _>(
            |c| c.move_to_edge(IndexType::max()),
            |valid| {
                let id = valid.edge_id();
                valid.move_to_edge(id)
            },
        )
    }
}

/// This trait implements some basic functionality for vertex cursors that works with half edge meshes and both mutable and immutable cursors.
pub trait VertexCursorHalfedgeBasics<'a, T: MeshType>: VertexCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
    /*/// Returns an edge cursor pointing to an outgoing halfedge incident to the vertex.
    /// If the vertex is void, the edge cursor is void. Won't panic.
    #[inline]
    #[must_use]
    fn outgoing_edge(self) -> Self::EC {
        let edge = todo!();
        self.move_to_edge(edge)
    }*/

    /// Returns an ingoing boundary edge incident to the vertex.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn ingoing_boundary_edge(&self) -> Option<T::E> {
        // TODO: Weird signature
        HalfEdgeVertex::ingoing_boundary_edge(self.try_inner()?, self.mesh())
    }

    /// Returns an outgoing boundary edge incident to the vertex.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn outgoing_boundary_edge(&self) -> Option<T::E> {
        // TODO: Weird signature
        HalfEdgeVertex::outgoing_boundary_edge(self.try_inner()?, self.mesh())
    }
}
