use itertools::Itertools;

use crate::{
    halfedge::{HalfEdgeFace, HalfEdgeMesh, HalfEdgeMeshType},
    mesh::{DefaultEdgePayload, DefaultFacePayload, EdgeBasics, FaceBasics, Halfedge, MeshBasics},
};

// TODO: move more functions to the builder trait!

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    pub fn close_hole(&mut self, e: T::E, fp: T::FP, curved: bool) -> T::F {
        let f = self.faces.push(HalfEdgeFace::new(e, curved, fp));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    pub fn close_hole_default(&mut self, e: T::E) -> T::F
    where
        T::FP: DefaultFacePayload,
    {
        self.close_hole(e, Default::default(), false)
    }

    /// Close the face by inserting a pair of halfedges, i.e.,
    /// connecting `inside` (targeting a vertex of the to-be-inserted edge) with the
    /// next halfedge to close the face and `outside` (targeting the other vertex)
    /// with the next halfedge to complete the outside.
    /// This works even with non-manifold vertices!
    ///
    /// Returns the new face and (first) the inside edge and (second) the outside edge.
    pub fn close_face(
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
        let f = self.faces.push(HalfEdgeFace::new(inside, curved, fp));

        self.edge(inside)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));

        return (f, e1, e2);
    }

    /// Close the face by connecting vertex `from` (coming from `prev`) with vertex `to`.
    /// Inserts a pair of halfedges between these two vertices.
    /// This will only work if the insertion is unambiguous without having to look at the vertex positions, i.e., this must be a manifold vertex!
    /// If `to` has more than one ingoing edge that can reach `from`, use `close_face` instead and provide the edges.
    pub fn close_face_vertices(
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

    /// Removes the provided face.
    pub fn remove_face(&mut self, f: T::F) -> T::FP {
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
}

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Same as `close_face_vertices` but with default edge and face payloads
    pub fn close_face_vertices_default(
        &mut self,
        prev: T::V,
        from: T::V,
        to: T::V,
        curved: bool,
    ) -> (T::F, T::E, T::E) {
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

    /// Same as `close_face` but with default edge and face payloads
    pub fn close_face_default(
        &mut self,
        inside: T::E,
        outside: T::E,
        curved: bool,
    ) -> (T::F, T::E, T::E) {
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
