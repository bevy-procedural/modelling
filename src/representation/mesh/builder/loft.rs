use crate::representation::{DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// This will walk clockwise (backwards) along the given boundary and add a "hem" made from triangles.
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
    pub fn loft_tri_back(
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

            let input_next = self.edge(input).next_id();
            input = self.edge(input).prev_id();

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_default(output, input_next, false);
            } else {
                ret = self.edge(output).prev_id();
            }

            pos = iter.next();
            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_default(input_next, input, false);
            }

            first = false;
        }

        ret
    }

    /// Like `loft_tri_back` but closes the "hem" with a face.
    pub fn loft_tri_back_closed(
        &mut self,
        start: T::E,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        let e = self.loft_tri_back(start, false, vp);
        let outside = self.edge(e).prev_id();
        self.close_face_default(self.edge(e).next(self).next_id(), e, false);
        self.close_face_default(self.edge(e).next_id(), outside, false);
        e
    }

    /// This will walk counter-clockwise along the given boundary and add a "hem" made from triangles.
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

        // output will walk forward around the boundary
        let mut output = start;

        let mut first = true;
        let mut iter = vp.into_iter();
        let mut pos = iter.next();
        let mut ret = start;

        if shift && pos.is_some() {
            let input = self.edge(output).prev_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let input = self.edge(output).prev_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_default(
                    self.edge(input).next_id(),
                    self.edge(input).prev_id(),
                    false,
                );
            } else {
                ret = self.edge(output).prev_id();
            }

            let new_output = self.edge(output).next_id();

            // only continue if there are more vertices
            pos = iter.next();

            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_default(output, self.edge(output).prev(self).prev_id(), false);
            }

            // advance output to the next edge on the boundary
            output = new_output;

            first = false;
        }

        ret
    }

    /// Like `loft_tri` but closes the "hem" with a face.
    /// Returns the edge pointing from the first inserted vertex to the second inserted vertex.
    pub fn loft_tri_closed(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let e = self.loft_tri(start, false, vp);
        let inside = self.edge(e).twin(self).prev_id();
        let outside = self.edge(inside).prev(self).prev_id();
        self.close_face_default(inside, outside, false);
        self.close_face_default(self.edge(e).twin_id(), outside, false);
        self.edge(outside).next(self).next_id()
    }

    /// Walks along the boundary given by `start` and adds a "hem" made from polygon faces.
    /// Each face consists of `n` vertices from the iterator
    /// and `m` vertices from the boundary of the existing mesh.
    /// Hence, it will create polygon faces with `n+m+2` vertices each.
    ///
    /// If the iterator is exactly the right length to go once around the mesh, the "hem" will be closed.
    ///
    /// Returns the edge pointing from the second inserted vertex to the first inserted vertex.
    ///
    /// For example, to create a quad loft, use `loft_polygon(start, 2, 2, vp)`.
    /// Pentagons with the tip pointing to the boundary can be created with `loft_polygon(start, 3, 2, vp)`
    /// while pentagons with the tip pointing away from the boundary can be created with `loft_polygon(start, 2, 3, vp)`.
    pub fn loft_polygon_back(
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

    // TODO: forward polygon loft!
}


// TODO: tests!