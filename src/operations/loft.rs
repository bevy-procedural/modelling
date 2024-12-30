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

    /// Walks along the given boundary and "crochet" a "hem" made from polygon faces.
    /// Each face consists of `n` vertices from the iterator
    /// and `m` vertices from the boundary of the existing mesh.
    /// Hence, it will create polygon faces with `n+m` vertices each.
    ///
    /// If the iterator is long enough to go once around the boundary,
    /// the "hem" will be automatically closed if `autoclose` is true.
    ///
    /// If the boundary is too short to fit the next `n` new vertices, none of them will be inserted.
    /// e.g., if you have a triangle and insert with `m=2`, then at most one face will be created since
    /// two won't fit.
    ///
    /// It returns the first and last edge on the new boundary, e.g.,
    /// - if `n>=2`: the edge between the first inserted vertex and the second inserted
    ///   vertex and the edge between the last to the second to last. The direction will
    ///   be chosen such that it is the edge that is not part of the inserted face. If the
    ///   hem is autoclosed, the last edge will be the edge from the last inserted vertex
    ///   to the first instead, so the edges will actually be neighbors.
    /// - if `n=0`: the edge between the first boundary vertex to the `n`th if `m=0`.
    /// If there is no new boundary edge, it will return `start` instead.
    /// The function will return `None` if the boundary had unclear connectivity
    /// (which shouldn't happen on half-edge meshes).
    ///
    /// The operation is guaranteed to not insert edges that are not incident to any face.
    ///
    /// Parameters:
    /// - `start`: the edge on the boundary pointing from the first vertex used in the hem to the second.
    ///    Iff `m=0`, the edge will be ignored and can be safely set to `IndexType::max()`.
    /// - `n`: the number of vertices in the iterator that will be inserted in each face.
    /// - `m`: the number of vertices from the boundary that will be used in each face.
    ///   Notice that `n+m` must be at least 3.
    /// - `backwards`: if true, the boundary is walked backwards instead of forwards.
    ///    The `target` of `start` will be used instead of the `origin` to build the first face.
    /// - `autoclose`: if true and the iterator is long enough, the hem will be automatically closed.
    /// - `open`: if true, the hem will be connected back to the inner boundary in every iteration.
    ///   Hence, the newly created boundary will be longer by `2` edges per created face.
    ///   `open` cannot be `true` if `autoclose` is `true`.
    /// - `vp`: the iterator of vertices to be inserted in each face.
    ///   - If the iterator is too long to fit everything along the boundary, all additional vertices will be ignored.
    ///     Hence, it is always safe to provide an infinite iterator.
    ///   - If `n=0`, the iterator is ignored.
    ///   - If the iterator is too short and doesn't end with a completed face, the last face will have less vertices.
    ///   - If the iterator is too short and there is exactly one vertex left and `open` is `true`, there would be a single edge not connected to a face,
    ///     which is considered invalid. Hence, in that specific situation the last vertex will be ignored!
    ///   - If the iterator is empty but `n>0`, the function will return `start` instead.
    ///   - If the iterator has only length `1` and `m <= 11`, the function will return `start` instead.
    ///
    ///
    /// Some examples to illustrate special cases (also see `--example loft`):
    /// - `n=0, m>=3`: append faces by using `m` vertices from the boundary and ignoring the iterator.
    ///    The value of `open` doesn't matter in this case.
    /// - `n=1, m=1, open`: Create a star graph (not a star polygon).
    ///    i.e., the first vertex (depending on `backwards` the `origin` or `target` of `start`)
    ///    is connected to all vertices of the iterators without inserting any faces.
    ///    This parameter combination is currently not supported since it doesn't insert any faces.
    /// - `n=1, m=2, open`: append triangles by using two vertices from the boundary and one from the iterator each,
    ///    e.g., if you start with a regular polygon, you'll get a star where the tips are from the iterator.
    /// - `n=1, m=2, !open`: insert only one vertex from the iterator and create a windmill around it,
    ///    e.g., if you start with a regular polygon, the result will be a pyramid.
    /// - `n>=2, m=1, !open`: Use only one vertex from the boundary and create a fan around it.
    ///    When `backwards` is true, this will be the target of `start`, otherwise the origin.
    ///   `autoclose` doesn't have any effect here, since it could only come into effect if the mesh was a single isolated vertex.
    /// - `n>=2, m=1, !open`: Like above, but the triangles are disconnected.
    /// - `n=2, m=2, open`: append disconnected quads to the boundary
    /// - `n=2, m=2, !open`: append a quad loft to the boundary
    /// - `n=2, m=3, !open`: create a hem of pentagons. The "tip" of each pentagon points inwards.
    /// - `n>=3, m=0, open`: insert disconnected polygons with `n` vertices each. Ignores `start`.
    /// - `n>=3, m=0, !open`: Generate a polygon strip. Ignores `start`.
    ///    Each polygon will have `n` vertices from the iterator and will be
    ///    connected with it's first vertex to the previous' polygons last vertex.
    ///    When `autoclose` is true, the last polygon will be connected to the first.
    /// - `n=3, m=2, !open`: create a hem of pentagons. The "tip" of each pentagon points outwards.
    #[must_use]
    #[inline(always)]
    fn crochet(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        backwards: bool,
        autoclose: bool,
        open: bool,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<T::E> {
        assert!(n + m >= 3, "n+m must be at least 3");
        assert!(
            !autoclose || !open,
            "autoclose and open cannot be true at the same time"
        );

        // TODO
        assert!(n >= 2);
        // TODO
        assert!(m >= 2);
        // TODO
        assert!(backwards);
        // TODO
        assert!(!open);

        // TODO: implement the special cases of n=1 (insert one vertex only and generate tip) and m=1 (use only one vertex of the boundary and create a fan around it), and even n=0 (without iterator; bridge every second edge with a face) and m=0 (without start; generate a line strip)
        // TODO: replace all loft methods with this one. Quad is just n=2, m=2, triangle is two lofts: n=2, m=1 and n=0, m=2

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
                    if i == 1 && (n >= 3 || !autoclose) {
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
    }

    /// see `crochet(start, n, m, true, true, false, vp)`
    fn loft_polygon_back(
        &mut self,
        start: T::E,
        n: usize,
        m: usize,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> Option<T::E> {
        self.crochet(start, n, m, true, true, false, vp)

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

    /// see `crochet(start, n, m, true, false, false, vp)`
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
        self.crochet(start, n, m, true, false, false, vp)
    }
}

// TODO: tests!
