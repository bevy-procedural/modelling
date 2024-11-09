mod iterator;

pub use iterator::*;

use super::HalfEdgeMeshType;
use crate::{
    math::IndexType,
    mesh::{
        DefaultVertexPayload, EdgeBasics, Halfedge, MeshBasics, MeshType, Vertex, VertexBasics,
        VertexPayload,
    },
    util::Deletable,
};
use itertools::Itertools;

/// A vertex in a mesh.
#[derive(Clone, PartialEq)]
pub struct HalfEdgeVertex<T: HalfEdgeMeshType> {
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

impl<T: HalfEdgeMeshType> HalfEdgeVertex<T> {
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

    /// Changes the representative of the outgoing edges
    pub fn set_edge(&mut self, edge: T::E) {
        self.edge = edge;
    }

    /// Returns an outgoing boundary edge incident to the vertex
    pub fn outgoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        // TODO: Assumes a manifold vertex. Otherwise, there can be multiple boundary edges!
        debug_assert!(
            self.edges_out(mesh)
                .filter(|e| e.is_boundary_self())
                .exactly_one()
                .is_ok(),
            "Vertex {} is not manifold",
            self.id()
        );

        self.edges_out(mesh).find_map(|e| {
            if e.is_boundary_self() {
                Some(e.id())
            } else {
                None
            }
        })
    }

    /// Returns an ingoing boundary edge incident to the vertex
    pub fn ingoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        debug_assert!(
            self.edges_in(mesh)
                .filter(|e| e.is_boundary_self())
                .exactly_one()
                .is_ok(),
            "Vertex {} is not manifold",
            self.id()
        );

        self.edges_in(mesh).find_map(|e| {
            if e.is_boundary_self() {
                Some(e.id())
            } else {
                None
            }
        })
    }
}

impl<T: HalfEdgeMeshType> VertexBasics<T> for HalfEdgeVertex<T> {
    /// Returns the index of the vertex
    #[inline(always)]
    fn id(&self) -> T::V {
        self.id
    }

    /// Returns the payload of the vertex
    #[inline(always)]
    fn payload(&self) -> &T::VP {
        &self.payload
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline(always)]
    fn payload_mut(&mut self) -> &mut T::VP {
        &mut self.payload
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline(always)]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.edges_out(mesh).any(|e| e.is_boundary(mesh))
    }

    /*
    /// Returns whether the vertex is manifold
    #[inline(always)]
    fn is_manifold(&self) -> bool {
        self.next == IndexType::max()
    }*/

    /// Returns whether the vertex has only one edge incident to it
    #[inline(always)]
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool {
        // self.edges(mesh).count() == 1
        let e = self.edge(mesh);
        e.prev_id() == e.twin_id()
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline(always)]
    fn edge(&self, mesh: &T::Mesh) -> T::Edge {
        *mesh.edge(self.edge)
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Vertex> + 'a {
        // TODO: slightly inefficient because of the clone and target being indirect
        self.edges_out(mesh).map(|e| e.target(mesh).clone())
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Face> + 'a
    where
        T: 'a,
    {
        self.edges_out(mesh).filter_map(|e| e.face(mesh).cloned())
    }
}

impl<T: HalfEdgeMeshType> Vertex for HalfEdgeVertex<T>
where
    T: MeshType<Vertex = HalfEdgeVertex<T>>,
{
    type T = T;
}

impl<T: HalfEdgeMeshType> std::fmt::Debug for HalfEdgeVertex<T> {
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

impl<T: HalfEdgeMeshType> Deletable<T::V> for HalfEdgeVertex<T> {
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

impl<T: HalfEdgeMeshType> Default for HalfEdgeVertex<T>
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
