use crate::{
    halfedge::{HalfEdgeImplMeshTypePlus, HalfEdgeMeshImpl},
    math::IndexType,
    mesh::{
        CursorData, EdgeBasics, EdgeCursorBasics, EdgeCursorHalfedgeBasics, EdgePayload, HalfEdge,
        HalfEdgeVertex, MeshBasics, MeshBuilder, MeshHalfEdgeBuilder, VertexCursorBasics,
        VertexCursorHalfedgeBasics,
    },
    prelude::HalfEdgeFaceImpl,
};

/*
impl<T:HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
    fn remove_halfedge_unsafe(&mut self, e:T::E) {
        let edge = self.edge(e);

        // the origin might point to this edge: find a different representative
        if edge.origin(self).edge_id(self) == edge.id() {
            let mut alt = edge.prev(self).twin_id();
            if alt == edge.id() {
                // it was the only edge at this vertex
               alt = IndexType::max();
            }
            self.vertex_mut(edge.origin_id()).set_edge(alt);
        }

        // it is the next of the previous

        todo!("");

        // it is the previous of the next

        todo!("");

        // remove from the datastructure

        todo!("");
    }

    /// Remove the halfedge and its twin.
    /// Adjacent faces are kept. Hence, the graph might be invalid after this operation.
    fn remove_edge_unsafe(&mut self, e: T::E) -> T::F {
        let edge = self.edge(e).clone();
        let twin = edge.twin(self).clone();
        let target = edge.target(self).clone();

       self.remove_halfedge_unsafe(e);
       self.remove_halfedge_unsafe(edge.twin_id());


        0
    }

}*/

impl<T: HalfEdgeImplMeshTypePlus> MeshBuilder<T> for HalfEdgeMeshImpl<T> {
    fn insert_vertex(&mut self, vp: T::VP) -> T::V {
        let new = self.vertices.allocate();
        self.vertices.set(new, T::Vertex::new(IndexType::max(), vp));
        new
    }

    fn try_remove_vertex(&mut self, v: T::V) -> bool {
        if self.vertex(v).edge_id() != IndexType::max() {
            return false;
        }
        self.vertices.delete(v);
        true
    }

    fn try_remove_edge(&mut self, e: T::E) -> bool {
        let edge = self.edge_ref(e).clone();
        if self.try_remove_halfedge(e) {
            let twin_id = edge.twin_id();
            if !self.has_edge(twin_id) {
                // if the twin doesn't exist, that's fine
                return true;
            }
            if self.try_remove_halfedge(twin_id) {
                return true;
            }
            // failed to remove the twin -> revert the removal of the first edge
            self.halfedges.set(e, edge);
        }
        false
    }

    #[inline]
    fn insert_edge_ee_forced(&mut self, input: T::E, output: T::E, ep: T::EP) -> T::E {
        let (e1, _e2) = self.insert_edge_unchecked(
            input,
            output,
            ep,
            IndexType::max(),
            IndexType::max(),
            false,
        );
        e1
    }

    #[inline]
    fn insert_edge_ee(&mut self, input: T::E, output: T::E, ep: T::EP) -> Option<T::E> {
        /*#[cfg(debug_assertions)]
        {
            let i = self.edge(input);
            let o = self.edge(output);
            debug_assert!(i.same_boundary_back(self, o.origin_id()));
            debug_assert!(o.same_boundary_back(self, i.target_id(self)));
        }*/

        // TODO: are there any other checks necessary?

        let (e1, e2) =
            self.insert_edge_unchecked(input, output, ep, IndexType::max(), IndexType::max(), true);

        debug_assert_eq!(self.edge(e1).validate(), Ok(()));
        debug_assert_eq!(self.edge(e2).validate(), Ok(()));

        Some(e1)
    }

