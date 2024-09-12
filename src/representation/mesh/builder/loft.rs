use crate::representation::{DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
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
    /// This doesn't affect the number of triangles but shifts the "hem" by one.
    pub fn loft_tri(
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

    /// Like `loft_tri` but closes the "hem" with a face.
    pub fn loft_tri_closed(&mut self, _start: T::E, _vp: impl IntoIterator<Item = T::VP>) -> T::E {
        todo!("loft_tri_closed")
    }

    /// Like `hem_tri` but for quad faces.
    pub fn loft_quads(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        // TODO: Can be written faster without the "smart" edge functions but by bulk inserting into the mesh

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

    /// Like `hem_quads` but closes the "hem" with a face.
    pub fn loft_quads_closed(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let e = self.loft_quads(start, vp);
        self.close_face_default(self.edge(e).next(self).next(self).next_id(), e, false);
        e
    }

    /// Like `loft_quad`, but each face consists of `n` vertices from the iterator
    /// and `m` vertices from the boundary of the existing mesh.
    /// Hence, it will create polygon faces with `n+m+2` vertices each.
    /// If the iterator is exactly the right length to go once around the mesh, the "hem" will be closed.
    pub fn loft_polygon(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        assert!(n >= 2);
        assert!(m >= 2);
        // TODO: implement the special cases of n=1 (insert one vertex only and generate tip) and m=1 (use only one vertex of the boundary and create a fan around it), and even n=0 (without iterator; bridge every second edge with a face) and m=0 (without start; generate a line strip)
        // TODO: replace all loft methods with this one. Quad is just n=2, m=2, triangle is two lofts: n=2, m=1 and n=0, m=2

        let mut iter = vp.into_iter();
        let mut input = start;
        let start_vertex = self.edge(start).target_id(self);
        if let Some(vp) = iter.next() {
            self.add_vertex_via_edge_default(input, self.edge(start).next_id(), vp);
        }

        let mut ret = start;
        loop {
            input = self.edge(input).prev_id();

            let mut inside = self.edge(input).next(self).next_id();
            for _ in 2..n {
                let Some(vp) = iter.next() else {
                    return ret;
                };
                let (_, e1, _) =
                    self.add_vertex_via_edge_default(inside, self.edge(inside).next_id(), vp);
                inside = e1;

                // the edge pointing to the first generated vertex
                if ret == start {
                    ret = self.edge(e1).twin_id();
                }
            }

            for _ in 2..m {
                input = self.edge(input).prev_id();
            }

            let Some(vp) = iter.next() else {
                if start_vertex == self.edge(input).target_id(self) {
                    // reached the start again - close the last vertex!
                    self.close_face_default(inside, self.edge(input).prev_id(), false);
                }
                return ret;
            };

            let output = self.edge(input).next_id();
            self.add_vertex_via_edge_default(input, output, vp);
            self.close_face_default(inside, self.edge(input).next_id(), false);

            // when n==2, we cannot set the `ret` until now
            if ret == start {
                ret = self.edge(inside).next(self).twin_id();
            }
        }
    }
}
