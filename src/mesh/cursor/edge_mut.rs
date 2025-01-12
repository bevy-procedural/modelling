use super::{
    CursorData, EdgeCursor, EdgeCursorBasics, EdgeCursorData, EdgeCursorHalfedgeBasics,
    FaceCursorMut, VertexCursorMut,
};
use crate::{
    math::IndexType,
    mesh::{HalfEdge, MeshBasics, MeshBuilder, MeshType},
};
use std::fmt::Debug;

/// An edge cursor pointing to an edge of a mesh with a mutable reference to the mesh.
#[derive(Debug)]
pub struct EdgeCursorMut<'a, T: MeshType + 'a> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
    // TODO: Integrate the path builder into the edge cursor mut! This should now include setting the start etc.
}

impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Returns an immutable clone pointing to the same edge.
    #[inline]
    #[must_use]
    pub fn immutable(&'a self) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, self.edge)
    }

    /// Returns a mutable reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::EP {
        self.mesh.edge_payload_mut(self.edge)
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
    /// If the edge was successfully removed or didn't exist, returns `None`.
    /// Otherwise, returns an cursor still pointing to the same edge.
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

    /// Inserts a new vertex and half-edge pair. The halfedge leading to the
    /// new vertex will become the "next" of the current edge and the cursor will move
    /// to this newly created halfedge.
    /// Returns `None` if the insertion was not successful or the cursor was void.
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
    /// See [MeshBuilder::insert_edge_ee] for more information.
    #[inline]
    #[must_use]
    pub fn connect(self, other: T::E, ep: T::EP) -> Option<Self> {
        let e = self.mesh.insert_edge_ee(self.edge, other, ep)?;
        Some(self.move_to(e))
    }

    /// Connects the current halfedge to the given vertex.
    /// Returns `None` if the connection was not successful or the cursor was void.
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    pub fn connect_v(self, other: T::V, ep: T::EP) -> Option<Self> {
        let e = self.mesh.insert_edge_ev(self.edge, other, ep)?;
        Some(self.move_to(e))
    }

    /// Inserts a face in the boundary of the current halfedge and move the cursor to the new face.
    /// If the face already exists, move there and return that cursor instead.
    /// Returns `None` on error or if the cursor was void.
    /// See [MeshBuilder::insert_face] for more information.
    #[inline]
    pub fn insert_face(self, fp: T::FP) -> Option<FaceCursorMut<'a, T>>
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        if self.is_void() {
            return None;
        }
        Some(if let Some(f) = self.mesh.insert_face(self.edge, fp) {
            self.move_to_face(f)
        } else {
            self.face()
        })
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    /// Runs the closure on all outgoing halfedges of the target.
    pub fn all_next<F: Fn(Self) -> Self>(self, f: F) -> Self {
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
    pub fn set_next(&mut self, next: T::E) {
        self.mesh.edge_ref_mut(self.edge).set_next(next);
    }

    /// Sets the previous halfedge of the edge in the mesh.
    pub fn set_prev(&mut self, prev: T::E) {
        self.mesh.edge_ref_mut(self.edge).set_prev(prev);
    }

    /// Sets the twin halfedge of the edge in the mesh.
    pub fn set_twin(&mut self, twin: T::E) {
        self.mesh.edge_ref_mut(self.edge).set_twin(twin);
    }

    /// Sets the face of the edge in the mesh.
    pub fn set_face(&mut self, face: T::F) {
        self.mesh.edge_ref_mut(self.edge).set_face(face);
    }

    /// Removes the face of the edge in the mesh.
    pub fn remove_face(&mut self) {
        self.mesh.edge_ref_mut(self.edge).remove_face();
    }

    /// Sets the origin vertex of the edge in the mesh.
    pub fn set_origin(&mut self, origin: T::V) {
        self.mesh.edge_ref_mut(self.edge).set_origin(origin);
    }
}
