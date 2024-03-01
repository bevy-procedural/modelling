use super::{HalfEdge, IndexType, Mesh};
use payload::Payload;
mod iterator;
pub use iterator::*;
pub mod payload;

/// A vertex in a mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex<EdgeIndex, VertexIndex, PayloadType>
where
    EdgeIndex: IndexType,
    VertexIndex: IndexType,
    PayloadType: Payload,
{
    /// the index of the vertex
    id: VertexIndex,

    /// An outgoing half-edge incident to the vertex.
    edge: EdgeIndex,

    /// Since we support non-manifold vertices, there can be a "wheel" of vertices,
    /// each connected to its own "wheel" of manifold edges.
    next: VertexIndex,

    /// the payload of the vertex
    payload: PayloadType,
}

impl<E: IndexType, V: IndexType, P: Payload> Vertex<E, V, P> {
    /// Creates a new vertex
    pub fn new(id: V, edge: E, next: V, payload: P) -> Self {
        assert!(
            next != IndexType::max(),
            "next must be id if the vertex is manifold"
        );
        assert!(id != IndexType::max());
        assert!(edge != IndexType::max());
        Self {
            id,
            edge,
            next,
            payload,
        }
    }

    /// Returns the index of the vertex
    #[inline(always)]
    pub fn id(&self) -> V {
        self.id
    }

    /// Returns the payload of the vertex
    #[inline(always)]
    pub fn payload(&self) -> &P {
        &self.payload
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline(always)]
    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline(always)]
    pub fn edge<F: IndexType>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.edge)
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline(always)]
    pub fn is_boundary<F: IndexType>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        self.edges(mesh).any(|e| e.is_boundary(mesh))
    }

    /// Returns whether the vertex is manifold
    #[inline(always)]
    pub fn is_manifold(&self) -> bool {
        self.next == self.id
    }

    /// Returns whether the vertex has only one edge incident to it
    #[inline(always)]
    pub fn has_only_one_edge<F: IndexType>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        // self.edges(mesh).count() == 1
        let e = self.edge(mesh);
        e.prev_id() == e.twin_id()
    }
}

impl<E: IndexType, V: IndexType, P: Payload> std::fmt::Display for Vertex<E, V, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}) --{}-->; payload: {:?}",
            self.id().index(),
            self.edge.index(),
            self.payload
        )
    }
}
