use super::{Deletable, HalfEdge, IndexType, Mesh};
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
    /// Will be IndexType::max() if the vertex is manifold.
    next: VertexIndex,

    /// the payload of the vertex
    payload: PayloadType,
}

impl<E: IndexType, V: IndexType, P: Payload> Vertex<E, V, P> {
    /// Creates a new vertex
    pub fn new(edge: E, payload: P) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            next: IndexType::max(),
            payload,
        }
    }

    /// Changes the representative of the outgoing edges 
    pub fn set_edge(&mut self, edge: E) {
        self.edge = edge;
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

    /// Returns the vertex coordinates of the payload
    #[inline(always)]
    pub fn vertex(&self) -> &P::Vec {
        self.payload.vertex()
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
        self.next == IndexType::max()
    }

    /// Returns whether the vertex has only one edge incident to it
    #[inline(always)]
    pub fn has_only_one_edge<F: IndexType>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        // self.edges(mesh).count() == 1
        let e = self.edge(mesh);
        e.prev_id() == e.twin_id()
    }

    /// Transforms the payload.
    #[inline(always)]
    pub fn transform(&mut self, transform: &P::Trans) {
        self.payload = self.payload.transform(transform);
    }

    /// Translates the payload.
    #[inline(always)]
    pub fn translate(&mut self, transform: &P::Vec) {
        self.payload = self.payload.translate(transform);
    }
}

impl<E: IndexType, V: IndexType, P: Payload> std::fmt::Display for Vertex<E, V, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}) --{}-->; payload: {}",
            self.id().index(),
            self.edge.index(),
            self.payload
        )
    }
}

impl<E: IndexType, V: IndexType, P: Payload> Deletable<V> for Vertex<E, V, P> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max());
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: V) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
    }
}

impl<E: IndexType, V: IndexType, P: Payload> Default for Vertex<E, V, P> {
    /// Creates a deleted vertex
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            next: IndexType::max(),
            payload: P::default(),
        }
    }
}