    #[inline]
    fn insert_edge_vv(&mut self, a: T::V, b: T::V, ep: T::EP) -> Option<T::E> {
        // TODO: When allowing non-manifold meshes, their vertices might not be at boundary and in the same component, e.g., we could allow an edge from one interior point to another.

        if !self.has_vertex(a) || !self.has_vertex(b) {
            return None;
        }

        let av = self.vertex(a);
        let bv = self.vertex(b);

        if av.is_isolated() {
            if bv.is_isolated() {
                // both isolated - trivial case!
                let (e1, e2) = self.insert_halfedge_pair_forced(
                    IndexType::max(),
                    a,
                    IndexType::max(),
                    IndexType::max(),
                    b,
                    IndexType::max(),
                    IndexType::max(),
                    IndexType::max(),
                    ep,
                );
                self.vertex_mut(a).set_edge(e1);
                self.vertex_mut(b).set_edge(e2);
                debug_assert_eq!(self.edge(e1).validate(), Ok(()));
                debug_assert_eq!(self.edge(e2).validate(), Ok(()));
                return Some(e1);
            } else {
                // a is isolated, b is not isolated

                // find a unique boundary edge
                let Some(b_in) = bv.ingoing_boundary_edge() else {
                    return None;
                };
                let (e1, e2) = self.insert_halfedge_pair_forced(
                    IndexType::max(),
                    a,
                    IndexType::max(),
                    b_in,
                    b,
                    self.edge(b_in).next_id(),
                    IndexType::max(),
                    IndexType::max(),
                    ep,
                );
                self.vertex_mut(a).set_edge(e1);
                debug_assert_eq!(self.edge(e1).validate(), Ok(()));
                debug_assert_eq!(self.edge(e2).validate(), Ok(()));
                return Some(e1);
            }
        } else if bv.is_isolated() {
            // a is not isolated, b is isolated

            // find a unique boundary edge
            let Some(a_in) = av.ingoing_boundary_edge() else {
                return None;
            };
            let next = self.edge_ref(a_in).next_id();
            let (e1, e2) = self.insert_halfedge_pair_forced(
                a_in,
                a,
                next,
                IndexType::max(),
                b,
                IndexType::max(),
                IndexType::max(),
                IndexType::max(),
                ep,
            );
            self.vertex_mut(b).set_edge(e2);
            self.edge_mut(a_in).set_next(e1);
            self.edge_mut(next).set_prev(e2);

            debug_assert_eq!(self.edge(e1).validate(), Ok(()));
            debug_assert_eq!(self.edge(e2).validate(), Ok(()));
            return Some(e1);
        } else {
            // both are not isolated - there must be a shared boundary to figure out the orientation

            // check that there are only multiple edges if the payload allows it
            if !ep.allow_multigraph() {
                if let Some(duplicate) = self.shared_edge(a, b) {
                    if !self.edge_payload(duplicate.id()).allow_multigraph() {
                        return None;
                    }
                }
                if let Some(duplicate) = self.shared_edge(b, a) {
                    if !self.edge_payload(duplicate.id()).allow_multigraph() {
                        return None;
                    }
                }
            }

            // find outgoing edges that are boundary and can reach the other vertex
            let mut m = None;
            for e in av.edges_out() {
                if !e.is_boundary_self() {
                    return None;
                }
                if let Some(other_e) = e.clone().same_boundary(b) {
                    if m.is_some() {
                        return None;
                    }
                    m = Some((e.id(), other_e.id()));
                    // continue searching to make sure there is only one
                }
            }
            let Some((from_a, from_b)) = m else {
                return None;
            };

            return self.insert_edge_ee(self.edge(from_a).prev_id(), from_b, ep);
        }
    }

    fn insert_edge_ev(&mut self, e: T::E, v: T::V, ep: T::EP) -> Option<T::E> {
        if self.vertex(v).is_isolated() {
            // Trivial case where the connectivity is already given
            let edge = self.edge(e);
            let origin = edge.target_id();
            let fo = self.edge(e).next_id();
            let (e1, e2) = self.insert_halfedge_pair_forced(
                e,
                origin,
                fo,
                IndexType::max(),
                v,
                IndexType::max(),
                IndexType::max(),
                IndexType::max(),
                ep,
            );
            self.edge_mut(e).set_next(e1);
            self.edge_mut(fo).set_prev(e2);
            self.vertex_mut(v).set_edge(e2);

            debug_assert_eq!(self.edge(e1).validate(), Ok(()));
            debug_assert_eq!(self.edge(e2).validate(), Ok(()));
            debug_assert_eq!(self.edge(e).validate(), Ok(()));
            debug_assert_eq!(self.edge(e1).target_id(), v);
            debug_assert_eq!(self.edge(e1).origin_id(), self.edge(e).target_id());
            debug_assert_eq!(self.edge(e2).origin_id(), v);

            return Some(e1);
        }

        // If there is only one boundary through `v`, use that one
        if let Some(outgoing) = self.vertex(v).outgoing_boundary_edge() {
            return self.insert_edge_ee(e, outgoing, ep);
        };

        // Otherwise, find a unique boundary from e to v
        if let Some(outgoing) = self.edge(e).same_boundary(v) {
            return self.insert_edge_ee(e, outgoing.id(), ep);
        }

        None
    }

