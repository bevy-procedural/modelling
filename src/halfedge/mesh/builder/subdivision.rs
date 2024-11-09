use crate::{
    halfedge::{HalfEdgeImpl, HalfEdgeMeshImpl, HalfEdgeMeshType, HalfEdgeVertexImpl},
    mesh::{DefaultEdgePayload, EdgeBasics, HalfEdge, MeshBasics},
};

impl<T: HalfEdgeMeshType> HalfEdgeMeshImpl<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    pub(crate) fn subdivide_unsafe(&mut self, e: T::E, vp: T::VP) -> T::E {
        let old_edge = self.edge(e).clone();

        let new_v = self.vertices.allocate();
        let new_edge = self.halfedges.allocate();

        self.halfedges.set(
            new_edge,
            HalfEdgeImpl::new(
                old_edge.next_id(),
                old_edge.twin_id(),
                old_edge.id(),
                new_v,
                old_edge.face_id(),
                Default::default(),
            ),
        );
        self.vertices.set(new_v, HalfEdgeVertexImpl::new(new_edge, vp));

        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        new_edge
    }

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    pub(crate) fn subdivide_unsafe_try_fixup(&mut self, e: T::E) -> Option<T::E> {
        let old_edge = self.edge(e).clone();
        let other_old = old_edge.twin(self);

        // find the "other_new". It has the characteristic property of sharing the same twin with the old edge.
        let mut other_new = other_old.next(self);
        let first_other_new_origin = other_new.origin_id();
        loop {
            if other_new.twin_id() == e {
                break;
            }
            other_new = other_new.twin(self).next(self);
            if other_new.origin_id() != first_other_new_origin {
                // Not a valid wheel
                return None;
            }
            if other_new.prev_id() == other_old.id() {
                // Went a full round
                return None;
            }
        }

        // Insert the new edge
        let new_edge = self.halfedges.allocate();
        self.halfedges.set(
            new_edge,
            HalfEdgeImpl::new(
                old_edge.next_id(),
                other_old.id(),
                old_edge.id(),
                other_new.origin_id(),
                old_edge.face_id(),
                Default::default(),
            ),
        );

        // update the neighbors
        self.edge_mut(old_edge.id()).set_twin(other_new.id());
        self.edge_mut(other_old.id()).set_twin(new_edge);
        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        Some(new_edge)
    }
}
