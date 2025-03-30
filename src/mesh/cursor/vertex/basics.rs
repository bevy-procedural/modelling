use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::{
        cursor::*, EdgeBasics, HalfEdge, HalfEdgeMesh, HalfEdgeVertex, MeshBasics, MeshTopology,
        MeshType, VertexBasics,
    },
};

/// This trait defines the basic functionality for accessing the data fields of a vertex cursor.
pub trait VertexCursorData<'a, T: MeshType>: CursorData<T = T, I = T::V, S = T::Vertex> {
    /// The associated face cursor type
    type FC: FaceCursorData<'a, T>;

    /// The associated edge cursor type
    type EC: EdgeCursorData<'a, T>;

    /// Derives a new face cursor pointing to the given face id.
    fn move_to_face(self, id: T::F) -> Self::FC;

    /// Derives a new edge cursor pointing to the given vertex id.
    fn move_to_edge(self, id: T::E) -> Self::EC;
}

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
    fn edge(self) -> Self::EC {
        let edge = self.unwrap().edge_id(self.mesh());
        self.move_to_edge(edge)
    }

    /// Returns the id of a representative edge incident to the vertex, `IndexType::max()` if it has none, or panic if the vertex is void.
    #[inline]
    #[must_use]
    fn edge_id(&self) -> T::E {
        self.unwrap().edge_id(self.mesh())
    }

    /// Whether the vertex is isolated.
    /// Panics if the vertex is void.
    /// See [VertexBasics::is_isolated] for more information.
    #[inline]
    #[must_use]
    fn is_isolated(&self) -> bool {
        self.unwrap().is_isolated(self.mesh())
    }

    /// Returns the vertex position.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn pos<S: Scalar, const D: usize, Vec: Vector<S, D>>(&self) -> Vec
    where
        T::VP: HasPosition<D, Vec, S = S>,
    {
        self.unwrap().pos()
    }

    /// Returns the vertex degree.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn degree(&self) -> usize {
        self.unwrap().degree(self.mesh())
    }

    /// Whether the vertex is manifold.
    /// See [VertexBasics::is_manifold] for more information.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn is_manifold(&self) -> bool {
        self.unwrap().is_manifold(self.mesh())
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
    /// Panics if the vertex is void.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn ingoing_boundary_edge(&self) -> Option<T::E> {
        HalfEdgeVertex::ingoing_boundary_edge(self.unwrap(), self.mesh())
    }

    /// Returns an outgoing boundary edge incident to the vertex.
    /// Panics if the vertex is void.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn outgoing_boundary_edge(&self) -> Option<T::E> {
        HalfEdgeVertex::outgoing_boundary_edge(self.unwrap(), self.mesh())
    }
}

pub trait ValidVertexCursorBasics<'a, T: MeshType>: VertexCursorData<'a, T> + ValidCursor {
    fn shortest_path(self, other: T::V) -> Option<(T::E, T::E, usize)>
    where
        T::Edge: HalfEdge<T>,
        Self::S: HalfEdgeVertex<T>,
    {
        self.inner().shortest_path(self.mesh(), other)
    }
}
