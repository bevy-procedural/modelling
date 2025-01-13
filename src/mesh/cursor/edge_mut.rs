use super::{
    CursorData, EdgeCursor, EdgeCursorBasics, EdgeCursorData, EdgeCursorHalfedgeBasics,
    FaceCursorMut, VertexCursorMut,
};
use crate::{
    math::IndexType,
    mesh::{HalfEdge, MeshBasics, MeshBuilder, MeshType},
};

/// An edge cursor pointing to an edge of a mesh with a mutable reference to the mesh.
pub struct EdgeCursorMut<'a, T: MeshType + 'a> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
    // TODO: Integrate the path builder into the edge cursor mut! This should now include setting the start etc.
}

impl<'a, T: MeshType> std::fmt::Debug for EdgeCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeCursorMut({:?})", self.edge)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Creates a new void edge cursor.
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a mut T::Mesh) -> Self {
        Self::new(mesh, IndexType::max())
    }

    /// Returns an immutable clone pointing to the same edge.
    #[inline]
    #[must_use]
    pub fn as_immutable(&'a self) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, self.try_id())
    }

    /// Returns a mutable reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::EP {
        self.mesh.edge_payload_mut(self.try_id())
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursorMut<'a, T> {
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

impl<'a, T: MeshType + 'a> CursorData for EdgeCursorMut<'a, T> {
    type I = T::E;
    type S = T::Edge;
    type T = T;

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_edge(self.try_id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Edge> {
        self.mesh().get_edge(self.try_id())
    }

    #[inline]
    fn try_id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> EdgeCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursorMut<'a, T> {}
impl<'a, T: MeshType + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursorMut<'a, T> where
    T::Edge: HalfEdge<T>
{
}

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the edge etc. using a chaining syntax.
impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T> {
    /// Tries to remove the current edge.
    ///
    /// If the edge was successfully removed or didn't exist, returns `None`.
    /// Otherwise, returns an edge cursor still pointing to the same edge.
    ///
    /// See [MeshBuilder::try_remove_edge] for more information.
    #[inline]
    #[must_use]
    pub fn remove(self) -> Option<Self> {
        if self.mesh.try_remove_edge(self.edge) {
            None
        } else if self.is_void() {
            None
        } else {
            Some(self)
        }
    }

    /// "Recursively" removes the edge and all adjacent faces.
    /// If you want to preserve the faces, use [EdgeCursorMut::collapse] instead.
    /// Panics if the edge is void.
    ///
    /// Moves the cursor to the next edge.
    /// If the next edge is the same as the current twin, the cursor will be void.
    ///
    /// See [MeshBuilder::remove_edge_r] for more information.
    #[inline]
    #[must_use]
    pub fn remove_r(self) -> Self
    where
        T::Edge: HalfEdge<T>,
    {
        let id = self.edge;
        let c = if self.next_id() == self.twin_id() {
            self.void()
        } else {
            self.next()
        };
        c.mesh.remove_edge_r(id);
        c
    }

    /// Inserts a new vertex and half-edge pair. The halfedge leading to the
    /// new vertex will become the "next" of the current edge and the cursor will move
    /// to this newly created halfedge.
    ///
    /// Returns `None` if the insertion was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_vertex_e] for more information.
    #[inline]
    #[must_use]
    pub fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Option<Self> {
        let old_target = self.target_id();
        let (e, _v) = self.mesh.insert_vertex_e(self.edge, vp, ep)?;
        let c = self.move_to(e);
        debug_assert!(old_target == c.origin_id());
        Some(c)
    }

    /// Connects the current halfedge to the given halfedge.
    /// Returns `None` if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ee] for more information.
    #[inline]
    #[must_use]
    pub fn connect(self, other: T::E, ep: T::EP) -> Option<Self> {
        let e = self.mesh.insert_edge_ee(self.edge, other, ep)?;
        Some(self.move_to(e))
    }

    /// Connects the current halfedge to the given vertex.
    /// Returns `None` if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    pub fn connect_v(self, other: T::V, ep: T::EP) -> Option<Self> {
        let e = self.mesh.insert_edge_ev(self.edge, other, ep)?;
        Some(self.move_to(e))
    }

    /// Inserts a face in the boundary of the current halfedge and move the cursor to the new face.
    /// If the face already exists, move there and return that cursor instead.
    ///
    /// Returns `None` on error.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_face] for more information.
    #[inline]
    #[must_use]
    pub fn insert_face(self, fp: T::FP) -> Option<FaceCursorMut<'a, T>>
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        assert!(self.is_valid());
        Some(if let Some(f) = self.mesh.insert_face(self.edge, fp) {
            self.move_to_face(f)
        } else {
            self.face()
        })
    }

    /// Sets the face of the edge in the mesh even if it already has a face.
    /// Calling this method with `IndexType::max()` will remove the face.
    ///
    /// Panics if the cursor is void.
    #[inline]
    pub fn replace_face(self, face: T::F) -> Self
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        let f = if self.has_face() {
            self.remove_face()
        } else {
            self
        };
        if face != IndexType::max() {
            f.set_face(face)
        } else {
            f
        }
    }

    /// Removes the face from the edge.
    /// Won't update the neighbors - so the mesh will be invalid after this operation.
    ///
    /// Panics if the cursor is void.
    ///
    /// Doesn't delete the face itself from the mesh.
    /// Use `c.face().remove()` to delete the face from the mesh and remove it from all adjacent edges.
    #[inline]
    pub fn remove_face(self) -> Self
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        self.mesh.edge_ref_mut(self.edge).remove_face();
        self
    }

    /// Insert an edge to the given vertex and move the cursor to that new edge.
    /// Close the resulting face with the given face payload.
    /// Return `None` if the insertion failed.
    ///
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::close_face_ev] for more information.
    #[inline]
    #[must_use]
    pub fn close_face_v(self, v: T::V, ep: T::EP, fp: T::FP) -> Option<Self> {
        let (e, _f) = self.mesh.close_face_ev(self.edge, v, ep, fp)?;
        Some(self.move_to(e))
    }

    /// Insert an edge to the given edge's input and move the cursor to that new edge.
    /// Close the resulting face with the given face payload.
    /// Return `None` if the insertion failed.
    ///
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::close_face_ee] for more information.
    #[inline]
    #[must_use]
    pub fn close_face(self, e: T::E, ep: T::EP, fp: T::FP) -> Option<Self> {
        let (e, _f) = self.mesh.close_face_ee(self.edge, e, ep, fp)?;
        Some(self.move_to(e))
    }

    /// Subdivides the given edge by inserting a vertex in the middle, using
    /// that vertex as the new target and inserting a new edge from the middle vertex
    /// to the original target.
    ///
    /// Panics if the cursor is void.
    ///
    /// Moves the cursor to the new edge (the original edge will be the `prev` of the new edge).
    ///
    /// See [MeshBuilder::subdivide_edge] for more information.
    #[inline]
    pub fn subdivide(self, vp: T::VP, ep: T::EP) -> Self {
        let e = self.mesh.subdivide_edge(self.edge, vp, ep);
        self.move_to(e)
    }

    /// Collapses the edge with the next edge.
    /// Keeps the payload of the current edge.
    /// If the target of the current edge doesn't have a degree of 2, the operation will fail and return `None`.
    ///
    /// Panics if the cursor is void.
    /// Doesn't move the cursor.
    ///
    /// See [MeshBuilder::collapse_edge] for more information.
    #[inline]
    #[must_use]
    pub fn collapse(self) -> Option<Self> {
        let e = self.mesh.collapse_edge(self.edge)?;
        Some(self.move_to(e))
    }

    /// Subdivide the adjacent face by inserting an edge from the current target to the given other edge's origin.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns `None` if the other edge is not adjacent to the same face or the resulting faces would've been invalid.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::subdivide_face] for more information.
    #[inline]
    #[must_use]
    pub fn subdivide_face(self, output: T::E, ep: T::EP, fp: T::FP) -> Option<Self>
    where
        T::Edge: HalfEdge<T>,
    {
        let e = self.mesh.subdivide_face(self.edge, output, ep, fp)?;
        Some(self.move_to(e))
    }

    /// Subdivide the adjacent face by inserting an edge from the current target to the given vertex.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns `None` if the resulting faces would've been invalid.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::subdivide_face_v] for more information.
    #[inline]
    #[must_use]
    pub fn subdivide_face_v(self, v: T::V, ep: T::EP, fp: T::FP) -> Option<Self>
    where
        T::Edge: HalfEdge<T>,
    {
        let e = self
            .mesh
            .subdivide_face_v(self.face_id(), self.target_id(), v, ep, fp)?;
        Some(self.move_to(e))
    }

    /// Appends multiple edges to the current edge given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last edge (such that the last inserting vertex is the target).
    /// If the iterator is empty, don't move the cursor.
    /// Panics if the cursor is void.
    #[inline]
    pub fn append_path(self, iter: impl IntoIterator<Item = (T::VP, T::EP)>) -> Self {
        let mut c = self;
        for (vp, ep) in iter {
            c = c.insert_vertex(vp, ep).unwrap();
        }
        c
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    /// Runs the closure on all outgoing halfedges of the target.
    pub fn for_each_next<F: Fn(Self) -> Self>(self, f: F) -> Self {
        let twin = self.twin();
        let id = twin.id();
        let mut c = twin.next_sibling();
        while c.id() != id {
            let c_id = c.id();
            // execute closure, reset to the original edge and continue with the next sibling
            c = f(c).move_to(c_id).next_sibling();
        }
        c
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
    pub fn set_target(self, target: T::V) -> Self {
        self.twin().set_origin(target).twin()
    }
}
