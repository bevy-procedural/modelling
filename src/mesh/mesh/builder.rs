use crate::mesh::{DefaultEdgePayload, EdgeCursorBasics, MeshBasics, MeshType};
use itertools::Itertools;

/// Some basic operations to build meshes.
pub trait MeshBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// add a new vertex and return it's id
    fn insert_vertex(&mut self, vp: T::VP) -> T::V;

    /// Removes the vertex `v`.
    /// Panics if the vertex doesn't exist or if it is not isolated.
    #[inline]
    fn remove_vertex(&mut self, v: T::V) {
        assert!(self.try_remove_vertex(v), "Could not remove vertex {}", v);
    }

    /// Removes the vertex `v` "recursively", i.e., also removes all edges and faces connected to it.
    /// Panics if the vertex doesn't exist.
    #[inline]
    fn remove_vertex_r(&mut self, v: T::V) {
        // PERF: avoid allocation
        let es = self.vertex(v).edges_out_ids().collect_vec();
        for e in es {
            self.remove_edge_r(e);
        }
        self.remove_vertex(v);
    }

    /// Tries to remove the vertex `v` and returns whether it was successful.
    /// Fails if the vertex is not isolated.
    fn try_remove_vertex(&mut self, v: T::V) -> bool;

    /// Inserts vertices a and b and adds an isolated edge between a and b.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from `a` to `b`. This is also the half-edge that is returned.
    #[inline]
    fn insert_isolated_edge(&mut self, a: T::VP, b: T::VP, ep: T::EP) -> T::E {
        let v = self.insert_vertex(a);
        self.insert_vertex_v(v, b, ep).unwrap().0
    }

    /// Creates a new vertex based on `vp` and connects it to vertex `v`.
    /// Returns the new edge and vertex id.
    ///
    /// Fails if the connectivity to the existing vertex is ambiguous, i.e.,
    /// there is not exactly one boundary passing through `v` or `v` is isolated.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the current vertex to the new vertex. This is also the half-edge that is returned.
    #[inline]
    #[must_use]
    fn insert_vertex_v(&mut self, v: T::V, vp: T::VP, ep: T::EP) -> Option<(T::E, T::V)> {
        let w = self.insert_vertex(vp);
        if let Some(e) = self.insert_edge_vv(v, w, ep) {
            Some((e, w))
        } else {
            self.remove_vertex(w);
            None
        }
    }

    /// Creates a new vertex based on `vp` and connects it to (the target of) edge `e`.
    /// Returns the new edge and vertex id.
    ///
    /// Shouldn't fail for half-edge meshes since the connectivity is never ambiguous.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the current vertex to the new vertex. This is also the half-edge that is returned.
    #[inline]
    #[must_use]
    fn insert_vertex_e(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> Option<(T::E, T::V)> {
        let v = self.insert_vertex(vp);
        self.insert_edge_ev(e, v, ep).map(|e| (e, v))
    }

    /// Connects the vertices `a` and `b` with an edge and returns the edge id.
    /// This will not close any face! The method will not check whether the vertices
    /// are in different connected components, so, you can generate non-manifold meshes
    /// using this method.
    ///
    /// If `a` and `b` are connected by some boundary, it will walk backwards from `b`
    /// and use the first edge coming from `a` to create the new boundary connectivity.
    ///
    /// The edge will be updated with the matching faces to continue the boundary.
    ///
    /// Fails if the connectivity is ambiguous, i.e., if `a` and `b` both have edges but
    /// are not connected by exactly one boundary of minimal length, e.g., when they
    /// are in different connected components such that chirality is ambiguous or when
    /// there is more than one boundary cycle of minimal length passing through both vertices.
    ///
    /// Notice that this boundary checks can be costly if you have large faces!
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the `a` to `b`. This is also the half-edge that is returned.
    #[must_use]
    fn insert_edge_vv(&mut self, a: T::V, b: T::V, ep: T::EP) -> Option<T::E>;

    /// Inserts an edge from the target of `input` to the origin of `output`.
    ///
    /// Connectivity is inferred from the graph. In case of half-edge meshes, the
    /// method should never fail since the connectivity is never ambiguous.
    ///
    /// The edge will be updated with the matching faces to continue the boundary.
    ///
    /// For half-edge meshes, the payload will be added to the half-edge
    /// from the `input` to `output`. This is also the half-edge that is returned.
    #[must_use]
    fn insert_edge_ee(&mut self, input: T::E, output: T::E, ep: T::EP) -> Option<T::E>;

    /// Inserts an edge from the target of `input` to the origin of `output`.
    ///
    /// It behaves similar to [MeshBuilder::insert_edge_ee] but will not run any checks to see if the insertion is valid!
    /// This allows you to run this method on invalid meshes given that the `next` of the two edges is valid.
    fn insert_edge_ee_forced(&mut self, input: T::E, output: T::E, ep: T::EP) -> T::E;

    /// Inserts an edge from the target of `e` to the vertex `v`.
    ///
    /// Connectivity at `v` is inferred from the graph. Behaves similar to [MeshBuilder::insert_edge_vv].
    #[must_use]
    fn insert_edge_ev(&mut self, e: T::E, v: T::V, ep: T::EP) -> Option<T::E>;

    /// Removes the edge `e`.
    /// Panics if the edge doesn't exist or there are adjacent faces.
    ///
    /// On half-edge meshes, this will also remove the twin edge and update the neighbors' connectivity.
    #[inline]
    fn remove_edge(&mut self, e: T::E) {
        assert!(self.try_remove_edge(e), "Could not remove edge {}", e);
    }

    /// Removes the edge `e`.
    /// Doesn't check whether the edge is isolated or not.
    /// Will behave like [MeshBuilder::try_remove_edge] otherwise.
    ///
    /// On half-edge meshes, this will also remove the twin edge and update the neighbors' connectivity.
    fn try_remove_edge_forced(&mut self, e: T::E) -> bool;

    /// Removes the edge `e` "recursively", i.e., also removes all faces connected to it.
    /// Panics if the edge doesn't exist.
    #[inline]
    fn remove_edge_r(&mut self, e: T::E) {
        // PERF: avoid allocation
        let fs = self.edge(e).face_ids().collect_vec();
        for f in fs {
            self.remove_face(f);
        }
        self.remove_edge(e);
    }

    /// Tries to remove the edge `e` and returns whether it was successful.
    /// Fails if there are adjacent faces or the edge doesn't exist.
    /// Doesn't panic. Restores the original state if the method fails.
    ///
    /// On half-edge meshes, this will also remove the twin edge and update the neighbors' connectivity.
    fn try_remove_edge(&mut self, e: T::E) -> bool;

    ////////////////////////////////////////////////////////////////////////////////////
    //************************ Face-related ******************************************//

    /// Removes the face `f`.
    /// Panics if the face doesn't exist.
    #[inline]
    fn remove_face(&mut self, f: T::F) {
        assert!(self.try_remove_face(f), "Could not remove face {}", f);
    }

    /// Tries to remove the face `f` and returns whether it was successful.
    /// Fails only if the face doesn't exist.
    fn try_remove_face(&mut self, f: T::F) -> bool;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    /// Fails if there is already a face using the boundary.
    ///
    /// The given edge will be the representative edge of the face.
    #[must_use]
    fn insert_face(&mut self, e: T::E, fp: T::FP) -> Option<T::F>;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    ///
    /// The given edge will be the representative edge of the face.
    #[must_use]
    fn insert_face_forced(&mut self, e: T::E, fp: T::FP) -> T::F;

    /// Close the given boundary by inserting an edge from `from.target` to
    /// `to.origin` and insert a face.
    ///
    /// There must be exactly one boundary path from `to` to `from` without a face.
    /// This boundary will be used to construct the face.
    /// See [MeshBuilder::insert_edge_ee] for more information.
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from.target` to `to.origin`.
    #[must_use]
    #[inline]
    fn close_face_ee(
        &mut self,
        from: T::E,
        to: T::E,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        // TODO: should we check that there is a path from `to` to `from`?
        let e = self.insert_edge_ee(from, to, ep)?;
        debug_assert_eq!(self.edge(e).target_id(), self.edge(to).origin_id());
        debug_assert_eq!(self.edge(e).origin_id(), self.edge(from).target_id());

        // `insert_face` fails if there is already a face using the boundary
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary by inserting an edge from `from.target` to
    /// `to` and insert a face.
    ///
    /// There must be exactly one boundary path from `to` to `from` without a face.
    /// This boundary will be used to construct the face.
    /// See [MeshBuilder::insert_edge_ev] for more information.
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from.target` to `to.origin`.
    #[must_use]
    #[inline]
    fn close_face_ev(
        &mut self,
        from: T::E,
        to: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        // TODO: debug_assert!(self.edge(from).same_boundary_back(to));
        let e = self.insert_edge_ev(from, to, ep)?;
        debug_assert_eq!(self.edge(e).target_id(), to);
        debug_assert_eq!(self.edge(e).origin_id(), self.edge(from).target_id());

        // `insert_face` fails if there is already a face using the boundary
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary by inserting an edge from `from` to `to` and insert a face.
    /// The face will be inserted such that the edge from `from` to `to` appears ccw in the face.
    ///
    /// The connection must be unambiguous in the same sense as required by [MeshBuilder::insert_edge_vv].
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from` to `to`.
    #[must_use]
    #[inline]
    fn close_face_vv(
        &mut self,
        from: T::V,
        to: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        let e = self.insert_edge_vv(from, to, ep)?;
        debug_assert_eq!(self.edge(e).target_id(), to);
        debug_assert_eq!(self.edge(e).origin_id(), from);

        // `insert_face` fails if there is already a face using the boundary
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary by inserting an edge from `from` to `to` and insert a face.
    /// The vertex `prev` must also lie on the face with an edge from `prev` to `from`. That way
    /// we can know which side of the edge to insert the face.
    ///
    /// There must be exactly one edge chain from `to` to `from` without a face.
    /// Otherwise, the method will return `None`.
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from` to `to`.
    #[must_use]
    #[inline]
    fn close_face_vvv(
        &mut self,
        prev: T::V,
        from: T::V,
        to: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<(T::E, T::F)> {
        self.close_face_ev(self.shared_edge_id(prev, from)?, to, ep, fp)
    }

    ////////////////////////////////////////////////////////////////////////////////////
    //************************ More complex operations *******************************//

    /// Subdivide the given edge (resp. half-edge pair) by inserting a new vertex,
    /// changing the edge's target vertex to the new vertex and connecting the new vertex
    /// to the edge's original target vertex.
    ///
    /// Returns the (half)edge starting in the new vertex.
    /// Panics if the edge doesn't exist.
    /// Sets the same faces on the inserted edge as the original edge.
    fn subdivide_edge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E;

    /// Like [MeshBuilder::subdivide_edge] but takes an iterator of vertices and edges to insert.
    ///
    /// Returns the last (half)edge inserted, i.e., the one pointing to the original target vertex.
    #[inline]
    fn subdivide_edge_iter(&mut self, e: T::E, vs: impl IntoIterator<Item = (T::EP, T::VP)>) -> T::E {
        let mut last = e;
        for (ep, vp) in vs {
            last = self.subdivide_edge(last, vp, ep);
        }
        last
    }

    /// Subdivide the face by inserting a diagonal edge from the target `v` of `from` to the origin `w` of `to`.
    /// The face containing edge `wv` will keep the old face payload, the face containing `vw` will get the new payload.
    /// Returns the edge `vw`.
    ///
    /// Panics if the face doesn't exist or not both vertices are part of the face.
    ///
    /// Doesn't care about whether the diagonal is geometrically inside the face.
    #[must_use]
    fn subdivide_face(&mut self, from: T::E, to: T::E, ep: T::EP, fp: T::FP) -> Option<T::E>;

    /// Subdivide the given face by inserting a diagonal edge from `v` to `w`.
    /// The face containing edge `wv` will keep the old face payload, the face containing `vw` will get the new payload.
    /// Returns the edge `vw`.
    ///
    /// Panics if the face doesn't exist or not both vertices are part of the face.
    ///
    /// Doesn't care about whether the diagonal is geometrically inside the face.
    #[must_use]
    fn subdivide_face_v(&mut self, f: T::F, v: T::V, w: T::V, ep: T::EP, fp: T::FP)
        -> Option<T::E>;

    /// Append a chain of edges to the vertex `v` from the finite iterator of vertices and edges.
    /// Returns the first edge inserted after `v` as well as the last vertex's id.
    /// If the iterator is empty, the method will return only the vertex `v`.
    #[must_use]
    fn append_path(
        &mut self,
        v: T::V,
        iter: impl IntoIterator<Item = (T::EP, T::VP)>,
    ) -> (Option<T::E>, T::V) {
        let mut tail = v;
        let mut first_e = None;
        for (ep, vp) in iter.into_iter() {
            let (last_e, last_v) = self.insert_vertex_v(tail, vp, ep).unwrap();
            tail = last_v;
            if first_e.is_none() {
                first_e = Some(last_e);
            }
        }
        (first_e, tail)
    }

    /// Insert a path of vertices and edges starting at `vp`.
    /// Returns the first edge  inserted after `vp` as well as the last vertex's id.
    /// If the iterator is empty, the method will return only the vertex `vp`.
    #[inline]
    fn insert_path(
        &mut self,
        vp: T::VP,
        iter: impl IntoIterator<Item = (T::EP, T::VP)>,
    ) -> (Option<T::E>, T::V) {
        let v = self.insert_vertex(vp);
        self.append_path(v, iter)
    }

    /// Same as `insert_path` but closes the path by connecting the last vertex with the first one.
    /// Also, returns the first edge (outer boundary of the loop when constructed ccw).
    /// The first edge's target is the first vertex of the loop.
    /// Panics if the iterator has a length of less than 2.
    #[inline]
    fn insert_loop(&mut self, iter: impl IntoIterator<Item = (T::EP, T::VP)>) -> T::E {
        let mut iter = iter.into_iter();
        let (ep, vp) = iter.next().unwrap();
        let (e, last_v) = self.insert_path(vp, iter);
        let first_edge = self
            .insert_edge_vv(
                last_v,
                self.edge(e.expect("Iterator too short")).origin_id(),
                ep,
            )
            .unwrap();

        debug_assert_eq!(
            self.edge(first_edge).target_id(),
            self.edge(e.unwrap()).origin_id()
        );

        first_edge
    }

    /// Same as `insert_loop` but uses the default edge payload.
    #[inline]
    fn insert_loop_default(&mut self, iter: impl IntoIterator<Item = T::VP>) -> T::E
    where
        T::EP: DefaultEdgePayload,
    {
        self.insert_loop(iter.into_iter().map(|v| (Default::default(), v)))
    }

    /// Insert a face with the given vertices.
    /// If some edges to construct this face are missing, they will be created.
    /// Uses the default edge payload.
    #[must_use]
    fn insert_face_v(&mut self, _fp: T::FP, vs: impl IntoIterator<Item = T::V>) -> Option<T::F>
    where
        T::EP: DefaultEdgePayload,
    {
        let _iter = vs.into_iter();
        // use insert_edge_ee to avoid ambiguity

        todo!()
    }
}
