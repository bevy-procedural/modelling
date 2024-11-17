use crate::{
    halfedge::{HalfEdgeFaceImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl},
    math::IndexType,
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, FaceBasics, HalfEdge, HalfEdgeVertex,
        MeshBasics, MeshBuilder, MeshHalfEdgeBuilder,
    },
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

impl<T: HalfEdgeImplMeshType> MeshBuilder<T> for HalfEdgeMeshImpl<T> {
    fn add_vertex_via_vertex_default(&mut self, v: T::V, vp: T::VP) -> (T::V, T::E, T::E)
    where
        T::EP: DefaultEdgePayload,
    {
        self.add_vertex_via_vertex(v, vp, T::EP::default(), T::EP::default())
    }

    fn add_vertex_via_edge_default(
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

    fn insert_vertices_into_edge<I: Iterator<Item = (T::EP, T::EP, T::VP)>>(
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

    fn add_vertex(&mut self, vp: T::VP) -> T::V {
        let new = self.vertices.allocate();
        self.vertices.set(new, T::Vertex::new(IndexType::max(), vp));
        new
    }
}