    fn try_remove_face(&mut self, f: T::F) -> bool {
        todo!("{}{:?}", f, self.face(f))
    }

    fn insert_face(&mut self, e: T::E, fp: T::FP) -> Option<T::F> {
        if !self.has_edge(e) {
            return None;
        }
        let edge = self.edge_ref(e).clone();
        if edge.face_id() != IndexType::max() {
            return None;
        }
        let f = self.faces.push(HalfEdgeFaceImpl::new(e, fp));
        edge.edges_face_mut(self).for_each(|e| e.set_face(f));
        return Some(f);
    }

    fn close_face_ee_legacy(
        &mut self,
        from: T::E,
        to: T::E,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        self.close_face_ee(from, self.edge(to).next_id(), ep, fp)
    }

    fn close_face_ee(
        &mut self,
        from: T::E,
        to: T::E,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        if !self.has_edge(from) || !self.has_edge(to) {
            return None;
        }
        if self.edge(from).face_id() != IndexType::max()
            || self.edge(to).face_id() != IndexType::max()
        {
            return None;
        }
        let Some(e) = self.insert_edge_ee(from, to, ep) else {
            return None;
        };
        let f = self.insert_face(e, fp).unwrap();
        Some((e, f))
    }

    #[must_use]
    fn close_face_vv(
        &mut self,
        prev: T::V,
        from: T::V,
        to: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        todo!("{}{}{}{:?}{:?}", prev, from, to, ep, fp)
    }

    fn subdivide_edge<I: Iterator<Item = (T::EP, T::VP)>>(&mut self, e: T::E, _vs: I) -> T::E {
        todo!("{}", e)
    }

