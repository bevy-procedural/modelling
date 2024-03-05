use super::{payload::Payload, Deletable, Face, IndexType, Mesh, Vertex};
mod iterator;
pub use iterator::*;

/// Half-edge inspired data structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalfEdge<EdgeIndex, VertexIndex, FaceIndex>
where
    EdgeIndex: IndexType,
    VertexIndex: IndexType,
    FaceIndex: IndexType,
{
    /// the index of the half-edge
    id: EdgeIndex,

    /// next half-edge incident to the same face
    /// (first edge encountered when traversing around the target vertex in clockwise order).
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    next: EdgeIndex,

    /// The other, opposite half-edge.
    /// This will always exist.
    twin: EdgeIndex,

    /// The previous half-edge incident to the same face.
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    prev: EdgeIndex,

    /// The source vertex of the half-edge.
    /// This will always exist.
    origin: VertexIndex,

    /// The face the half-edge is incident to.
    /// The face lies to the left of the half-edge.
    /// Half-edges traverse the boundary of the face in counter-clockwise order.
    /// This index will be FaceIndex.max() if it doesn't exist, i.e., if the edge is a boundary.
    face: FaceIndex,
    // TODO: Memory alignment?
    // TODO: include payload?
    // TODO: include a reference to the mesh?
    // TODO: include a way to explicitly access faces around vertex/face? https://en.wikipedia.org/wiki/Polygon_mesh
}

impl<E: IndexType, V: IndexType, F: IndexType> HalfEdge<E, V, F> {
    // TODO: should the operations return a copy or a reference?

    /// Creates a new half-edge
    pub fn new(next: E, twin: E, prev: E, origin: V, face: F) -> Self {
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
        }
    }

    /// Sets the face of the HalfEdge. Panics if the face is already set.
    pub fn set_face(&mut self, face: F) {
        assert!(self.face == IndexType::max());
        self.face = face;
    }

    /// Deletes the face of the HalfEdge
    pub fn delete_face(&mut self) {
        assert!(self.face != IndexType::max());
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
    pub fn next<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.next)
    }

    /// Returns the next id
    #[inline(always)]
    pub fn next_id(&self) -> E {
        self.next
    }

    /// Returns the other, opposite half-edge
    #[inline(always)]
    pub fn twin<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.twin)
    }

    /// Returns the twin id
    #[inline(always)]
    pub fn twin_id(&self) -> E {
        self.twin
    }

    /// Returns the previous half-edge incident to the same face or boundary
    #[inline(always)]
    pub fn prev<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.prev)
    }

    /// Returns the prev id
    #[inline(always)]
    pub fn prev_id(&self) -> E {
        self.prev
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    pub fn origin<'a, P: Payload>(&'a self, mesh: &'a Mesh<E, V, F, P>) -> &Vertex<E, V, P> {
        mesh.vertex(self.origin)
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    pub fn origin_id(&self) -> V {
        self.origin
    }

    /// Returns the target vertex of the half-edge
    #[inline(always)]
    pub fn target<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> Vertex<E, V, P> {
        // TODO: avoid this clone?
        self.twin(mesh).origin(mesh).clone()
    }

    /// Returns the target vertex id of the half-edge
    #[inline(always)]
    pub fn target_id<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> V {
        self.twin(mesh).origin_id()
    }

    /// Returns the face the half-edge is incident to
    #[inline(always)]
    pub fn face<'a, P: Payload>(&'a self, mesh: &'a Mesh<E, V, F, P>) -> Option<Face<E, F>> {
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
    pub fn other_face<'a, P: Payload>(&'a self, mesh: &'a Mesh<E, V, F, P>) -> Option<Face<E, F>> {
        self.twin(mesh).face(mesh)
    }

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    #[inline(always)]
    pub fn is_boundary<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        self.is_boundary_self() || self.twin(mesh).is_boundary_self()
    }

    /// Returns whether the edge (i.e., this HalfEdge and not necessarily its twin) is a boundary edge
    #[inline(always)]
    pub fn is_boundary_self(&self) -> bool {
        self.face == IndexType::max()
    }

    /// Returns whether the edge can reach the vertex (searching counter-clockwise)
    pub fn can_reach<P: Payload>(&self, mesh: &Mesh<E, V, F, P>, v: V) -> bool {
        self.edges_face(mesh).find(|e| e.origin_id() == v).is_some()
    }

    /// Returns whether the edge can reach the vertex (searching clockwise)
    pub fn can_reach_back<P: Payload>(&self, mesh: &Mesh<E, V, F, P>, v: V) -> bool {
        self.edges_face_back(mesh)
            .find(|e| e.origin_id() == v)
            .is_some()
    }

    /// Returns the center of the edge
    pub fn center<P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec {
        let v1 = self.origin(mesh).vertex().clone();
        let v2 = self.target(mesh).vertex().clone();
        (v1 + v2) * P::S::from(0.5)
    }
}

impl<E: IndexType, V: IndexType, F: IndexType> std::fmt::Display for HalfEdge<E, V, F> {
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

impl<E: IndexType, V: IndexType, F: IndexType> Deletable<E> for HalfEdge<E, V, F> {
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
}

impl<E: IndexType, V: IndexType, F: IndexType> Default for HalfEdge<E, V, F> {
    /// Creates a deleted edge
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin: IndexType::max(),
            face: IndexType::max(),
        }
    }
}
