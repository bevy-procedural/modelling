use super::{CursorData, EdgeCursorData, FaceCursorData};
use crate::mesh::{HalfEdge, HalfEdgeVertex, MeshType, VertexBasics};

/// This trait defines the basic functionality for accessing the data fields of a vertex cursor.
pub trait VertexCursorData<'a, T: MeshType + 'a>:
    CursorData<T = T, I = T::V, S = T::Vertex>
{
    /// The associated face cursor type
    type FC: FaceCursorData<'a, T>;

    /// The associated edge cursor type
    type EC: EdgeCursorData<'a, T>;

    /// Derives a new face cursor pointing to the given face id.
    fn move_to_face(self, id: T::F) -> Self::FC;

    /// Derives a new edge cursor pointing to the given vertex id.
    fn move_to_edge(self, id: T::E) -> Self::EC;
}

/// This trait implements some basic functionality for vertex cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait VertexCursorBasics<'a, T: MeshType + 'a>: VertexCursorData<'a, T> {
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
}

/// This trait implements some basic functionality for vertex cursors that works with half edge meshes and both mutable and immutable cursors.
pub trait VertexCursorHalfedgeBasics<'a, T: MeshType + 'a>: VertexCursorData<'a, T>
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
