use crate::{
    math::IndexType,
    mesh::{DefaultEdgePayload, DefaultFacePayload, HalfEdge, MeshBuilder, MeshTypeHalfEdge},
};

// TODO: Adjust this to not be halfedge-specific

/// A trait for lofting a mesh.
pub trait MeshLoft<T: MeshTypeHalfEdge<Mesh = Self>>
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
    fn loft_tri_back(
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
            self.insert_vertex_e(input, pos.unwrap(), Default::default());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let output = self.edge(input).next_id();
            self.insert_vertex_e(input, pos.unwrap(), Default::default());

            let input_next = self.edge(input).next_id();
            input = self.edge(input).prev_id();

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_ee_legacy(
                    output,
                    input_next,
                    Default::default(),
                    Default::default(),
                );
            } else {
                ret = self.edge(output).prev_id();
            }

            pos = iter.next();
            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_ee_legacy(
                    input_next,
                    input,
                    Default::default(),
                    Default::default(),
                );
            }

            first = false;
        }

        ret
    }

    /// Like `loft_tri_back` but closes the "hem" with a face.
    fn loft_tri_back_closed(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let e = self.loft_tri_back(start, false, vp);
        let outside = self.edge(e).prev_id();
        self.close_face_ee_legacy(
            self.edge(e).next(self).next_id(),
            e,
            Default::default(),
            Default::default(),
        );
        self.close_face_ee_legacy(
            self.edge(e).next_id(),
            outside,
            Default::default(),
            Default::default(),
        );
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
    fn loft_tri(&mut self, start: T::E, shift: bool, vp: impl IntoIterator<Item = T::VP>) -> T::E {
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
            self.insert_vertex_e(input, pos.unwrap(), Default::default());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let input = self.edge(output).prev_id();
            self.insert_vertex_e(input, pos.unwrap(), Default::default());

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_ee_legacy(
                    self.edge(input).next_id(),
                    self.edge(input).prev_id(),
                    Default::default(),
                    Default::default(),
                );
            } else {
                ret = self.edge(output).prev_id();
            }

            let new_output = self.edge(output).next_id();

            // only continue if there are more vertices
            pos = iter.next();

            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_ee_legacy(
                    output,
                    self.edge(output).prev(self).prev_id(),
                    Default::default(),
                    Default::default(),
                );
            }

            // advance output to the next edge on the boundary
            output = new_output;

            first = false;
        }

        ret
    }

    /// Like `loft_tri` but closes the "hem" with a face.
    /// Returns the edge pointing from the first inserted vertex to the second inserted vertex.
    fn loft_tri_closed(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let e = self.loft_tri(start, false, vp);
        let inside = self.edge(e).twin(self).prev_id();
        let outside = self.edge(inside).prev(self).prev_id();
        self.close_face_ee_legacy(inside, outside, Default::default(), Default::default());
        self.close_face_ee_legacy(
            self.edge(e).twin_id(),
            outside,
            Default::default(),
            Default::default(),
        );
        self.edge(outside).next(self).next_id()
    }

    /// Like `loft_polygon`, but walks the boundary backwards.
    fn loft_polygon_back(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,

        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<T::E> {
        assert!(n >= 2);
        assert!(m >= 2);
        // TODO: implement the special cases of n=1 (insert one vertex only and generate tip) and m=1 (use only one vertex of the boundary and create a fan around it), and even n=0 (without iterator; bridge every second edge with a face) and m=0 (without start; generate a line strip)
        // TODO: replace all loft methods with this one. Quad is just n=2, m=2, triangle is two lofts: n=2, m=1 and n=0, m=2

        // TODO: Make this optional?
        let autoclose = true;

        // PERF: Instead of insert_face, we could directly insert the face indices when creating the edges

        // insert the outer boundary
        let mut iter = vp.into_iter();
        let mut inner = self.edge(start).prev_id();
        let mut last = false;
        let mut last_inner = start;
        let current_inner = inner;
        let mut outer = IndexType::max();
        let mut res = None;

        loop {
            // Skip the center edges
            for _ in 1..m {
                if inner == last_inner {
                    // We reached the start again - so we are done!
                    // TODO: test this!
                    return res;
                }
                inner = self.edge(inner).prev_id();
            }

            // insert first diagonal towards bow in the first iteration
            if outer == IndexType::max() {
                let (e, _) =
                    self.insert_vertex_e(current_inner, iter.next()?, Default::default())?;
                last_inner = self.edge(e).twin_id();
                outer = e;
            }

            // Insert next bow
            for i in 1..n {
                let Some(vp) = iter.next() else {
                    if i == 1 {
                        // We are done - the iterator ended just after the last bow
                        return res;
                    }
                    // We are done - the iterator ended in the middle of the bow. Close it!
                    last = true;
                    break;
                };
                let (e, _) = self.insert_vertex_e(outer, vp, Default::default())?;
                outer = e;

                if res == None {
                    res = Some(e);
                }
            }

            if autoclose && last && inner == last_inner {
                // automatically close the shape
                inner = self.edge(inner).prev_id();
            }

            // Insert the diagonal between inner and outer and create a face
            outer = self.insert_edge_ee(inner, self.edge(outer).next_id(), Default::default())?;
            self.insert_face(self.edge(outer).twin_id(), Default::default())?;

            // TODO: Why is this not the same as above?
            /*let (e, f) = self.close_face_ee(
                inner,
                self.edge(outer).next_id(),
                Default::default(),
                Default::default(),
            )?;
            outer = self.edge(e).twin_id();*/

            if last {
                return res;
            }
        }

        /*
        assert!(n >= 2);
        assert!(m >= 2);
        // TODO: implement the special cases of n=1 (insert one vertex only and generate tip) and m=1 (use only one vertex of the boundary and create a fan around it), and even n=0 (without iterator; bridge every second edge with a face) and m=0 (without start; generate a line strip)
        // TODO: replace all loft methods with this one. Quad is just n=2, m=2, triangle is two lofts: n=2, m=1 and n=0, m=2

        let mut iter = vp.into_iter();
        let mut input = start;
        let start_vertex = self.edge(start).target_id(self);
        if let Some(vp) = iter.next() {
            self.insert_vertex_e(input, vp, Default::default());
        }

        let mut ret = start;
        loop {
            input = self.edge(input).prev_id();

            let mut inside = self.edge(input).next(self).next_id();
            for _ in 2..n {
                let Some(vp) = iter.next() else {
                    return ret;
                };
                let (e1, _) = self
                    .insert_vertex_e(inside, vp, Default::default())
                    .unwrap(); // TODO: error handling
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
                    self.close_face_ee_legacy(
                        inside,
                        self.edge(input).prev_id(),
                        Default::default(),
                        Default::default(),
                    );
                }
                return ret;
            };

            self.insert_vertex_e(input, vp, Default::default());
            self.close_face_ee_legacy(
                inside,
                self.edge(input).next_id(),
                Default::default(),
                Default::default(),
            );

            // when n==2, we cannot set the `ret` until now
            if ret == start {
                ret = self.edge(inside).next(self).twin_id();
            }
        }*/
    }

    /// Walks counter-clockwise along the given boundary and adds a "hem" made from polygon faces.
    /// Each face consists of `n` vertices from the iterator
    /// and `m` vertices from the boundary of the existing mesh.
    /// Hence, it will create polygon faces with `n+m` vertices each.
    ///
    /// If the iterator is long enough to go once around the boundary,
    /// the "hem" will be automatically closed.
    ///
    /// Returns the edge pointing from the second inserted vertex to the first inserted vertex.
    /// Will return `None` if the boundary was weird or the iterator was empty. Will not complain
    /// about an iterator of length 1 or an iterator that is too short or too long. If the iterator
    /// is too long, all additional vertices will be ignored. If it is too short and not divisible by `n`,
    /// the last face will be smaller.
    /// 
    /// If the boundary is too short to fit the next `n` new vertices, none of them will be inserted.
    /// e.g., if you have a triangle and insert with `m=2`, then at most one face will be created since
    /// two won't fit. 
    ///
    /// Some examples (see `--example loft`):
    /// - To create a quad loft, use `loft_polygon(start, 2, 2, vp)`.
    /// - Pentagons with the tip pointing to the boundary can be created with `loft_polygon(start, 3, 2, vp)`
    /// - while pentagons with the tip pointing away from the boundary can be created with `loft_polygon(start, 2, 3, vp)`.
    /// - n=1: insert one vertex only and generate tip
    /// - m=1: use only one vertex of the boundary and create a fan around it.
    /// - n=0: without iterator; bridge every second edge with a face
    /// - m=0: without start; generate a line strip
    /// - a really long iterator that spirals around the original mesh in multiple layers
    /// - Iterator of length 1: will insert one vertex and make a face with the `m` next vertices of the boundary
    #[must_use]
    fn loft_polygon(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<T::E>
    where
        T::Mesh: MeshBuilder<T>,
    {
        assert!(n >= 2);
        assert!(m >= 2);

        return None;

        /*assert!(n >= 2);
        assert!(m >= 2);
        // TODO: implement the special cases of n=1 (insert one vertex only and generate tip) and m=1 (use only one vertex of the boundary and create a fan around it), and even n=0 (without iterator; bridge every second edge with a face) and m=0 (without start; generate a line strip)
        // TODO: replace all loft methods with this one. Quad is just n=2, m=2, triangle is two lofts: n=2, m=1 and n=0, m=2

        let mut iter = vp.into_iter();
        let mut input = start;
        let start_vertex = self.edge(start).origin_id();
        let Some(vp) = iter.next() else {
            return None;
        };
        let (first_edge, first_vertex) = self.insert_vertex_e(self.edge(start).prev_id(), vp, Default::default())?;
        println!("first_edge: {} {}", first_edge, first_vertex);

        let mut ret = start;
        loop {
            // Move `input` forward along the boundary
            input = self.edge(input).next_id();

            // TODO: only provisory impl!
            assert!(n == 2);

            // Initialize `inside` edge
            let mut inside = self.edge(input).prev(self).prev_id();
            for _ in 2..n {
                let Some(vp) = iter.next() else {
                    return Some(ret);
                };
                // Insert vertex between `inside`'s previous edge and `inside`
                let (e1, _) =
                    self.insert_vertex_e(self.edge(inside).prev_id(), vp, Default::default())?;
                inside = e1;

                // Set `ret` to the edge pointing to the first generated vertex
                if ret == start {
                    ret = self.edge(e1).twin_id();
                }
            }

            // TODO: only provisory impl!
            assert!(m == 2);

            // Move `input` forward along the boundary for `m - 2` steps
            for _ in 2..m {
                input = self.edge(input).next_id();
            }

            let Some(vp) = iter.next() else {
                if start_vertex == self.edge(input).origin_id() {
                    // Reached the start again - close the last face
                    self.close_face_ee_legacy(
                        input,
                        self.edge(inside).prev_id(),
                        Default::default(),
                        Default::default(),
                    )?;
                }
                return Some(ret);
            };
            // Insert a new vertex between the previous edge of `input` and `input`
            self.insert_vertex_e(self.edge(input).prev_id(), vp, Default::default())?;

            // Close the face between `inside` and the new vertex
            self.close_face_ee_legacy(
                self.edge(input).prev(self).twin_id(),
                self.edge(inside).prev_id(),
                Default::default(),
                Default::default(),
            )?;

            // When `n == 2`, we cannot set `ret` until now
            if ret == start {
                ret = self.edge(inside).prev(self).twin_id();
            }
        }*/
    }
}

// TODO: tests!
