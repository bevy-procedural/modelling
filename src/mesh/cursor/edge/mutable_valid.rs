use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType, MeshTypeHalfEdge},
};

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

impl_debug_eq_cursor!(ValidEdgeCursorMut, edge);

#[rustfmt::skip]
impl_specific_cursor_data!(
    EdgeCursorData, ValidEdgeCursorMut,
    FC, move_to_face, T::F,FaceCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, MutableCursor, ValidEdgeCursorMut, EdgeCursorMut, 
    edge, E, Edge, EP, 
    get_edge, get_edge_mut, has_edge,
    EdgeCursorBuilder, ValidEdgeCursorBasics, EdgeCursorBasics, EdgeCursorHalfedgeBasics
);

impl<'a, T: MeshType + 'a> ValidEdgeCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
    /// Runs the closure on all outgoing halfedges of the target.
    /// Panics if one of the outgoing halfedges doesn't have a twin.
    pub fn for_each_next<F: Fn(Self) -> Self>(self, f: F) -> Self {
        let twin = self.twin();
        let id = twin.id_unchecked();
        let mut c = twin.next_sibling();
        while c.id_unchecked() != id {
            let c_id = c.id_unchecked();
            // execute closure, reset to the original edge and continue with the next sibling
            c = f(c.load().unwrap()).move_to(c_id).next_sibling();
        }

        assert!(c.id_unchecked() == id, "Invalid edge cursor: {}", c.id_unchecked());
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

impl<'a, T: MeshType + 'a> ValidEdgeCursorHalfedgeBasics<'a, T> for ValidEdgeCursorMut<'a, T> where
    T::Edge: HalfEdge<T>
{
}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorHalfedgeBuilder<'a, T> for ValidEdgeCursorMut<'a, T> {}
