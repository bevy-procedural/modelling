use crate::mesh::{cursor::*, HalfEdge, MeshBasics, MeshType};

/// A `ValidEdgeCursorMut` behaves the same as an `EdgeCursorMut` but is guaranteed to point to a existing non-deleted edge.
///
/// It is created by calling `load` on a `EdgeCursorMut`.
/// You can convert it back to a `EdgeCursorMut` by calling `into_maybe` or any other method that moves the cursor.
///
/// Unlike `EdgeCursorMut`, `ValidEdgeCursorMut` has accessors to retrieve and set the id of the edge, its payload, etc...
pub struct ValidEdgeCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> std::fmt::Debug for ValidEdgeCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidEdgeCursorMut({:?})", self.edge)
    }
}

impl<'a, T: MeshType> ValidEdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> ValidEdgeCursor<'a, T> {
        ValidEdgeCursor::new(&*self.mesh, self.mesh.get_edge(self.edge).unwrap())
    }
}

impl<'a, T: MeshType> EdgeCursorData<'a, T> for ValidEdgeCursorMut<'a, T> {
    type VC = VertexCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for ValidEdgeCursorMut<'a, T> {
    type I = T::E;
    type S = T::Edge;
    type T = T;
    type Maybe = EdgeCursorMut<'a, T>;
    type Valid = Self;

    #[inline]
    fn try_id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> Self::Maybe {
        EdgeCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        self.mesh.get_edge(self.edge)
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        EdgeCursorMut::new(self.mesh, self.edge)
    }
}

impl<'a, T: MeshType> ValidCursor for ValidEdgeCursorMut<'a, T> {
    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.mesh.get_edge(self.edge).unwrap()
    }
}

impl<'a, T: MeshType> ValidEdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    /// Returns a mutable reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::EP {
        self.mesh.edge_payload_mut(self.edge)
    }

    /// Runs the closure on all outgoing halfedges of the target.
    /// Panics if one of the outgoing halfedges doesn't have a twin.
    pub fn for_each_next<F: Fn(Self) -> Self>(self, f: F) -> Self {
        let twin = self.twin();
        let id = twin.try_id();
        let mut c = twin.next_sibling();
        while c.try_id() != id {
            let c_id = c.try_id();
            // execute closure, reset to the original edge and continue with the next sibling
            c = f(c.load().unwrap()).move_to(c_id).next_sibling();
        }

        assert!(c.try_id() == id, "Invalid edge cursor: {}", c.try_id());
        c.load()
            .expect("The original edge disappeared during the iteration")
    }

    /// Sets the next halfedge of the edge in the mesh.
    /// Also sets the previous halfedge of the given next edge to be the current edge.
    pub fn link(self, next: T::E) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_next(next);
        self.mesh.edge_ref_mut(next).set_prev(self.edge);
        self
    }

    /// Sets the next halfedge of the edge in the mesh.
    pub fn set_next(self, next: T::E) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_next(next);
        self
    }

    /// Sets the previous halfedge of the edge in the mesh.
    pub fn set_prev(self, prev: T::E) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_prev(prev);
        self
    }

    /// Sets the twin halfedge of the edge in the mesh.
    pub fn set_twin(self, twin: T::E) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_twin(twin);
        self
    }

    /// Sets the face of the edge in the mesh.
    pub fn set_face(self, face: T::F) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_face(face);
        self
    }

    /// Sets the origin vertex of the edge in the mesh.
    pub fn set_origin(self, origin: T::V) -> Self {
        self.mesh.edge_ref_mut(self.edge).set_origin(origin);
        self
    }

    /// Sets the target vertex of the edge in the mesh.
    /// This is equivalent to setting the origin of the twin.
    ///
    /// Panics if the twin is void.
    pub fn set_target(self, target: T::V) -> Self {
        self.twin()
            .load()
            .unwrap()
            .set_origin(target)
            .twin()
            .load()
            .unwrap()
    }
}

impl<'a, T: MeshType> ValidEdgeCursorHalfedgeBasics<'a, T> for ValidEdgeCursorMut<'a, T>
where
    T: 'a,
    T::Edge: HalfEdge<T>,
{
}
impl<'a, T: MeshType> ValidEdgeCursorBasics<'a, T> for ValidEdgeCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> EdgeCursorBasics<'a, T> for ValidEdgeCursorMut<'a, T> {}
impl<'a, T: MeshType> EdgeCursorHalfedgeBasics<'a, T, EdgeCursorMut<'a, T>> for ValidEdgeCursorMut<'a, T> where
    T::Edge: HalfEdge<T>
{
}
impl<'a, T: MeshType> EdgeCursorBuilder<'a, T> for ValidEdgeCursorMut<'a, T> where T: 'a {}
