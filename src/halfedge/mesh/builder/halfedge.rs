use crate::{
    halfedge::{HalfEdgeImpl, HalfEdgeImplMeshTypePlus, HalfEdgeMeshImpl, HalfEdgeVertexImpl},
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdge, MeshBasics, MeshHalfEdgeBuilder},
};

impl<T: HalfEdgeImplMeshTypePlus> MeshHalfEdgeBuilder<T> for HalfEdgeMeshImpl<T> {
    #[inline]
    fn insert_halfedge_pair_forced(
        &mut self,
        to_origin: T::E,
        origin: T::V,
        from_origin: T::E,
        to_target: T::E,
        target: T::V,
        from_target: T::E,
        forward_face: T::F,
        backward_face: T::F,
        ep: T::EP,
    ) -> (T::E, T::E) {
        let forward = self.halfedges.allocate();
        let backward = self.halfedges.allocate();
        self.insert_halfedge_forced(
            forward,
            origin,
            forward_face,
            if to_origin == IndexType::max() {
                backward
            } else {
                to_origin
            },
            backward,
            if from_target == IndexType::max() {
                backward
            } else {
                from_target
            },
            Some(ep),
        );
        self.insert_halfedge_forced(
            backward,
            target,
            backward_face,
            if to_target == IndexType::max() {
                forward
            } else {
                to_target
            },
            forward,
            if from_origin == IndexType::max() {
                forward
            } else {
                from_origin
            },
            None,
        );
        (forward, backward)
    }

    #[inline]
    fn try_remove_halfedge(&mut self, e: T::E) -> bool {
        if self.edge(e).is_void() {
            return false;
        }
        self.halfedges.delete(e);
        true
    }

    fn subdivide_halfedge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E {
        // TODO: This could be done in a more generic way in a standard impl

        let old_edge = self.edge_ref(e).clone();

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
                Some(ep),
            ),
        );
        self.vertices
            .set(new_v, HalfEdgeVertexImpl::new(new_edge, vp));

        // TODO: Unwrap
        self.edge_mut(old_edge.next_id())
            .unwrap()
            .set_prev(new_edge);
        self.edge_mut(old_edge.id()).unwrap().set_next(new_edge);

        new_edge
    }

    fn subdivide_halfedge_try_fixup(&mut self, e: T::E, ep: T::EP) -> Option<T::E> {
        let old_edge = self.edge_ref(e).clone();
        let other_old = old_edge.twin(self).id();

        // find the "other_new". It has the characteristic property of sharing the same twin with the old edge.
        let mut other_new = old_edge.twin(self).next(self);
        let first_other_new_origin = other_new.origin_id(self);
        loop {
            if other_new.twin_id() == e {
                break;
            }
            other_new = other_new.twin(self).next(self);
            if other_new.origin_id(self) != first_other_new_origin {
                // Not a valid wheel
                return None;
            }
            if other_new.prev_id() == other_old {
                // Went a full round
                return None;
            }
        }
        let ono = other_new.origin_id(self);
        let oi = other_new.id();

        // Insert the new edge
        let new_edge = self.halfedges.allocate();
        self.halfedges.set(
            new_edge,
            HalfEdgeImpl::new(
                old_edge.next_id(),
                other_old,
                old_edge.id(),
                ono,
                old_edge.face_id(),
                Some(ep),
            ),
        );

        // TODO: Handle invalid intermediate state

        // update the neighbors
        self.edge_mut(old_edge.id()).load()?.set_twin(oi);
        self.edge_mut(other_old).load()?.set_twin(new_edge);
        self.edge_mut(old_edge.next_id()).load()?.set_prev(new_edge);
        self.edge_mut(old_edge.id()).load()?.set_next(new_edge);

        Some(new_edge)
    }
}

impl<T: HalfEdgeImplMeshTypePlus> HalfEdgeMeshImpl<T> {
    /// Inserts a half-edge pair with the given ids.
    /// Updates the neighbors.
    /// Doesn't check whether the operation is allowed!
    #[inline]
    pub(super) fn insert_edge_unchecked(
        &mut self,
        input: T::E,
        output: T::E,
        ep: T::EP,
        f_face: T::F,
        b_face: T::F,
        should_be_valid: bool,
    ) -> (T::E, T::E) {
        let (fv, tw, v, w) = {
            // TODO: unwrap
            let e_input = self.edge(input).unwrap();
            let e_output = self.edge(output).unwrap();
            (
                e_input.next_id(),
                e_output.prev_id(),
                e_input.target_id(),
                e_output.origin_id(),
            )
        };

        debug_assert_eq!(self.edge(input).unwrap().target_id(), v);
        debug_assert_eq!(self.edge(output).unwrap().origin_id(), w);

        let (e1, e2) = if should_be_valid {
            self.insert_halfedge_pair(input, v, fv, tw, w, output, f_face, b_face, ep)
        } else {
            self.insert_halfedge_pair_forced(input, v, fv, tw, w, output, f_face, b_face, ep)
        };

        // TODO: unwrap
        self.edge_mut(fv).unwrap().set_prev(e2);
        self.edge_mut(tw).unwrap().set_next(e2);
        self.edge_mut(output).unwrap().set_prev(e1);
        self.edge_mut(input).unwrap().set_next(e1);
        self.vertex_mut(v).set_edge(e1);
        self.vertex_mut(w).set_edge(e2);

        (e1, e2)
    }

    /// Inserts a single half-edge with the given id.
    /// This will not update the neighbors and will not check whether the operation is allowed!
    /// After this operation, the mesh might be in an inconsistent state.
    #[inline]
    pub(crate) fn insert_halfedge_forced(
        &mut self,
        edge: T::E,
        origin: T::V,
        face: T::F,
        prev: T::E,
        twin: T::E,
        next: T::E,
        ep: Option<T::EP>,
    ) {
        self.halfedges
            .set(edge, HalfEdgeImpl::new(next, twin, prev, origin, face, ep));
    }
}
