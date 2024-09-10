mod iterator;
mod payload;

use super::{Deletable, Face, IndexType, Mesh, MeshType, Vertex};
pub use iterator::*;
pub use payload::*;

// TODO: Memory alignment?
// TODO: include a way to explicitly access faces around vertex/face? https://en.wikipedia.org/wiki/Polygon_mesh

/// Half-edge inspired data structure
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HalfEdge<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> {
    /// the index of the half-edge
    id: E,

    /// next half-edge incident to the same face
    /// (first edge encountered when traversing around the target vertex in clockwise order).
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    next: E,

    /// The other, opposite half-edge.
    /// This will always exist.
    twin: E,

    /// The previous half-edge incident to the same face.
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    prev: E,

    /// The source vertex of the half-edge.
    /// This will always exist.
    origin: V,

    /// The face the half-edge is incident to.
    /// The face lies to the left of the half-edge.
    /// Half-edges traverse the boundary of the face in counter-clockwise order.
    /// This index will be FaceIndex.max() if it doesn't exist, i.e., if the edge is a boundary.
    face: F,

    /// Some user-defined payload
    payload: EP,
}

impl<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> HalfEdge<E, V, F, EP> {
    // TODO: should the operations return a copy or a reference?

    /// Creates a new half-edge
    pub fn new(next: E, twin: E, prev: E, origin: V, face: F, payload: EP) -> Self {
        assert!(next != IndexType::max());
        assert!(prev != IndexType::max());
        assert!(twin != IndexType::max());
        Self {
            id: IndexType::max(),
            next,
            twin,
            prev,
            origin,
            face,
            payload,
        }
    }

    /// Sets the face of the HalfEdge. Panics if the face is already set.
    pub fn set_face(&mut self, face: F) {
        debug_assert!(self.face == IndexType::max());
        self.face = face;
    }

    /// Deletes the face of the HalfEdge
    pub fn delete_face(&mut self) {
        debug_assert!(self.face != IndexType::max());
        self.face = IndexType::max();
    }

    /// Sets the next half-edge incident to the same face (including the holes)
    pub fn set_next(&mut self, next: E) {
        self.next = next;
    }

    /// Sets the previous half-edge incident to the same face (including the holes)
    pub fn set_prev(&mut self, prev: E) {
        self.prev = prev;
    }

    /// Returns the index of the half-edge
    #[inline(always)]
    pub fn id(&self) -> E {
        self.id
    }

    /// Returns the next half-edge incident to the same face or boundary
    #[inline(always)]
    pub fn next<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> HalfEdge<E, V, F, EP> {
        *mesh.edge(self.next)
    }

    /// Returns the next id
    #[inline(always)]
    pub fn next_id(&self) -> E {
        self.next
    }

    /// Returns the other, opposite half-edge
    #[inline(always)]
    pub fn twin<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> HalfEdge<E, V, F, EP> {
        *mesh.edge(self.twin)
    }

    /// Returns the twin id
    #[inline(always)]
    pub fn twin_id(&self) -> E {
        self.twin
    }

    /// Returns the previous half-edge incident to the same face or boundary
    #[inline(always)]
    pub fn prev<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> HalfEdge<E, V, F, EP> {
        *mesh.edge(self.prev)
    }

    /// Returns the prev id
    #[inline(always)]
    pub fn prev_id(&self) -> E {
        self.prev
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    pub fn origin<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> &'a Vertex<E, V, T::VP> {
        mesh.vertex(self.origin)
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    pub fn origin_id(&self) -> V {
        self.origin
    }

    /// Returns the target vertex of the half-edge
    #[inline(always)]
    pub fn target<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
    ) -> Vertex<E, V, T::VP> {
        // TODO: avoid this clone?
        self.twin(mesh).origin(mesh).clone()
    }

    /// Returns the target vertex id of the half-edge
    #[inline(always)]
    pub fn target_id<T: MeshType<E = E, V = V, F = F, EP = EP>>(&self, mesh: &Mesh<T>) -> V {
        self.twin(mesh).origin_id()
    }

    /// Returns the face the half-edge is incident to
    #[inline(always)]
    pub fn face<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> Option<Face<E, F, T::FP>> {
        if self.face == IndexType::max() {
            None
        } else {
            Some(*mesh.face(self.face))
        }
    }

    /// Returns the face id
    #[inline(always)]
    pub fn face_id(&self) -> F {
        self.face
    }

    /// Returns the other face (incident to the twin)
    #[inline(always)]
    pub fn other_face<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> Option<Face<E, F, T::FP>> {
        self.twin(mesh).face(mesh)
    }

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    #[inline(always)]
    pub fn is_boundary<T: MeshType<E = E, V = V, F = F, EP = EP>>(&self, mesh: &Mesh<T>) -> bool {
        self.is_boundary_self() || self.twin(mesh).is_boundary_self()
    }

    /// Returns whether the edge (i.e., this HalfEdge and not necessarily its twin) is a boundary edge
    #[inline(always)]
    pub fn is_boundary_self(&self) -> bool {
        self.face == IndexType::max()
    }

    /// Returns whether the edge can reach the vertex (searching counter-clockwise)
    pub fn can_reach<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
        v: V,
    ) -> bool {
        self.edges_face(mesh).find(|e| e.origin_id() == v).is_some()
    }

    /// Returns whether the edge can reach the vertex (searching clockwise)
    pub fn can_reach_back<T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &self,
        mesh: &Mesh<T>,
        v: V,
    ) -> bool {
        self.edges_face_back(mesh)
            .find(|e| e.origin_id() == v)
            .is_some()
    }

    /// Returns the center of the edge
    pub fn center<T: MeshType<E = E, V = V, F = F, EP = EP>>(&self, mesh: &Mesh<T>) -> T::Vec {
        let v1 = self.origin(mesh).vertex().clone();
        let v2 = self.target(mesh).vertex().clone();
        (v1 + v2) * T::S::from(0.5)
    }

    /// Flips the direction of the edge and its twin
    pub fn flip<T: MeshType<E = E, V = V, F = F, EP = EP>>(e: E, mesh: &mut Mesh<T>) {
        let origin = mesh.edge(e).origin_id();
        let target = mesh.edge(e).target_id(mesh);

        let twin_id = {
            let edge = mesh.edge_mut(e);
            let tmp = edge.next;
            edge.next = edge.prev;
            edge.prev = tmp;
            edge.origin = target;
            edge.twin_id()
        };
        {
            let twin = mesh.edge_mut(twin_id);
            let tmp = twin.next;
            twin.next = twin.prev;
            twin.prev = tmp;
            twin.origin = origin;
        }
        let o = mesh.vertex_mut(origin);
        o.set_edge(twin_id);
        let t = mesh.vertex_mut(target);
        t.set_edge(e);
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> std::fmt::Display
    for HalfEdge<E, V, F, EP>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --{}-->; twin: {}, face: {} [{}] {}",
            self.origin.index(),
            self.id().index(),
            self.twin.index(),
            self.prev.index(),
            if self.face == IndexType::max() {
                "none".to_string()
            } else {
                self.face.index().to_string()
            },
            self.next.index(),
        )
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> Deletable<E>
    for HalfEdge<E, V, F, EP>
{
    fn delete(&mut self) {
        assert!(self.id != IndexType::max());
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: E) {
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
            origin: IndexType::max(),
            face: IndexType::max(),
            payload: EP::allocate(),
        }
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> Default for HalfEdge<E, V, F, EP>
where
    EP: DefaultEdgePayload,
{
    /// Creates a deleted edge
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin: IndexType::max(),
            face: IndexType::max(),
            payload: EP::default(),
        }
    }
}
