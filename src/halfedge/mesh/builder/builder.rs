use crate::{
    halfedge::{HalfEdgeImplMeshTypePlus, HalfEdgeMeshImpl},
    math::IndexType,
    mesh::{
        cursor::*, EdgeBasics, EdgePayload, HalfEdge, HalfEdgeVertex, MeshBasics, MeshBuilder,
        MeshHalfEdgeBuilder, MeshTypeHalfEdge, VertexBasics,
    },
    prelude::HalfEdgeFaceImpl,
};
use itertools::Itertools;

impl<T: HalfEdgeImplMeshTypePlus> MeshBuilder<T> for HalfEdgeMeshImpl<T> {
    #[inline]
    fn insert_vertex(&mut self, vp: T::VP) -> T::V {
        let new = self.vertices.allocate();
        self.vertices.set(new, T::Vertex::new(IndexType::max(), vp));
        new
    }

    #[inline]
    fn try_remove_vertex(&mut self, v: T::V) -> bool {
        let c = self.vertex(v);
        let Some(vertex) = c.try_inner() else {
            return false;
        };
        if vertex.edge_id(self) != IndexType::max() {
            return false;
        }
        self.vertices.delete(v);
        true
    }

    #[inline]
    fn try_remove_edge(&mut self, e: T::E) -> bool {
        let Some(edge) = self.edge(e).load() else {
            return false;
        };
        if edge.has_face() {
            return false;
        }
        let Some(twin) = edge.twin().load() else {
            return false;
        };
        if twin.has_face() {
            return false;
        }
        self.try_remove_edge_forced(e)
    }

