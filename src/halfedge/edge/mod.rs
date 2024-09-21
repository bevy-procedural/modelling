mod iterator;

pub use iterator::*;

use super::{HalfEdgeMesh, HalfEdgeMeshType};
use crate::{
    math::IndexType,
    mesh::{DefaultEdgePayload, Edge, EdgePayload, MeshBasics},
    util::Deletable,
};

// TODO: Memory alignment?
// TODO: include a way to explicitly access faces around vertex/face? https://en.wikipedia.org/wiki/Polygon_mesh

/// Half-edge inspired data structure
#[derive(Clone, Copy, PartialEq)]
pub struct HalfEdge<T: HalfEdgeMeshType> {
    /// the index of the half-edge
    id: T::E,

    /// next half-edge incident to the same face
    /// (first edge encountered when traversing around the target vertex in clockwise order).
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    next: T::E,

    /// The other, opposite half-edge.
    /// This will always exist.
    twin: T::E,

    /// The previous half-edge incident to the same face.
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    prev: T::E,

    /// The source vertex of the half-edge.
    /// This will always exist.
    origin_id: T::V,

    /// The face the half-edge is incident to.
    /// The face lies to the left of the half-edge.
    /// Half-edges traverse the boundary of the face in counter-clockwise order.
    /// This index will be FaceIndex.max() if it doesn't exist, i.e., if the edge is a boundary.
    face: T::F,

    /// Some user-defined payload
    payload: T::EP,
}

impl<T: HalfEdgeMeshType> Edge for HalfEdge<T> {
    type T = T;
    
    /// Returns the index of the half-edge
    #[inline(always)]
    fn id(&self) -> T::E {
        self.id
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.origin_id)
    }

    /// Returns the target vertex of the half-edge. Reached via the next half-edge, not the twin.
    #[inline(always)]
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.next(mesh).origin_id())
    }

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    #[inline(always)]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.is_boundary_self() || self.twin(mesh).is_boundary_self()
    }
}

impl<T: HalfEdgeMeshType> HalfEdge<T> {
    // TODO: should the operations return a copy or a reference?

    /// Creates a new half-edge
    pub fn new(
        next: T::E,
        twin: T::E,
        prev: T::E,
        origin: T::V,
        face: T::F,
        payload: T::EP,
    ) -> Self {
        assert!(next != IndexType::max());
        assert!(prev != IndexType::max());
        assert!(twin != IndexType::max());
        Self {
            id: IndexType::max(),
            next,
            twin,
            prev,
            origin_id: origin,
            face,
            payload,
        }
    }

    /// Sets the face of the HalfEdge. Panics if the face is already set.
    pub fn set_face(&mut self, face: T::F) {
        debug_assert!(self.face == IndexType::max());
        self.face = face;
    }

    /// Deletes the face of the HalfEdge
    pub fn delete_face(&mut self) {
        debug_assert!(self.face != IndexType::max());
        self.face = IndexType::max();
    }

    /// Sets the next half-edge incident to the same face (including the holes)
    pub fn set_next(&mut self, next: T::E) {
        self.next = next;
    }

    /// Sets the previous half-edge incident to the same face (including the holes)
    pub fn set_prev(&mut self, prev: T::E) {
        self.prev = prev;
    }

    /// Sets the twin half-edge
    pub fn set_twin(&mut self, twin: T::E) {
        self.twin = twin;
    }

    /// Returns the next half-edge incident to the same face or boundary
    #[inline(always)]
    pub fn next(&self, mesh: &HalfEdgeMesh<T>) -> HalfEdge<T> {
        *mesh.edge(self.next)
    }

    /// Returns the next id
    #[inline(always)]
    pub fn next_id(&self) -> T::E {
        self.next
    }

    /// Returns the other, opposite half-edge
    #[inline(always)]
    pub fn twin(&self, mesh: &HalfEdgeMesh<T>) -> HalfEdge<T> {
        // TODO: Make this return a reference?
        *mesh.edge(self.twin)
    }

    /// Returns the twin id
    #[inline(always)]
    pub fn twin_id(&self) -> T::E {
        self.twin
    }

