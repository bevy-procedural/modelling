use crate::{
    representation::{DefaultEdgePayload, DefaultFacePayload, Face, IndexType, Mesh, MeshType},
    util::iter::contains_exactly_one,
};

impl<T: MeshType> Mesh<T> {
    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    pub fn close_hole(&mut self, e: T::E, fp: T::FP, curved: bool) -> T::F {
        let f = self.faces.push(Face::new(e, curved, fp));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }

    /// Close the face by inserting a pair of halfedges, i.e.,
    /// connecting `inside` (targeting a vertex of the to-be-inserted edge) with the
    /// next halfedge to close the face and `outside` (targeting the other vertex)
    /// with the next halfedge to complete the outside.
    /// This works even with non-manifold vertices!
    pub fn close_face(
        &mut self,
        inside: T::E,
        ep1: T::EP,
        outside: T::E,
        ep2: T::EP,
        fp: T::FP,
        curved: bool,
    ) -> T::F {
        let e_inside = self.edge(inside);
        let e_outside = self.edge(outside);
        let v = e_inside.target(self).id();
        let w = e_outside.target(self).id();

        debug_assert!(e_inside.same_face_back(self, w));
        debug_assert!(e_outside.same_face_back(self, v));

        let other_inside = e_outside.next(self);
        let other_outside = e_inside.next(self);

        let (e1, e2) = self.insert_edge(
            (other_inside.id(), inside, v, IndexType::max(), ep1),
            (other_outside.id(), outside, w, IndexType::max(), ep2),
        );

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        // Insert the face
        let f = self.faces.push(Face::new(inside, curved, fp));

        // TODO: The clone is weird
        self.edge(inside)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));

        return f;
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
    ) -> T::F {
        let inside = self.shared_edge(prev, from).unwrap().id();

        // TODO: is it enough to assert this vertex is manifold? Also, add code to check for manifold vertices!
        debug_assert!(
            contains_exactly_one(self.vertex(to).edges_in(self), |e| {
                e.is_boundary_self() && e.same_face(self, self.edge(inside).origin_id())
            }),
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
    pub fn remove_face(&mut self, f: T::F) {
        let face = self.face(f);

        let edge_ids: Vec<_> = face.edges(self).map(|e| e.id()).collect();
        for e in edge_ids {
            self.edge_mut(e).delete_face();
        }
        self.faces.delete_internal(f);
    }
}

impl<T: MeshType> Mesh<T>
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
    ) -> T::F {
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
    pub fn close_face_default(&mut self, inside: T::E, outside: T::E, curved: bool) -> T::F {
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