    /*fn insert_vertex_e(
         &mut self,
         input: T::E,
         output: T::E,
         vp: T::VP,
     ) -> (T::V, T::E, T::E)
     where
         T::EP: DefaultEdgePayload,
     {
         self.add_vertex_via_edge(input, output, vp, T::EP::default(), T::EP::default())
     }

     fn remove_face(&mut self, f: T::F) -> T::FP {
         let face = self.face(f);

         // TODO: move the payload out of the face without cloning
         let fp = face.payload().clone();

         let edge_ids: Vec<_> = face.edges(self).map(|e| e.id()).collect();
         for e in edge_ids {
             self.edge_mut(e).delete_face();
         }
         self.faces.delete_internal(f);
         fp
     }

     fn close_hole(&mut self, e: T::E, fp: T::FP, curved: bool) -> T::F {
         let f = self.faces.push(HalfEdgeFaceImpl::new(e, curved, fp));
         self.edge(e)
             .clone()
             .edges_face_mut(self)
             .for_each(|e| e.set_face(f));
         return f;
     }

     fn close_face_vertices_default(
         &mut self,
         prev: T::V,
         from: T::V,
         to: T::V,
         curved: bool,
     ) -> (T::F, T::E, T::E)
     where
         T::EP: DefaultEdgePayload,
         T::FP: DefaultFacePayload,
     {
         self.close_face_vertices(
             prev,
             Default::default(),
             from,
             Default::default(),
             to,
             Default::default(),
             curved,
         )
     }

     fn subdivide_edge<I: Iterator<Item = (T::EP, T::EP, T::VP)>>(
         &mut self,
         e: T::E,
         ps: I,
     ) -> T::E {
         let twin_id = self.edge(e).twin_id();
         let mut current = self.edge(e).prev_id();
         let mut current_twin = self.edge(twin_id).next_id();
         let f1 = self.edge(e).face_id();
         let f2 = self.edge(twin_id).face_id();
         let mut last_v = self.edge(e).origin_id();
         let mut first = true;
         for (ep1, ep2, vp) in ps {
             let (v, e1, e2) =
                 self.add_vertex_via_edge(current, self.edge(current).twin_id(), vp, ep1, ep2);
             current = e1;
             current_twin = e2;
             last_v = v;
             self.edge_mut(current).set_face(f1);
             self.edge_mut(current_twin).set_face(f2);
             if first {
                 self.vertex_mut(self.edge(e).origin_id()).set_edge(e1);
                 first = false;
             }
         }

         self.edge_mut(current).set_next(e);
         self.edge_mut(e).set_prev(current);
         self.edge_mut(current_twin).set_prev(twin_id);
         self.edge_mut(twin_id).set_next(current_twin);
         self.edge_mut(e).set_origin(last_v);

         return e;
     }
     fn insert_isolated_edge(&mut self, a: T::VP, b: T::VP) -> (T::V, T::V)
     where
         T::EP: DefaultEdgePayload,
     {
         self.insert_isolated_edge(a, T::EP::default(), b, T::EP::default())
     }

    fn add_vertex_via_edge(
             &mut self,
             input: T::E,
             output: T::E,
             vp: T::VP,
             ep1: T::EP,
             ep2: T::EP,
         ) -> (T::V, T::E, T::E) {
             let v = self.edge(output).origin_id();
             debug_assert!(self.edge(input).target_id(self) == v);

             let new = self.vertices.allocate();

             let (e1, e2) = self.insert_halfedge_pair_forced_checked_no_check(
                 (IndexType::max(), input, v, IndexType::max(), ep1),
                 (output, IndexType::max(), new, IndexType::max(), ep2),
             );

             self.vertices.set(new, T::Vertex::new(e2, vp));

             self.edge_mut(input).set_next(e1);
             self.edge_mut(output).set_prev(e2);

             return (new, e1, e2);
         }

         fn add_vertex_via_vertex(
             &mut self,
             v: T::V,
             vp: T::VP,
             ep1: T::EP,
             ep2: T::EP,
         ) -> (T::V, T::E, T::E) {
             let (input, output) = if self.vertex(v).has_only_one_edge(self) {
                 let e = self.vertex(v).edge(self).unwrap();
                 (e.twin_id(), e.id())
             } else {
                 let Some(boundary) = self
                     .vertex(v)
                     .edges_out(self)
                     .find(|e| e.is_boundary_self())
                 else {
                     panic!("Vertex is not a boundary vertex");
                 };
                 debug_assert!(
                     self.vertex(v)
                         .edges_out(self)
                         .filter(|e| e.is_boundary_self())
                         .count()
                         == 1
                 );
                 (boundary.prev_id(), boundary.id())
             };

             debug_assert!(self.edge(input).is_boundary_self());
             debug_assert!(self.edge(output).is_boundary_self());

             return self.add_vertex_via_edge(input, output, vp, ep1, ep2);
         }

         fn add_isolated_edge(&mut self, a: T::VP, epa: T::EP, b: T::VP, epb: T::EP) -> (T::V, T::V) {
             let v0 = self.vertices.allocate();
             let v1 = self.vertices.allocate();
             let (e0, e1) = self.insert_halfedge_pair_forced_checked_no_check(
                 (
                     IndexType::max(),
                     IndexType::max(),
                     v0,
                     IndexType::max(),
                     epa,
                 ),
                 (
                     IndexType::max(),
                     IndexType::max(),
                     v1,
                     IndexType::max(),
                     epb,
                 ),
             );
             self.vertices.set(v0, HalfEdgeVertexImpl::new(e0, a));
             self.vertices.set(v1, HalfEdgeVertexImpl::new(e1, b));

             (v0, v1)
         }

         fn insert_edge_between(
             &mut self,
             origin0: T::V,
             ep0: T::EP,
             origin1: T::V,
             ep1: T::EP,
         ) -> (T::E, T::E) {
             debug_assert!(
                 self.has_vertex(origin0),
                 "First Vertex {} does not exist",
                 origin0
             );
             debug_assert!(
                 self.has_vertex(origin1),
                 "Second Vertex {} does not exist",
                 origin1
             );
             // TODO: test cases with multigraphs! For curved edges, this is perfectly fine
             /*debug_assert!(
                 self.shared_edge(origin0, origin1).is_none(),
                 "There is already an edge between first vertex {} and second vertex {}",
                 origin0,
                 origin1
             );
             debug_assert!(
                 self.shared_edge(origin1, origin0).is_none(),
                 "There is already an edge between second vertex {} and first vertex {}",
                 origin1,
                 origin0
             );*/
             // TODO: is this necessary or not?
             /*debug_assert!(
                 self.shortest_path(origin0, origin1).is_none(),
                 "Vertices {} and {} must be in different connected components",
                 origin0,
                 origin1
             );*/

