mod iterator;
pub use iterator::*;
use itertools::Itertools;

use super::{Deletable, HalfEdge, IndexType, Mesh, MeshType};
use crate::math::Vector;
use payload::{DefaultVertexPayload, HasPosition, Transformable, VertexPayload};

/// A vertex in a mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex<E: IndexType, V: IndexType, VP: VertexPayload> {
    /// the index of the vertex
    id: V,

    /// An outgoing half-edge incident to the vertex.
    edge: E,

    /*
    /// Since we support non-manifold vertices, there can be a "wheel" of vertices,
    /// each connected to its own "wheel" of manifold edges.
    /// Will be IndexType::max() if the vertex is manifold.
    /// TODO: This is only necessary for non-manifold vertices where there are multiple next-prev wheels. But even with one wheel, this can be non-manifold if the vertex is singular.
    next: V,
    */
    /// the payload of the vertex
    payload: VP,
}

impl<E: IndexType, V: IndexType, VP: VertexPayload> Vertex<E, V, VP> {
    /// Creates a new vertex
    pub fn new(edge: E, payload: VP) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            //next: IndexType::max(),
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
    pub fn payload(&self) -> &VP {
        &self.payload
    }

    /// Returns the vertex coordinates of the payload
    #[inline(always)]
    pub fn pos<Vec: Vector<VP::S>>(&self) -> &Vec
    where
        VP: HasPosition<Vec>,
    {
        self.payload.pos()
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline(always)]
    pub fn payload_mut(&mut self) -> &mut VP {
        &mut self.payload
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline(always)]
    pub fn edge<T: MeshType<E = E, V = V, VP = VP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> HalfEdge<E, V, T::F, T::EP> {
        *mesh.edge(self.edge)
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline(always)]
    pub fn is_boundary<T: MeshType<E = E, V = V, VP = VP>>(&self, mesh: &Mesh<T>) -> bool {
        self.edges_out(mesh).any(|e| e.is_boundary(mesh))
    }

    /*
    /// Returns whether the vertex is manifold
    #[inline(always)]
    pub fn is_manifold(&self) -> bool {
        self.next == IndexType::max()
    }*/

    /// Returns whether the vertex has only one edge incident to it
    #[inline(always)]
    pub fn has_only_one_edge<T: MeshType<E = E, V = V, VP = VP>>(&self, mesh: &Mesh<T>) -> bool {
        // self.edges(mesh).count() == 1
        let e = self.edge(mesh);
        e.prev_id() == e.twin_id()
    }

    /// Returns an outgoing boundary edge incident to the vertex
    pub fn outgoing_boundary_edge<T: MeshType<E = E, V = V, VP = VP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> Option<E> {
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
    pub fn ingoing_boundary_edge<T: MeshType<E = E, V = V, VP = VP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> Option<E> {
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

impl<E: IndexType, V: IndexType, VP: VertexPayload + Transformable> Vertex<E, V, VP> {
    /// Transforms the payload.
    #[inline(always)]
    pub fn transform(&mut self, transform: &VP::Trans) {
        self.payload = self.payload.transform(transform);
    }

    /// Translates the payload.
    #[inline(always)]
    pub fn translate(&mut self, transform: &VP::Vec) {
        self.payload = self.payload.translate(transform);
    }

    /// Rotates the payload.
    #[inline(always)]
    pub fn rotate(&mut self, transform: &VP::Rot) {
        self.payload = self.payload.rotate(transform);
    }

    /// Scales the payload.
    #[inline(always)]
    pub fn scale(&mut self, transform: &VP::Vec) {
        self.payload = self.payload.scale(transform);
    }
}

impl<E: IndexType, V: IndexType, VP: VertexPayload> std::fmt::Display for Vertex<E, V, VP> {
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

impl<E: IndexType, V: IndexType, VP: VertexPayload> Deletable<V> for Vertex<E, V, VP> {
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

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            payload: VP::allocate(),
        }
    }
}

impl<E: IndexType, V: IndexType, VP: VertexPayload + DefaultVertexPayload> Default
    for Vertex<E, V, VP>
{
    /// Creates a deleted vertex
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            //next: IndexType::max(),
            payload: VP::default(),
        }
    }
}