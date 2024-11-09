mod basics;
mod iterator;

pub use iterator::*;

use super::HalfEdgeMeshType;
use crate::{
    math::IndexType,
    mesh::{DefaultVertexPayload, HalfEdgeVertex, MeshType, Vertex, VertexBasics, VertexPayload},
    util::Deletable,
};

/// A vertex in a mesh.
#[derive(Clone, PartialEq)]
pub struct HalfEdgeVertexImpl<T: HalfEdgeMeshType> {
    /// the index of the vertex
    id: T::V,

    /// An outgoing half-edge incident to the vertex.
    edge: T::E,

    /*
    /// Since we support non-manifold vertices, there can be a "wheel" of vertices,
    /// each connected to its own "wheel" of manifold edges.
    /// Will be IndexType::max() if the vertex is manifold.
    /// TODO: This is only necessary for non-manifold vertices where there are multiple next-prev wheels. But even with one wheel, this can be non-manifold if the vertex is singular.
    next: V,
    */
    /// the payload of the vertex
    payload: T::VP,
}

impl<T: HalfEdgeMeshType> HalfEdgeVertexImpl<T> {
    /// Creates a new vertex
    pub fn new(edge: T::E, payload: T::VP) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            //next: IndexType::max(),
            payload,
        }
    }
}

impl<T: HalfEdgeMeshType> HalfEdgeVertex<T> for HalfEdgeVertexImpl<T> {
    fn set_edge(&mut self, edge: T::E) {
        self.edge = edge;
    }
}

impl<T: HalfEdgeMeshType> Vertex for HalfEdgeVertexImpl<T>
where
    T: MeshType<Vertex = HalfEdgeVertexImpl<T>>,
{
    type T = T;
}

impl<T: HalfEdgeMeshType> std::fmt::Debug for HalfEdgeVertexImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{: >w$}) -{:-^w$}->; payload: {:?}",
            self.id().index(),
            self.edge.index(),
            self.payload,
            w = 3
        )
    }
}

impl<T: HalfEdgeMeshType> Deletable<T::V> for HalfEdgeVertexImpl<T> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max());
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: T::V) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
    }

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            payload: T::VP::allocate(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeVertexImpl<T>
where
    T::VP: DefaultVertexPayload,
{
    /// Creates a deleted vertex
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            //next: IndexType::max(),
            payload: T::VP::default(),
        }
    }
}