    /// Returns the previous half-edge incident to the same face or boundary
    #[inline(always)]
    pub fn prev(&self, mesh: &HalfEdgeMesh<T>) -> HalfEdge<T> {
        *mesh.edge(self.prev)
    }

    /// Returns the prev id
    #[inline(always)]
    pub fn prev_id(&self) -> T::E {
        self.prev
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    pub fn origin_id(&self) -> T::V {
        self.origin_id
    }

    /// Returns the target vertex id of the half-edge. Reached via the next half-edge, not the twin.
    #[inline(always)]
    pub fn target_id(&self, mesh: &HalfEdgeMesh<T>) -> T::V {
        self.next(mesh).origin_id()
    }

    /// Returns the face the half-edge is incident to
    #[inline(always)]
    pub fn face<'a>(&'a self, mesh: &'a HalfEdgeMesh<T>) -> Option<&'a T::Face> {
        if self.face == IndexType::max() {
            None
        } else {
            Some(mesh.face(self.face))
        }
    }

    /// Returns the face id
    #[inline(always)]
    pub fn face_id(&self) -> T::F {
        self.face
    }

    /// Returns the other face (incident to the twin)
    #[inline(always)]
    pub fn other_face<'a>(&'a self, mesh: &'a HalfEdgeMesh<T>) -> Option<&'a T::Face> {
        let face = self.twin(mesh).face_id();
        if face == IndexType::max() {
            None
        } else {
            Some(mesh.face(face))
        }
    }

    /// Returns whether the edge (i.e., this HalfEdge and not necessarily its twin) is a boundary edge
    #[inline(always)]
    pub fn is_boundary_self(&self) -> bool {
        self.face == IndexType::max()
    }

    /// Returns whether the edge can reach the vertex when searching counter-clockwise along the face
    pub fn same_face(&self, mesh: &HalfEdgeMesh<T>, v: T::V) -> bool {
        self.edges_face(mesh).find(|e| e.origin_id() == v).is_some()
    }

    /// Like `same_face` but searches clockwise
    pub fn same_face_back(&self, mesh: &HalfEdgeMesh<T>, v: T::V) -> bool {
        self.edges_face_back(mesh)
            .find(|e| e.origin_id() == v)
            .is_some()
    }

    /// Flips the direction of the edge and its twin
    pub fn flip(e: T::E, mesh: &mut HalfEdgeMesh<T>) {
        let origin = mesh.edge(e).origin_id();
        let target = mesh.edge(e).target_id(mesh);

        let twin_id = {
            let edge = mesh.edge_mut(e);
            let tmp = edge.next;
            edge.next = edge.prev;
            edge.prev = tmp;
            edge.origin_id = target;
            edge.twin_id()
        };
        {
            let twin = mesh.edge_mut(twin_id);
            let tmp = twin.next;
            twin.next = twin.prev;
            twin.prev = tmp;
            twin.origin_id = origin;
        }
        let o = mesh.vertex_mut(origin);
        o.set_edge(twin_id);
        let t = mesh.vertex_mut(target);
        t.set_edge(e);
    }
}

/*
impl<T: HalfEdgeMeshType> HalfEdge<T>
where
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Returns the center of the edge
    pub fn center(&self, mesh: &HalfEdgeMesh<T>) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * T::S::HALF
    }
}*/

impl<T: HalfEdgeMeshType> std::fmt::Debug for HalfEdge<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --{}--> ; twin: {}, face: {} [{}] {}",
            self.origin_id.index(),
            self.id().index(),
            self.twin.index(),
            self.prev.index(),
            if self.face == IndexType::max() {
                "none".to_string()
            } else {
                self.face.index().to_string()
            },
            self.next.index(),
        )?;
        if !self.payload.is_empty() {
            write!(f, ", payload: {:?}", self.payload)?;
        }
        Ok(())
    }
}

impl<T: HalfEdgeMeshType> Deletable<T::E> for HalfEdge<T> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max());
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: T::E) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        assert!(self.next != id);
        assert!(self.prev != id);
        self.id = id;
    }

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin_id: IndexType::max(),
            face: IndexType::max(),
            payload: T::EP::allocate(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdge<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Creates a deleted edge
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin_id: IndexType::max(),
            face: IndexType::max(),
            payload: T::EP::default(),
        }
    }
}
