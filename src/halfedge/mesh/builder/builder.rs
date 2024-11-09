use crate::{
    halfedge::{HalfEdgeFaceImpl, HalfEdgeMeshImpl, HalfEdgeMeshType},
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, FaceBasics, HalfEdge, MeshBasics,
        MeshBuilder, MeshHalfEdgeBuilder,
    },
};

impl<T: HalfEdgeMeshType> MeshBuilder<T> for HalfEdgeMeshImpl<T> {
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

    
}