             // We are connecting two vertices at the boundary of two connected components.
             // Hence, the edge from v0 to v1 will come from the ingoing boundary
             // edge of v0 and go to the outgoing boundary edge of v1.

             // TODO: When allowing non-manifold meshes, their vertices might not be at boundary and in the same component, e.g., we could allow an edge from one interior point to another.

             let inserter = |inwards: bool, origin: T::V| {
                 if self.vertex(origin).edge_id(self) == IndexType::max() {
                     // if the vertex doesn't have edges the edges should refer to their twins
                     IndexType::max()
                 } else if inwards {
                     self.vertex(origin)
                         .ingoing_boundary_edge(self)
                         .expect("There must be an intgoing boundary edge")
                 } else {
                     self.vertex(origin)
                         .outgoing_boundary_edge(self)
                         .expect("There must be an outgoing boundary edge")
                 }
             };

             let next0 = inserter(false, origin1);
             let prev0 = inserter(true, origin0);
             let next1 = inserter(false, origin0);
             let prev1 = inserter(true, origin1);

             let (e0, e1) = self.insert_halfedge_pair_forced_checked_no_check(
                 (next0, prev0, origin0, IndexType::max(), ep0),
                 (next1, prev1, origin1, IndexType::max(), ep1),
             );

             if next0 != IndexType::max() {
                 self.edge_mut(next0).set_prev(e0);
             } else {
                 self.vertex_mut(origin1).set_edge(e1);
             }

             if prev0 != IndexType::max() {
                 self.edge_mut(prev0).set_next(e0);
             }

             if next1 != IndexType::max() {
                 self.edge_mut(next1).set_prev(e1);
             } else {
                 self.vertex_mut(origin0).set_edge(e0);
             }

             if prev1 != IndexType::max() {
                 self.edge_mut(prev1).set_next(e1);
             }

             (e0, e1)
         }

         fn close_face(
             &mut self,
             inside: T::E,
             ep1: T::EP,
             outside: T::E,
             ep2: T::EP,
             fp: T::FP,
             curved: bool,
         ) -> (T::F, T::E, T::E) {
             let (e1, e2) = self.insert_edge(inside, ep1, outside, ep2);

             // Insert the face
             let f = self.faces.push(HalfEdgeFaceImpl::new(inside, curved, fp));

             self.edge(inside)
                 .clone()
                 .edges_face_mut(self)
                 .for_each(|e| e.set_face(f));

             return (f, e1, e2);
         }

         fn close_face_vertices(
             &mut self,
             prev: T::V,
             ep1: T::EP,
             from: T::V,
             ep2: T::EP,
             to: T::V,
             fp: T::FP,
             curved: bool,
         ) -> (T::F, T::E, T::E) {
             let inside = self.shared_edge(prev, from).unwrap().id();

             // TODO: is it enough to assert this vertex is manifold? Also, add code to check for manifold vertices!
             debug_assert!(
                 self.vertex(to).edges_in(self).filter( |e| {
                     e.is_boundary_self() && e.same_face(self, self.edge(inside).origin_id())
                 }).exactly_one().is_ok(),
                 "There mus be exactly one ingoing edge to {} that can reach edge {} but there were the following ones: {:?}",
                 to,
                 inside,
                 self.vertex(to).edges_in(self).filter(|e| {
                     e.is_boundary_self() && e.same_face(self, self.edge(inside).origin_id())
                 }).collect::<Vec<_>>()
             );

             let outside = self
                 .vertex(to)
                 .edges_in(self)
                 .find(|e| e.is_boundary_self() && e.same_face(self, self.edge(inside).origin_id()))
                 .unwrap()
                 .id();

             return self.close_face(inside, ep1, outside, ep2, fp, curved);
         }

         fn close_face_ee(
             &mut self,
             inside: T::E,
             outside: T::E,
             curved: bool,
         ) -> (T::F, T::E, T::E)
         where
             T::EP: DefaultEdgePayload,
             T::FP: DefaultFacePayload,
         {
             self.close_face(
                 inside,
                 Default::default(),
                 outside,
                 Default::default(),
                 Default::default(),
                 curved,
             )
         }
     }

     */
}
