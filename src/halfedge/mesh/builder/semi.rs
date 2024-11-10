use crate::{
    halfedge::{HalfEdgeImpl, HalfEdgeMeshImpl, HalfEdgeImplMeshType, HalfEdgeVertexImpl},
    math::IndexType,
    mesh::{
        EdgeBasics, HalfEdge, HalfEdgeSemiBuilder, MeshBasics, MeshHalfEdgeBuilder, VertexBasics,
    },
};

// TODO: Simplify these

impl<T: HalfEdgeImplMeshType> HalfEdgeSemiBuilder<T> for HalfEdgeMeshImpl<T> {
    fn insert_edge_no_check(
        &mut self,
        inside: T::E,
        ep1: T::EP,
        outside: T::E,
        ep2: T::EP,
    ) -> (T::E, T::E) {
        let e_inside = self.edge(inside);
        let e_outside = self.edge(outside);
        let v = e_inside.target(self).id();
        let w = e_outside.target(self).id();

        let other_inside = e_outside.next(self);
        let other_outside = e_inside.next(self);

        let (e1, e2) = self.insert_edge_no_update_no_check(
            (other_inside.id(), inside, v, IndexType::max(), ep1),
            (other_outside.id(), outside, w, IndexType::max(), ep2),
        );

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        (e1, e2)
    }

    fn insert_edge_no_update(
        &mut self,
        (next1, prev1, origin1, face1, ep1): (T::E, T::E, T::V, T::F, T::EP),
        (next2, prev2, origin2, face2, ep2): (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E) {
        debug_assert!(
            self.has_vertex(origin1),
            "First Vertex {} does not exist",
            origin1
        );
        debug_assert!(
            self.has_vertex(origin2),
            "Second Vertex {} does not exist",
            origin2
        );
        debug_assert!(
            self.edge(prev1).next_id() == next2,
            "Previous edge of first edge {} must point to the next edge {}",
            prev1,
            next1
        );
        debug_assert!(
            self.edge(prev2).next_id() == next1,
            "Previous edge of second edge {} must point to the next edge {}",
            prev2,
            next2
        );
        debug_assert!(
            self.edge(next2).origin_id() == origin1 && origin1 == self.edge(prev1).target_id(self),
            "Next edge of first edge {} must start at the target {} != {} != {} of the previous edge {}",
            next1,
            self.edge(next1).origin_id(),
            origin1,
            self.edge(prev1).target_id(self),
            prev1
        );
        debug_assert!(
            self.edge(next1).origin_id() == origin2 && origin2 == self.edge(prev2).target_id(self),
            "Next edge of second edge {} must start at the target {} != {} != {} of the previous edge {}",
            next2,
            self.edge(next2).origin_id(),
            origin2,
            self.edge(prev2).target_id(self),
            prev2
        );
        debug_assert!(
            self.shared_edge(origin1, origin2).is_none(),
            "There is already an edge between first vertex {} and second vertex {}",
            origin1,
            origin2
        );
        debug_assert!(
            self.shared_edge(origin2, origin1).is_none(),
            "There is already an edge between second vertex {} and first vertex {}",
            origin2,
            origin1
        );

        // TODO: validate that the setting of IndexType::Max() is valid!

        let res: (_, _) = self.insert_edge_no_update_no_check(
            (next1, prev1, origin1, face1, ep1),
            (next2, prev2, origin2, face2, ep2),
        );

        return res;
    }

    #[inline(always)]
    fn insert_edge_no_update_no_check(
        &mut self,
        (next1, prev1, origin1, face1, ep1): (T::E, T::E, T::V, T::F, T::EP),
        (next2, prev2, origin2, face2, ep2): (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E) {
        // TODO: remove the tuples!

        let e1 = self.halfedges.allocate();
        let e2 = self.halfedges.allocate();
        self.insert_halfedge_no_update_no_check(e1, origin1, face1, prev1, e2, next1, ep1);
        self.insert_halfedge_no_update_no_check(e2, origin2, face2, prev2, e1, next2, ep2);
        (e1, e2)
    }

    fn subdivide_unsafe(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E {
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
                ep,
            ),
        );
        self.vertices
            .set(new_v, HalfEdgeVertexImpl::new(new_edge, vp));

        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        new_edge
    }

    fn subdivide_unsafe_try_fixup(&mut self, e: T::E, ep: T::EP) -> Option<T::E> {
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
                ep,
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