    #[inline]
    fn try_remove_edge_forced(&mut self, e: T::E) -> bool {
        if !self.has_edge(e) {
            return false;
        }
        let edge = self.edge_ref(e).clone();
        let Some(twin) = self.get_edge(edge.twin_id()).cloned() else {
            return false;
        };

        self.halfedges.delete(edge.id());
        self.halfedges.delete(twin.id());

        debug_assert!(!self.has_edge(e));
        debug_assert!(!self.has_edge(twin.id()));

        fn fix_edge<T: MeshTypeHalfEdge>(edge: &T::Edge, twin: &T::Edge, mesh: &mut T::Mesh) {
            debug_assert_eq!(edge.twin_id(), twin.id());
            debug_assert_eq!(twin.twin_id(), edge.id());

            // if the edge is the representative edge of the vertex, update the vertex
            if edge.origin(mesh).edge_id(mesh) == edge.id() {
                let oi = edge.origin_id(mesh);
                let id = if edge.prev_id() == edge.twin_id() {
                    // it is the only edge of the vertex
                    IndexType::max()
                } else {
                    let prev_sibling = edge.prev(mesh).twin_id();
                    if prev_sibling == edge.id() {
                        // it was the only edge of the vertex
                        IndexType::max()
                    } else {
                        prev_sibling
                    }
                };
                mesh.vertex_mut(oi).set_edge(id);
            }

            debug_assert_ne!(edge.origin(mesh).edge_id(mesh), edge.id());

            // The next edge of the previous must be updated
            if edge.prev_id() != edge.twin_id() {
                mesh.edge_mut(edge.prev_id())
                    .unwrap()
                    .set_next(twin.next_id());
            }
            if edge.next_id() != edge.twin_id() {
                mesh.edge_mut(edge.next_id())
                    .unwrap()
                    .set_prev(twin.prev_id());
            }
        }

        fix_edge::<T>(&edge, &twin, self);
        fix_edge::<T>(&twin, &edge, self);

        debug_assert!(edge.prev_id() == twin.id() || self.edge(edge.prev_id()).check().is_ok());
        debug_assert!(edge.next_id() == twin.id() || self.edge(edge.next_id()).check().is_ok());
        debug_assert!(twin.prev_id() == edge.id() || self.edge(twin.prev_id()).check().is_ok());
        debug_assert!(twin.next_id() == edge.id() || self.edge(twin.next_id()).check().is_ok());

        true
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

        // update faces to match the previous edges
        let f1 = self.edge(e1).prev().load()?.face_id();
        if f1 != IndexType::max() {
            self.edge_mut(e1).load()?.set_face(f1);
        }
        let f2 = self.edge(e2).prev().load()?.face_id();
        if f2 != IndexType::max() {
            self.edge_mut(e2).load()?.set_face(f2);
        }

        debug_assert_eq!(self.edge(e1).check(), Ok(()));
        debug_assert_eq!(self.edge(e2).check(), Ok(()));

        Some(e1)
    }

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
                debug_assert_eq!(self.edge(e1).check(), Ok(()));
                debug_assert_eq!(self.edge(e2).check(), Ok(()));
                return Some(e1);
            } else {
                // a is isolated, b is not isolated

                // find a unique boundary edge
                let Some(b_in) = bv.ingoing_boundary_edge() else {
                    return None;
                };
                let face = self.edge(b_in).load()?.face_id();
                let (e1, e2) = self.insert_halfedge_pair_forced(
                    IndexType::max(),
                    a,
                    IndexType::max(),
                    b_in,
                    b,
                    self.edge(b_in).load()?.next_id(),
                    face,
                    face,
                    ep,
                );
                self.vertex_mut(a).set_edge(e1);
                debug_assert_eq!(self.edge(e1).check(), Ok(()));
                debug_assert_eq!(self.edge(e2).check(), Ok(()));
                return Some(e1);
            }
        } else if bv.is_isolated() {
            // a is not isolated, b is isolated

            // find a unique boundary edge
            let Some(a_in) = av.ingoing_boundary_edge() else {
                return None;
            };
            let face = self.edge(a_in).load()?.face_id();
            let next = self.edge_ref(a_in).next_id();
            let (e1, e2) = self.insert_halfedge_pair_forced(
                a_in,
                a,
                next,
                IndexType::max(),
                b,
                IndexType::max(),
                face,
                face,
                ep,
            );
            self.vertex_mut(b).set_edge(e2);
            self.edge_mut(a_in).load()?.set_next(e1);
            self.edge_mut(next).load()?.set_prev(e2);

            debug_assert_eq!(self.edge(e1).check(), Ok(()));
            debug_assert_eq!(self.edge(e2).check(), Ok(()));
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

            if a == b {
                // TODO: We currently don't support self-loops
                return None;
            }

            let (to_a, from_b, _) = bv.load()?.shortest_path(a)?;
            debug_assert_eq!(self.edge(to_a).target_id(), a);
            debug_assert_eq!(self.edge(from_b).origin_id(), b);
            return self.insert_edge_ee(to_a, from_b, ep);
        }
    }

    #[inline]
    fn insert_edge_ev(&mut self, e: T::E, v: T::V, ep: T::EP) -> Option<T::E> {
        if self.vertex(v).is_isolated() {
            // Trivial case where the connectivity is already given
            let edge = self.edge(e).load()?;
            let origin = edge.target_id();
            let fo = edge.next_id();
            let face = edge.face_id();
            let (e1, e2) = self.insert_halfedge_pair_forced(
                e,
                origin,
                fo,
                IndexType::max(),
                v,
                IndexType::max(),
                face,
                face,
                ep,
            );
            self.edge_mut(e).load()?.set_next(e1);
            self.edge_mut(fo).load()?.set_prev(e2);
            self.vertex_mut(v).set_edge(e2);

            debug_assert_eq!(self.edge(e1).check(), Ok(()));
            debug_assert_eq!(self.edge(e2).check(), Ok(()));
            debug_assert_eq!(self.edge(e).check(), Ok(()));
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
        if let Some(outgoing) = self.edge(e).same_boundary_back(v) {
            return self.insert_edge_ee(e, outgoing.id()?, ep);
        }

        None
    }

    #[inline]
    fn try_remove_face(&mut self, f: T::F) -> bool {
        let face = self.face(f);
        if face.is_void() {
            return false;
        }
        let e = self.edge_ref(face.edge_id()).clone();
        self.faces.delete(f);
        e.boundary_mut(self).for_each(|e| e.remove_face());
        true
    }

    #[inline]
    fn insert_face(&mut self, e: T::E, fp: T::FP) -> Option<T::F> {
        if self.edge(e).load()?.has_face() {
            return None;
        }
        Some(self.insert_face_forced(e, fp))
    }

    #[inline]
    fn insert_face_forced(&mut self, e: T::E, fp: T::FP) -> T::F {
        let f = self.faces.push(HalfEdgeFaceImpl::new(e, fp));
        self.edge_ref(e)
            .clone()
            .boundary_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }

    fn subdivide_edge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E {
        let edge = self.edge_ref(e).clone();
        let twin = self.edge_ref(edge.twin_id()).clone();
        let target = twin.origin_id(self);

        let origin = self.insert_vertex(vp);
        let (new_e, new_t) = self.insert_halfedge_pair_forced(
            edge.id(),
            origin,
            twin.id(),
            twin.prev_id(),
            target,
            edge.next_id(),
            edge.face_id(),
            twin.face_id(),
            ep,
        );

        // TODO: Remove the unwraps and use load instead

        self.vertex_mut(origin).set_edge(new_e);
        self.vertex_mut(target).set_edge(new_t);
        self.edge_mut(edge.next_id()).unwrap().set_prev(new_e);
        self.edge_mut(twin.prev_id()).unwrap().set_next(new_t);
        self.edge_mut(edge.id()).unwrap().set_next(new_e);
        self.edge_mut(twin.id()).unwrap().set_prev(new_t);
        self.edge_mut(twin.id()).unwrap().set_origin(origin);

        debug_assert_eq!(self.edge(new_e).check(), Ok(()));
        debug_assert_eq!(self.edge(new_t).check(), Ok(()));

        new_e
    }

    fn collapse_edge(&mut self, edge_id: T::E) -> Option<T::E> {
        let edge = self.edge(edge_id).load()?;
        let twin: ValidEdgeCursor<T> = edge.fork().twin().load()?;
        let next = edge.fork().next().load()?;
        let next_twin = next.fork().twin().load()?;

        let removed_v = twin.origin_id();
        let target = next_twin.origin_id();
        let next_twin_id = next_twin.id();
        let next_id = next.id();
        let next_next_id = next.next_id();
        let next_twin_prev_id = next_twin.prev_id();
        let twin_id = twin.id();

        if next_id == twin_id {
            // The target vertex has degree 1, so we cannot collapse the edge
            return None;
        }
        if next_twin.next_id() != twin_id {
            // The target vertex has degree > 2, so we cannot collapse the edge
            return None;
        }
        if next_next_id == edge_id {
            // The edge is parallel to edge_id.
            let face = edge.face_id();
            let twin_face = twin.face_id();

            // swapped, because `v` won't be removed
            self.vertex_mut(target).set_edge(edge_id);
            self.vertex_mut(removed_v).set_edge(twin_id);

            self.edge_mut(edge_id).load()?.link(twin_id);
            if next_twin_prev_id == twin_id {
                // both parallel
                self.edge_mut(twin_id).load()?.link(edge_id);
            }

            // The enclosed face must be removed
            if face != IndexType::max() {
                self.faces.delete(face);
            }
            self.edge_mut(edge_id).replace_face(twin_face);
            self.halfedges.delete(next_id);
            self.halfedges.delete(next_twin_id);
        } else if next_twin_prev_id == twin_id {
            // also a parallel edge; mirrored case of the above
            todo!();
            // TODO: Be aware, that there are three types of parallel-edges:
            // 1): edge.next = edge and twin.prev = twin
            // 2): edge.next = edge but twin.prev != twin
            // 3): edge.next != edge but twin.prev = twin
        } else if next_next_id == twin_id {
            // This is a self-loop
            // TODO: test this and other functions with self-loops and parallel edges
            todo!();
        } else {
            // TODO: Make sure the loads cannot cause invalid intermediate states
            self.vertex_mut(target).set_edge(twin_id);
            self.edge_mut(twin_id).load()?.set_origin(target);
            self.edge_mut(edge_id).load()?.link(next_next_id);
            self.edge_mut(next_twin_prev_id).load()?.link(twin_id);

            self.vertices.delete(removed_v);
            self.halfedges.delete(next_id);
            self.halfedges.delete(next_twin_id);
        }

        debug_assert_eq!(self.edge(edge_id).check(), Ok(()));
        debug_assert_eq!(self.edge(twin_id).check(), Ok(()));
        Some(edge_id)
    }

    #[inline]
    fn subdivide_face(&mut self, from: T::E, to: T::E, ep: T::EP, fp: T::FP) -> Option<T::E> {
        let f = self.edge(from).load()?.face_id();
        if f == IndexType::max() || f != self.edge(to).load()?.face_id() {
            return None;
        }
        if self.edge(from).target_id() == self.edge(to).origin_id() {
            // won't create self-loops
            // TODO: Actually, we should allow this.
            return None;
        }
        let new_e = self.insert_edge_ee(from, to, ep)?;
        self.insert_face_forced(new_e, fp);
        let new_twin = self.edge(new_e).load()?.twin_id();
        self.face_mut(f).set_edge(new_twin);
        Some(new_e)
    }

    #[inline]
    fn subdivide_face_v(
        &mut self,
        f: T::F,
        v: T::V,
        w: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<T::E> {
        if v == w {
            return None;
        }
        let Ok(to) = self
            .vertex(w)
            .edges_out()
            .filter(|e| e.face_id() == f)
            .exactly_one()
        else {
            return None;
        };
        let Ok(from) = self
            .vertex(v)
            .edges_in()
            .filter(|e| e.face_id() == f)
            .exactly_one()
        else {
            return None;
        };

        self.subdivide_face(from.id(), to.id(), ep, fp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};
    use itertools::Itertools;

    fn vp(x: f64, y: f64, z: f64) -> VertexPayloadPNU<f64, 3> {
        VertexPayloadPNU::<f64, 3>::from_pos(Vec3::<f64>::new(x, y, z))
    }

    fn sorted<T: Ord, I: IntoIterator<Item = T>>(v: I) -> Vec<T> {
        let mut v = v.into_iter().collect_vec();
        v.sort_unstable();
        v
    }

    #[test]
    fn test_remove_vertex() {
        let mut mesh = Mesh3d64::default();
        let v1 = mesh.insert_vertex(vp(0.0, 0.0, 0.0));
        let v2 = mesh.insert_vertex(vp(1.0, 0.0, 0.0));
        let e12 = mesh.insert_edge_vv(v1, v2, Default::default()).unwrap();
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 2);
        assert_eq!(mesh.num_halfedges(), 2);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);

        // should fail before removing the edge
        let mesh_clone = mesh.clone();
        assert_eq!(mesh.try_remove_vertex(v1), false);
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        mesh.remove_edge(e12);
        assert_eq!(mesh.try_remove_vertex(v1), true);
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 1);
        assert_eq!(mesh.num_halfedges(), 0);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v2).edge_id(), IndexType::max());
        assert_eq!(mesh.vertex(v2).neighbors().count(), 0);

        // should fail when trying to remove again
        let mesh_clone = mesh.clone();
        assert_eq!(mesh.try_remove_vertex(v1), false);
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        assert_eq!(mesh.try_remove_vertex(v2), true);
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 0);
        assert_eq!(mesh.num_halfedges(), 0);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
    }

    #[test]
    fn test_remove_edge() {
        let mut mesh = Mesh3d64::default();
        let e0 = mesh.insert_regular_polygon(1.0, 3).id();
        assert_eq!(mesh.is_open_2manifold(), true);
        let (e1, _v1) = mesh
            .insert_vertex_e(e0, vp(42.0, 0.0, 0.0), Default::default())
            .unwrap();
        let (e2, _v2) = mesh
            .insert_vertex_e(e1, vp(142.0, 0.0, 0.0), Default::default())
            .unwrap();
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_halfedges(), 10);
        assert_eq!(mesh.num_faces(), 1);
        assert_eq!(mesh.is_connected(), true);

        // should fail before removing the face
        let mesh_clone = mesh.clone();
        assert_eq!(mesh.try_remove_edge(e0), false);
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        assert_eq!(mesh.try_remove_edge(e1), true);
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_halfedges(), 8);
        assert_eq!(mesh.num_faces(), 1);
        assert_eq!(mesh.is_connected(), false);

        // should fail after being already removed
        let mesh_clone = mesh.clone();
        assert_eq!(mesh.try_remove_edge(e1), false);
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        assert_eq!(mesh.try_remove_edge(e2), true);
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_halfedges(), 6);
        assert_eq!(mesh.num_faces(), 1);
        assert_eq!(mesh.is_connected(), false);

        let f = mesh.face_ids().next().unwrap();
        mesh.remove_face(f);
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 5);
    }

    #[test]
    fn test_insert_vertex() {
        let mut mesh = Mesh3d64::default();
        let vp1 = vp(42.0, 42.0, 42.0);
        let v1 = mesh.insert_vertex(vp1);
        assert_eq!(v1, IndexType::new(0));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 1);
        assert_eq!(mesh.num_halfedges(), 0);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(*mesh.vertex(v1).unwrap().payload(), vp1);
        assert_eq!(mesh.vertex(v1).edge_id(), IndexType::max());
        assert_eq!(mesh.vertex(v1).neighbors().count(), 0);
        assert_eq!(mesh.is_connected(), true);

        let vp2 = vp(0.0, 0.0, 0.0);
        let v2 = mesh.insert_vertex(vp2);
        assert_eq!(v2, IndexType::new(1));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 2);
        assert_eq!(mesh.num_halfedges(), 0);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(*mesh.vertex(v2).unwrap().payload(), vp2);
        assert_eq!(mesh.vertex(v2).edge_id(), IndexType::max());
        assert_eq!(mesh.vertex(v2).neighbors().count(), 0);
        assert_eq!(mesh.is_connected(), false);

        let e12 = mesh.insert_edge_vv(v1, v2, Default::default()).unwrap();
        assert_eq!(
            sorted([e12, mesh.edge(e12).unwrap().twin_id()]),
            vec![IndexType::new(0), IndexType::new(1)]
        );
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 2);
        assert_eq!(mesh.num_halfedges(), 2);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v2).neighbor_ids().collect_vec(), vec![v1]);
        assert_eq!(mesh.vertex(v1).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(mesh.edge(e12).origin_id(), v1);
        assert_eq!(mesh.edge(e12).target_id(), v2);

        let vp3 = vp(1.0, 0.0, 0.0);
        let (e23, v3) = mesh.insert_vertex_v(v2, vp3, Default::default()).unwrap();
        assert_eq!(
            sorted([e23, mesh.edge(e23).unwrap().twin_id()]),
            vec![IndexType::new(2), IndexType::new(3)]
        );
        assert_eq!(v3, IndexType::new(2));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 3);
        assert_eq!(mesh.num_halfedges(), 4);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v3).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(sorted(mesh.vertex(v2).neighbor_ids()), sorted([v1, v3]));
        assert_eq!(mesh.vertex(v1).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(mesh.edge(e23).origin_id(), v2);
        assert_eq!(mesh.edge(e23).target_id(), v3);

        let e31 = mesh.insert_edge_vv(v3, v1, Default::default()).unwrap();
        assert_eq!(
            sorted([e31, mesh.edge(e31).unwrap().twin_id()]),
            vec![IndexType::new(4), IndexType::new(5)]
        );
        assert_eq!(mesh.check(), Ok(()));
        //assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 3);
        assert_eq!(mesh.num_halfedges(), 6);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(sorted(mesh.vertex(v3).neighbor_ids()), sorted([v2, v1]));
        assert_eq!(sorted(mesh.vertex(v2).neighbor_ids()), sorted([v1, v3]));
        assert_eq!(sorted(mesh.vertex(v1).neighbor_ids()), sorted([v2, v3]));
        assert_eq!(mesh.edge(e31).origin_id(), v3);
        assert_eq!(mesh.edge(e31).target_id(), v1);

        let es = [e12, e23, e31];
        for e in es.iter() {
            // No matter which edge we use to insert the face, the result should be the same
            let mut mesh = mesh.clone();
            let f = mesh.insert_face(*e, Default::default()).unwrap();
            assert_eq!(f, IndexType::new(0));
            assert_eq!(mesh.check(), Ok(()));
            assert_eq!(mesh.is_open_2manifold(), true);
            assert_eq!(mesh.has_holes(), true);
            assert_eq!(mesh.num_vertices(), 3);
            assert_eq!(mesh.num_halfedges(), 6);
            assert_eq!(mesh.num_faces(), 1);
            assert_eq!(sorted(mesh.face(f).edge_ids()), sorted([e12, e23, e31]));
            assert_eq!(sorted(mesh.face(f).vertex_ids()), sorted([v1, v2, v3]));
            assert_eq!(mesh.face(f).edge_id(), *e);
            assert_eq!(mesh.is_connected(), true);
            assert_eq!(mesh.face(f).edge_id(), *e);
            for e in es.iter() {
                assert_eq!(mesh.edge(*e).unwrap().face_id(), f);
                assert_eq!(mesh.edge(*e).twin().unwrap().has_face(), false);
            }

            // the edges should be in the correct order
            let es = mesh.face(f).edge_ids().collect_vec();
            for i in 0..es.len() {
                assert_eq!(
                    mesh.edge(es[i]).target_id(),
                    mesh.edge(es[(i + 1) % es.len()]).origin_id()
                );
                assert_eq!(mesh.edge(es[i]).unwrap().next_id(), es[(i + 1) % es.len()]);
            }
        }
    }

    #[test]
    fn test_insert_edge() {
        let mut mesh = Mesh3d64::default();
        let e12 =
            mesh.insert_isolated_edge(vp(0.0, 0.0, 0.0), vp(1.0, 0.0, 0.0), Default::default());
        let v1 = mesh.vertex(mesh.edge(e12).origin_id()).id().unwrap();
        let v2 = mesh.vertex(mesh.edge(e12).target_id()).id().unwrap();
        assert_eq!(
            sorted([e12, mesh.edge(e12).unwrap().twin_id()]),
            vec![IndexType::new(0), IndexType::new(1)]
        );
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 2);
        assert_eq!(mesh.num_edges(), 1);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v1).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(mesh.vertex(v2).neighbor_ids().collect_vec(), vec![v1]);

        let v3 = mesh.insert_vertex(vp(1.0, 1.0, 0.0));
        assert_eq!(v3, IndexType::new(2));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 3);
        assert_eq!(mesh.num_edges(), 1);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), false);
        assert_eq!(mesh.vertex(v3).neighbors().count(), 0);

        let e23 = mesh.insert_edge_ev(e12, v3, Default::default()).unwrap();
        assert_eq!(
            sorted([e23, mesh.edge(e23).unwrap().twin_id()]),
            vec![IndexType::new(2), IndexType::new(3)]
        );
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 3);
        assert_eq!(mesh.num_edges(), 2);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v3).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(sorted(mesh.vertex(v2).neighbor_ids()), sorted([v1, v3]));
        assert_eq!(mesh.vertex(v1).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(mesh.edge(e23).origin_id(), v2);
        assert_eq!(mesh.edge(e23).target_id(), v3);

        let (e34, v4) = mesh
            .insert_vertex_e(e23, vp(1.0, 1.0, 1.0), Default::default())
            .unwrap();
        assert_eq!(
            sorted([e34, mesh.edge(e34).unwrap().twin_id()]),
            vec![IndexType::new(4), IndexType::new(5)]
        );
        assert_eq!(v4, IndexType::new(3));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 4);
        assert_eq!(mesh.num_edges(), 3);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v4).neighbor_ids().collect_vec(), vec![v3]);
        assert_eq!(sorted(mesh.vertex(v3).neighbor_ids()), sorted([v2, v4]));
        assert_eq!(sorted(mesh.vertex(v2).neighbor_ids()), sorted([v1, v3]));
        assert_eq!(mesh.edge(e34).origin_id(), v3);
        assert_eq!(mesh.edge(e34).target_id(), v4);

        let (e25, v5) = mesh
            .insert_vertex_e(e12, vp(0.0, 1.0, 0.0), Default::default())
            .unwrap();
        let e52 = mesh.edge(e25).unwrap().twin_id();
        assert_eq!(
            sorted([e25, e52]),
            vec![IndexType::new(6), IndexType::new(7)]
        );
        assert_eq!(v5, IndexType::new(4));
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_open_2manifold(), false);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_edges(), 4);
        assert_eq!(mesh.num_faces(), 0);
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.vertex(v5).neighbor_ids().collect_vec(), vec![v2]);
        assert_eq!(sorted(mesh.vertex(v1).neighbor_ids()), sorted([v2]));
        assert_eq!(sorted(mesh.vertex(v2).neighbor_ids()), sorted([v1, v3, v5]));
        assert_eq!(mesh.edge(e25).origin_id(), v2);
        assert_eq!(mesh.edge(e25).target_id(), v5);

        // connect based on vertices
        {
            let mut mesh = mesh.clone();
            let (e45, f) = mesh
                .close_face_vvv(v3, v4, v5, Default::default(), Default::default())
                .unwrap();
            assert_eq!(f, IndexType::new(0));
            assert_eq!(mesh.check(), Ok(()));
            assert_eq!(mesh.is_open_2manifold(), false);
            assert_eq!(mesh.num_vertices(), 5);
            assert_eq!(mesh.num_edges(), 5);
            assert_eq!(mesh.num_faces(), 1);
            assert_eq!(mesh.is_connected(), true);
            assert_eq!(mesh.face(f).edge_id(), e45);
            let es = vec![e45, e52, e23, e34];
            assert_eq!(mesh.face(f).edge_ids().collect_vec(), es);
            for e in es.iter() {
                assert_eq!(mesh.edge(*e).unwrap().face_id(), f);
                assert_eq!(mesh.edge(*e).twin().unwrap().has_face(), false);
            }
        }

        // connect based on edges
        {
            let mut mesh = mesh.clone();
            let (e45, f) = mesh
                .close_face_ee(e34, e52, Default::default(), Default::default())
                .unwrap();
            assert_eq!(f, IndexType::new(0));
            assert_eq!(mesh.check(), Ok(()));
            assert_eq!(mesh.is_open_2manifold(), false);
            assert_eq!(mesh.num_vertices(), 5);
            assert_eq!(mesh.num_edges(), 5);
            assert_eq!(mesh.num_faces(), 1);
            assert_eq!(mesh.is_connected(), true);
            assert_eq!(mesh.face(f).edge_id(), e45);
            let es = vec![e45, e52, e23, e34];
            assert_eq!(mesh.face(f).edge_ids().collect_vec(), es);
            for e in es.iter() {
                assert_eq!(mesh.edge(*e).unwrap().face_id(), f);
                assert_eq!(mesh.edge(*e).twin().unwrap().has_face(), false);
            }
        }

        // insert some non-manifold edges to make things complicated
        {
            let mut mesh = mesh.clone();
            let v6 = mesh.insert_vertex(vp(0.0, 1.0, 1.0));
            assert_eq!(mesh.is_connected(), false);
            let mesh_copy = mesh.clone();
            // inserting this based on vertices is ambiguous - it should fail without changing anything
            assert_eq!(mesh.insert_edge_vv(v2, v6, Default::default()), None);
            assert!(mesh.is_trivially_isomorphic(&mesh_copy).eq());
            assert_eq!(mesh.check(), Ok(()));

            // inserting this based on edges is not ambiguous
            let e26 = mesh.insert_edge_ev(e52, v6, Default::default()).unwrap();
            assert_eq!(mesh.check(), Ok(()));
            assert_eq!(mesh.is_open_2manifold(), false);
            assert_eq!(mesh.num_vertices(), 6);
            assert_eq!(mesh.num_edges(), 5);
            assert_eq!(mesh.num_faces(), 0);
            assert_eq!(mesh.is_connected(), true);
            assert_eq!(mesh.vertex(v6).neighbor_ids().collect_vec(), vec![v2]);
            assert_eq!(
                sorted(mesh.vertex(v2).neighbor_ids()),
                sorted([v1, v3, v5, v6])
            );
            assert_eq!(mesh.edge(e26).origin_id(), v2);
            assert_eq!(mesh.edge(e26).target_id(), v6);

            {
                // we now have a mesh like this:
                //       v5
                //        |
                // v1 -- v2 -- v6
                //        |
                //       v3 -- v4

                {
                    // let's make a face v2-v3-v4 using different methods

                    // All the different faces should be able to make this connection
                    let mut m = vec![mesh.clone(), mesh.clone(), mesh.clone(), mesh.clone()];

                    let res = vec![
                        m[0].close_face_vvv(v3, v4, v2, Default::default(), Default::default())
                            .unwrap(),
                        m[1].close_face_vv(v4, v2, Default::default(), Default::default())
                            .unwrap(),
                        m[2].close_face_ev(e34, v2, Default::default(), Default::default())
                            .unwrap(),
                        m[3].close_face_ee(e34, e23, Default::default(), Default::default())
                            .unwrap(),
                    ];

                    for i in 0..4 {
                        let (e42, f) = res[i];
                        assert_eq!(f, IndexType::new(0));
                        assert_eq!(m[i].edge(e42).target_id(), v2);
                        assert_eq!(m[i].edge(e42).origin_id(), v4);
                        assert_eq!(m[i].check(), Ok(()));
                        assert!(!m[i].is_open_2manifold());
                        assert!(m[i].is_trivially_isomorphic(&m[0]).eq());
                        assert_eq!(m[i].num_vertices(), 6);
                        assert_eq!(m[i].num_edges(), 6);
                        assert_eq!(m[i].num_faces(), 1);
                        assert_eq!(m[i].face(f).edge_id(), e42);
                        let es = vec![e42, e23, e34];
                        assert_eq!(m[i].face(f).edge_ids().collect_vec(), es);
                        for e in es.iter() {
                            assert_eq!(m[i].edge(*e).unwrap().face_id(), f);
                            assert_eq!(m[i].edge(*e).twin().unwrap().has_face(), false);
                        }

                        // inserting again should fail (duplicate face)
                        let cloned = m[i].clone();
                        assert_eq!(
                            m[i].close_face_vv(v4, v2, Default::default(), Default::default()),
                            None
                        );
                        assert!(m[i].is_trivially_isomorphic(&cloned).eq());

                        // inserting the other way around should fail because the edge already exists
                        // and connectivity is unclear
                        let mut cloned = m[i].clone();
                        assert_eq!(cloned.insert_edge_vv(v2, v4, Default::default()), None);
                        assert_eq!(
                            cloned.close_face_vv(v2, v4, Default::default(), Default::default()),
                            None
                        );
                        assert!(cloned.is_trivially_isomorphic(&m[i]).eq());

                        // However, with some explicit connectivity info, we can still construct the backface
                        let e32 = cloned.edge(e23).unwrap().twin_id();
                        let e43 = cloned.edge(e34).unwrap().twin_id();
                        let (e24, backface) = cloned
                            .close_face_ee(e32, e43, Default::default(), Default::default())
                            .unwrap();
                        assert_eq!(backface, IndexType::new(1));
                        assert_eq!(cloned.check(), Ok(()));
                        assert!(cloned.is_trivially_isomorphic(&m[i]).ne());
                        assert_eq!(cloned.num_vertices(), 6);
                        assert_eq!(cloned.num_edges(), 7);
                        assert_eq!(cloned.num_faces(), 2);
                        assert_eq!(cloned.face(backface).edge_id(), e24);
                        let es = vec![e24, e43, e32];
                        assert_eq!(cloned.face(backface).edge_ids().collect_vec(), es);
                        // notice that there are multiple edges between v2 and v4 now
                        assert_eq!(cloned.shared_edges(v2, v4).count(), 2);
                    }
                }

                {
                    // We can make a degenerate face v4-v6-v2-v5-v2-v3-v4. This is fine!
                    let mut mesh = mesh.clone();
                    let e52 = mesh.edge(e25).unwrap().twin_id();
                    let (e45, f1) = mesh
                        .close_face_ee(e34, e52, Default::default(), Default::default())
                        .unwrap();
                    assert_eq!(f1, IndexType::new(0));
                    assert_eq!(mesh.check(), Ok(()));
                    assert_eq!(mesh.is_open_2manifold(), false);
                    assert_eq!(mesh.num_vertices(), 6);
                    assert_eq!(mesh.num_edges(), 6);
                    assert_eq!(mesh.num_faces(), 1);
                    assert_eq!(mesh.is_connected(), true);
                    assert_eq!(mesh.face(f1).edge_id(), e45);
                    let e62 = mesh.edge(e26).unwrap().twin_id();
                    let es = vec![e45, e52, e26, e62, e23, e34];
                    assert_eq!(mesh.face(f1).edge_ids().collect_vec(), es);

                    // this would be a duplicate face
                    assert_eq!(
                        mesh.close_face_ee(e23, e52, Default::default(), Default::default()),
                        None
                    );

                    // another degenerate face: v5-v3-v2-v1-v2-v5
                    let e32 = mesh.edge(e23).unwrap().twin_id();
                    let (e53, f2) = mesh
                        .close_face_ee(e25, e32, Default::default(), Default::default())
                        .unwrap();
                    assert_eq!(f2, IndexType::new(1));
                    assert_eq!(mesh.check(), Ok(()));
                    assert_eq!(mesh.is_open_2manifold(), false);
                    assert_eq!(mesh.num_vertices(), 6);
                    assert_eq!(mesh.num_edges(), 7);
                    assert_eq!(mesh.num_faces(), 2);
                    assert_eq!(mesh.is_connected(), true);
                    assert_eq!(mesh.face(f2).edge_id(), e53);
                    let e21 = mesh.edge(e12).unwrap().twin_id();
                    let es = vec![e53, e32, e21, e12, e25];
                    assert_eq!(mesh.face(f2).edge_ids().collect_vec(), es);

                    // let's make it manifold by removing v6 and v1
                    mesh.remove_face(f1);
                    assert_eq!(mesh.check(), Ok(()));
                    mesh.remove_face(f2);
                    assert_eq!(mesh.check(), Ok(()));
                    mesh.remove_edge(e26);
                    assert_eq!(mesh.check(), Ok(()));
                    mesh.remove_vertex(v6);
                    assert_eq!(mesh.check(), Ok(()));
                    mesh.remove_edge(e21);
                    assert_eq!(mesh.check(), Ok(()));
                    mesh.remove_vertex(v1);
                    assert_eq!(mesh.check(), Ok(()));
                    assert_eq!(mesh.is_open_2manifold(), false);

                    let f1 = mesh.insert_face(e45, Default::default()).unwrap();

                    assert_eq!(mesh.check(), Ok(()));
                    // still non-manifold because of e53 still being there
                    assert_eq!(mesh.is_open_2manifold(), false);
                    {
                        let mut mesh = mesh.clone();
                        mesh.remove_edge(e53);
                        assert_eq!(mesh.check(), Ok(()));
                        assert_eq!(mesh.is_open_2manifold(), true);
                    }

                    let f2 = mesh.insert_face(e53, Default::default()).unwrap();

                    assert_eq!(mesh.check(), Ok(()));
                    assert_eq!(mesh.is_open_2manifold(), true);
                    assert_eq!(mesh.num_vertices(), 4);
                    assert_eq!(mesh.num_edges(), 5);
                    assert_eq!(mesh.num_faces(), 2);
                    assert_eq!(mesh.is_connected(), true);
                    assert_eq!(mesh.has_vertex(v1), false);
                    assert_eq!(mesh.has_vertex(v6), false);
                    assert_eq!(mesh.has_edge(e21), false);
                    assert_eq!(mesh.has_edge(e26), false);
                    let es = vec![e45, e52, e23, e34];
                    assert_eq!(mesh.face(f1).edge_ids().collect_vec(), es);
                    let es = vec![e53, e32, e25];
                    assert_eq!(mesh.face(f2).edge_ids().collect_vec(), es);
                }
            }
        }
    }

    #[test]
    fn test_subdivide_edge() {
        let mut mesh = Mesh3d64::default();
        let e0 = mesh.insert_regular_polygon(1.0, 3).id();
        assert_eq!(mesh.check(), Ok(()));
        let f = mesh.edge(e0).twin().unwrap().face_id();

        let e1 = mesh.subdivide_edge(e0, vp(0.0, 0.0, 0.0), Default::default());
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 4);
        assert_eq!(mesh.num_edges(), 4);
        assert_eq!(mesh.num_faces(), 1);
        assert_eq!(mesh.face(f).unwrap().num_edges(), 4);

        let e2 = mesh.subdivide_edge(e0, vp(0.1, 0.0, 0.0), Default::default());
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_edges(), 5);
        assert_eq!(mesh.num_faces(), 1);
        assert_eq!(mesh.face(f).unwrap().num_edges(), 5);

        {
            let mut mesh = mesh.clone();
            for i in 1..=4 {
                let e0b = mesh.collapse_edge(e0).unwrap();
                assert_eq!(e0b, e0);
                assert_eq!(mesh.has_edge(e0), true);
                assert_eq!(mesh.has_edge(e1), i == 1);
                assert_eq!(mesh.has_edge(e2), false);
                if 5 - i >= 2 {
                    assert_eq!(mesh.check(), Ok(()));
                }
                assert_eq!(mesh.is_connected(), true);
                assert_eq!(mesh.is_open_2manifold(), 5 - i >= 2);
                assert_eq!(mesh.num_vertices(), (5 - i).max(2));
                assert_eq!(mesh.num_edges(), 5 - i);
                assert_eq!(mesh.num_faces(), 1);
            }
            assert_eq!(mesh.num_vertices(), 2);
            assert_eq!(mesh.num_edges(), 1);
            assert_eq!(mesh.num_halfedges(), 2);
            // TODO: verify existence of parallel edges
            // TODO: Verify other types of self loops

            let mesh_clone = mesh.clone();
            assert_eq!(mesh.collapse_edge(e0), None);
            assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);
        }

        /*
        let mesh_clone = mesh.clone();
        assert_eq!(
            mesh.subdivide_face(e1, e1, Default::default(), Default::default()),
            None
        );
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);*/
    }

    #[test]
    fn test_subdivide_face() {
        let mut mesh = Mesh3d64::default();
        let e0b = mesh.insert_regular_polygon(1.0, 4).id();
        let e0 = mesh.edge(e0b).unwrap().twin_id();
        assert_eq!(mesh.check(), Ok(()));

        let e1 = mesh.edge(e0).unwrap().next_id();
        let e2 = mesh.edge(e1).unwrap().next_id();
        let e3 = mesh.edge(e2).unwrap().next_id();

        let d1a = mesh
            .subdivide_face(e0, e3, Default::default(), Default::default())
            .unwrap();
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 4);
        assert_eq!(mesh.num_edges(), 5);
        assert_eq!(mesh.num_faces(), 2);

        let d1bt = mesh.subdivide_edge(d1a, vp(0.0, 0.0, 0.0), Default::default());
        let d1at = mesh.edge(d1bt).twin().unwrap().next_id();
        assert_ne!(d1a, d1bt);
        assert_eq!(d1a, mesh.edge(d1at).unwrap().twin_id());

        assert_eq!(mesh.num_vertices(), 5);

        let d2a = mesh
            .subdivide_face(e3, d1bt, Default::default(), Default::default())
            .unwrap();
        let d2b = mesh
            .subdivide_face(e1, d1at, Default::default(), Default::default())
            .unwrap();

        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_edges(), 8);
        assert_eq!(mesh.num_faces(), 4);

        let center = mesh.edge(d1bt).origin_id();
        assert_eq!(mesh.edge(d2a).target_id(), center);
        assert_eq!(mesh.edge(d2b).target_id(), center);
        assert_eq!(mesh.edge(d1a).target_id(), center);
        assert_eq!(mesh.edge(d1bt).origin_id(), center);
        assert_eq!(mesh.edge(d1at).origin_id(), center);

        for f in mesh.faces() {
            assert_eq!(f.num_edges(), 3);
        }

        let mesh_clone = mesh.clone();
        // not the same face
        assert_eq!(
            mesh.subdivide_face(e0, e1, Default::default(), Default::default()),
            None
        );
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        // still not the same face
        assert_eq!(
            mesh.subdivide_face(e0, e2, Default::default(), Default::default()),
            None
        );
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        // too small
        assert_eq!(
            mesh.subdivide_face(
                e0,
                mesh.edge(e0).unwrap().next_id(),
                Default::default(),
                Default::default()
            ),
            None
        );
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        // subdiving the outside is also not possible
        assert_eq!(
            mesh.subdivide_face(
                mesh.edge(e0).unwrap().twin_id(),
                mesh.edge(e3).unwrap().twin_id(),
                Default::default(),
                Default::default(),
            ),
            None
        );
        assert_eq!(mesh.is_trivially_isomorphic(&mesh_clone).eq(), true);

        // though, we can insert an outside face:
        mesh.insert_face(mesh.edge(e0).unwrap().twin_id(), Default::default())
            .unwrap();
        assert_eq!(mesh.check(), Ok(()));
        let _d3 = mesh
            .subdivide_face(
                mesh.edge(e0).unwrap().twin_id(),
                mesh.edge(e1).unwrap().twin_id(),
                Default::default(),
                Default::default(),
            )
            .unwrap();

        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(mesh.is_connected(), true);
        assert_eq!(mesh.is_open_2manifold(), true);
        assert_eq!(mesh.num_vertices(), 5);
        assert_eq!(mesh.num_edges(), 9);
        assert_eq!(mesh.num_faces(), 6);

        for f in mesh.faces() {
            assert_eq!(f.num_edges(), 3);
        }
    }
}
