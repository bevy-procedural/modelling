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
        let (e1, e2) = self.insert_edge_update(inside, ep1, outside, ep2);

        // Insert the face
        let f = self.faces.push(Face::new(inside, curved, fp));

        // TODO: The clone is weird
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

    /// This will walk along the given boundary and add a "hem" made from triangles.
    /// The payloads are given using the iterator.
    ///
    /// `start` must be an edge on the boundary pointing to the first vertex to be connected with the hem.
    ///
    /// Returns the edge pointing from the first inserted vertex to the target of `start`.
    /// If the iterator is empty, return `start` instead.
    ///
    /// If `shift` is true, the first inserted triangle will be with the tip pointing to the target of `start`.
    /// Otherwise, the first triangle will include the edge `start`.
    /// This doesn't affect the number of triangles but shifts the hem by one.
    pub fn triangle_hem(
        &mut self,
        start: T::E,
        shift: bool,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        // TODO: a more efficient implementation could bulk-insert everything at once
        // TODO: assertions

        let mut input = start;
        let mut first = true;
        let mut iter = vp.into_iter();
        let mut pos = iter.next();
        let mut ret = start;

        if shift && pos.is_some() {
            let output = self.edge(input).next_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let output = self.edge(input).next_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());

            let ip = self.edge(input);
            input = ip.prev_id();
            let ip_next = ip.next_id();

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_default(output, ip_next, false);
            } else {
                ret = self.edge(output).prev_id();
            }

            pos = iter.next();
            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_default(ip_next, input, false);
            }

            first = false;
        }

        ret
    }

    /// Like `triangle_hem` but for quad faces.
    pub fn quad_hem(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions

        let mut iter = vp.into_iter();

        let mut input = start;

        if let Some(vp) = iter.next() {
            self.add_vertex_via_edge_default(input, self.edge(start).next_id(), vp);
        }

        let mut ret = start;
        for vp in iter {
            input = self.edge(input).prev_id();
            let output = self.edge(input).next(self);
            self.add_vertex_via_edge_default(input, output.id(), vp);
            self.close_face_default(output.next_id(), self.edge(input).next_id(), false);
            if ret == start {
                ret = self.edge(input).next(self).next_id();
            }
        }

        ret
    }

    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `center` with the given vp and fill the hole along the boundary with triangles connected to the center vertex.
    /// Returns the vertex.
    pub fn fill_hole_with_vertex(&mut self, start: T::E, center: T::VP) -> T::V {
        let e0 = self.edge(start);
        let origin = e0.origin_id();
        let mut input = self.edge(start).prev_id();
        let (v, _, _) = self.add_vertex_via_edge_default(input, start, center);
        loop {
            let e = self.edge(input);
            if e.origin_id() == origin {
                break;
            }
            input = e.prev_id();
            self.close_face_default(self.edge(input).next(&self).next_id(), input, false);
        }
        self.close_hole(input, Default::default(), false);

        v
    }
}
