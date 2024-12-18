use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    math::IndexType,
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, FaceBasics, HalfEdge,
        HalfEdgeSemiBuilder, HalfEdgeVertex, MeshBasics, MeshHalfEdgeBuilder,
        MeshType, VertexBasics,
    },
};
use itertools::Itertools;

// TODO: Simplify these

impl<T: HalfEdgeImplMeshType> MeshHalfEdgeBuilder<T> for HalfEdgeMeshImpl<T> {
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

        let (e1, e2) = self.insert_edge_no_update_no_check(
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
        let (e0, e1) = self.insert_edge_no_update_no_check(
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

        let (e0, e1) = self.insert_edge_no_update_no_check(
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

    fn insert_edge(&mut self, inside: T::E, ep1: T::EP, outside: T::E, ep2: T::EP) -> (T::E, T::E) {
        let e_inside = self.edge(inside);
        let e_outside = self.edge(outside);
        let v = e_inside.target(self).id();
        let w = e_outside.target(self).id();

        debug_assert!(e_inside.same_face_back(self, w));
        debug_assert!(e_outside.same_face_back(self, v));

        let other_inside = e_outside.next(self);
        let other_outside = e_inside.next(self);

        let (e1, e2) = self.insert_edge_no_update(
            (other_inside.id(), inside, v, IndexType::max(), ep1),
            (other_outside.id(), outside, w, IndexType::max(), ep2),
        );

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        debug_assert!(self.edge(e1).prev(self).next_id() == e1);
        debug_assert!(self.edge(e1).next(self).prev_id() == e1);
        debug_assert!(self.edge(e2).prev(self).next_id() == e2);
        debug_assert!(self.edge(e2).next(self).prev_id() == e2);

        (e1, e2)
    }

    #[inline(always)]
    fn insert_halfedge_no_update_no_check(
        &mut self,
        e: T::E,
        origin: T::V,
        face: T::F,
        prev: T::E,
        twin: T::E,
        next: T::E,
        payload: T::EP,
    ) {
        self.halfedges.set(
            e,
            HalfEdgeImpl::new(
                if next == IndexType::max() { twin } else { next },
                twin,
                if prev == IndexType::max() { twin } else { prev },
                origin,
                face,
                payload,
            ),
        );
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

    fn close_face_default(
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

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
    pub(crate) fn import_mesh<FE, FV, FF, FM, T2: MeshType>(
        mesh: &T2::Mesh,
        fv: FV,
        fe: FE,
        ff: FF,
        fm: FM
    ) -> Self
    where
        FE: Fn(&T2::EP) -> T::EP,
        FV: Fn(&T2::VP) -> T::VP,
        FF: Fn(&T2::FP) -> T::FP,
        FM: Fn(&T2::MP) -> T::MP,
        T2::Edge: HalfEdge<T2>,
    {
        let mut res = Self::default();
        let mut vertex_map = std::collections::HashMap::new();
        for vertex in MeshBasics::vertices(mesh) {
            let v = res.vertices.allocate();
            vertex_map.insert(vertex.id(), v);
        }
        let mut face_map = std::collections::HashMap::new();
        face_map.insert(IndexType::max(), IndexType::max());
        for face in MeshBasics::faces(mesh) {
            let f = res.faces.allocate();
            face_map.insert(face.id(), f);
        }
        let mut edge_map = std::collections::HashMap::new();
        for edge in MeshBasics::edges(mesh) {
            let e = res.halfedges.allocate();
            edge_map.insert(edge.id(), e);
        }

        for vertex in MeshBasics::vertices(mesh) {
            res.vertices.set(
                vertex_map[&vertex.id()],
                HalfEdgeVertexImpl::new(
                    edge_map[&VertexBasics::edge_id(vertex, mesh)],
                    fv(vertex.payload()),
                ),
            );
        }

        for face in MeshBasics::faces(mesh) {
            res.faces.set(
                face_map[&face.id()],
                HalfEdgeFaceImpl::new(
                    edge_map[&FaceBasics::edge_id(face)],
                    false,
                    ff(face.payload()),
                ),
            );
        }

        for edge in MeshBasics::edges(mesh) {
            res.insert_halfedge_no_update_no_check(
                edge_map[&edge.id()],
                vertex_map[&edge.origin_id()],
                face_map[&edge.face_id()],
                edge_map[&edge.prev_id()],
                edge_map[&edge.twin_id()],
                edge_map[&edge.next_id()],
                fe(&edge.payload()),
            );
        }

        res.set_payload(fm(MeshBasics::payload(mesh)));

        res
    }
}
