use crate::mesh::{cursor::*, DefaultEdgePayload, MeshBasics, MeshType};
use itertools::Itertools;

/// Some basic operations to build meshes.
pub trait MeshBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// add a new vertex and return it's id
    fn insert_vertex_id(&mut self, vp: T::VP) -> T::V;

    /// add a new vertex and return a valid vertex cursor
    fn insert_vertex<'a>(&'a mut self, vp: T::VP) -> ValidVertexCursorMut<'a, T>
    where
        T: 'a,
    {
        let v = self.insert_vertex_id(vp);
        self.vertex_mut(v).unwrap()
    }

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
        let v = self.insert_vertex_id(a);
        self.insert_vertex_v(v, b, ep).unwrap().0
    }

    /// Creates a new vertex based on `vp` and connects it to vertex `v`.
    /// Returns the new edge and vertex id.
    ///
    /// Fails if the connectivity to the existing vertex is ambiguous, i.e.,
    /// there is not exactly one chain passing through `v` or `v` is isolated.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the current vertex to the new vertex. This is also the half-edge that is returned.
    #[inline]
    #[must_use]
    fn insert_vertex_v(&mut self, v: T::V, vp: T::VP, ep: T::EP) -> Option<(T::E, T::V)> {
        let w = self.insert_vertex_id(vp);
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
        let v = self.insert_vertex_id(vp);
        self.insert_edge_ev(e, v, ep).map(|e| (e, v))
    }

    // TODO: Check whether the use of boundary vs chain is correct here.

    /// Connects the vertices `a` and `b` with an edge and returns the edge id.
    /// This will not close any face! The method will not check whether the vertices
    /// are in different connected components, so, you can generate non-manifold meshes
    /// using this method.
    ///
    /// If `a` and `b` are connected by some boundary chain, it will walk backwards from `b`
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
        assert!(self.try_remove_face(f).is_some(), "Could not remove face {}", f);
    }

    /// Tries to remove the face `f`.
    /// Returns the face payload if the face was removed or `None` if the face doesn't exist.
    #[must_use]
    fn try_remove_face(&mut self, f: T::F) -> Option<T::FP>;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    /// Fails if there is already a face using the chain.
    ///
    /// The given edge will be the representative edge of the face.
    #[must_use]
    fn insert_face(&mut self, e: T::E, fp: T::FP) -> Option<T::F>;

    /// Close the chain with a single face. Doesn't create new edges or vertices.
    ///
    /// The given edge will be the representative edge of the face.
    fn insert_face_forced(&mut self, e: T::E, fp: T::FP) -> T::F;

    /// Close the given boundary chain by inserting an edge from `from.target` to
    /// `to.origin` and insert a face.
    ///
    /// There must be exactly one boundary path from `to` to `from` without a face.
    /// This chain will be used to construct the face.
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
        debug_assert_eq!(
            self.edge(e).unwrap().target_id(),
            self.edge(to).unwrap().origin_id()
        );
        debug_assert_eq!(
            self.edge(e).unwrap().origin_id(),
            self.edge(from).unwrap().target_id()
        );

        // `insert_face` fails if there is already a face using the chain
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary chain by inserting an edge from `from.target` to
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
        // TODO: debug_assert!(self.edge(from).same_chain_back(to));
        let e = self.insert_edge_ev(from, to, ep)?;
        debug_assert_eq!(self.edge(e).unwrap().target_id(), to);
        debug_assert_eq!(
            self.edge(e).unwrap().origin_id(),
            self.edge(from).unwrap().target_id()
        );

        // `insert_face` fails if there is already a face using the chain
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary chain by inserting an edge from `from` to `to` and insert a face.
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
        debug_assert_eq!(self.edge(e).unwrap().target_id(), to);
        debug_assert_eq!(self.edge(e).unwrap().origin_id(), from);

        // `insert_face` fails if there is already a face using the chain
        if let Some(f) = self.insert_face(e, fp) {
            Some((e, f))
        } else {
            assert!(self.try_remove_edge_forced(e));
            None
        }
    }

    /// Close the given boundary chain by inserting an edge from `from` to `to` and insert a face.
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

    /// Split the given edge (resp. half-edge pair) by inserting a new vertex,
    /// changing the edge's target vertex to the new vertex and connecting the new vertex
    /// to the edge's original target vertex.
    ///
    /// Returns the (half)edge starting in the new vertex.
    /// Panics if the edge doesn't exist.
    /// Sets the same faces on the inserted edge as the original edge.
    ///
    /// Keep in mind that edges could be curved and the curvature might not make sense after the split.
    fn split_edge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E;

    /*
    TODO:
    /// Join two faces that are separated by the given edge.
    /// Returns `None` if the edge doesn't exist or doesn't have exactly two faces.
    fn join_faces(&mut self, e: T::E) -> Option<T::F>;
    */

    /// Deletes the next edge and connects the given edge to the target of the next edge.
    /// Fails if the edge doesn't exist or the edge's target vertex doesn't have degree 2.
    /// Applying this to a pair of parallel edges or a dead-end with a self-loop will also delete the enclosed degenerate face.
    #[must_use]
    fn collapse_edge(&mut self, e: T::E) -> Option<T::E>;

    /// Like [MeshBuilder::split_edge] but takes an iterator of vertices and edges to insert.
    ///
    /// Returns the last (half)edge inserted, i.e., the one pointing to the original target vertex.
    ///
    /// Keep in mind that edges could be curved and the curvature might not make sense after the split.
    #[inline]
    fn split_edge_iter(&mut self, e: T::E, vs: impl IntoIterator<Item = (T::EP, T::VP)>) -> T::E {
        let mut last = e;
        for (ep, vp) in vs {
            last = self.split_edge(last, vp, ep);
        }
        last
    }

    /// Split the face by inserting a diagonal edge from the target `v` of `from` to the origin `w` of `to`.
    /// The face containing edge `wv` will keep the old face payload, the face containing `vw` will get the new payload.
    /// Returns the edge `vw`.
    ///
    /// Panics if the edges are not part of the same face or one of the created faces has less than 2 vertices.
    /// Won't complain about degenerate faces with 2 vertices and two parallel pairs of half-edges.
    ///
    /// Doesn't care about whether the diagonal is geometrically inside the face.
    ///
    /// Keep in mind that for concave faces not all diagonals are valid.
    /// Also keep in mind that the face could have islands.
    /// Splitting between the outer edge chain and an island is currently not supported.
    #[must_use]
    fn split_face(&mut self, from: T::E, to: T::E, ep: T::EP, fp: T::FP) -> Option<T::E>;

    /// Split the given face by inserting a diagonal edge from `v` to `w`.
    /// The face containing edge `wv` will keep the old face payload, the face containing `vw` will get the new payload.
    /// Returns the edge `vw`.
    ///
    /// Panics if the face doesn't exist or not both vertices are part of the face or one of the created faces has less than 2 vertices.
    /// Won't complain about degenerate faces with 2 vertices and two parallel pairs of half-edges.
    ///
    /// Doesn't care about whether the diagonal is geometrically inside the face.
    ///
    /// Keep in mind that for concave faces not all diagonals are valid.
    /// Also keep in mind that the face could have islands.
    /// Splitting between the outer edge chain and an island is currently not supported.
    #[must_use]
    fn split_face_v(&mut self, f: T::F, v: T::V, w: T::V, ep: T::EP, fp: T::FP) -> Option<T::E>;

    /// Append a chain of edges to the vertex `v` from the finite iterator of vertices and edges.
    ///
    /// Returns the first edge inserted after `v` as well as the last inserted edge.
    ///
    /// If the iterator is empty or the path cannot be appended (e.g., due to connectivity ambiguities), the method will `None`.
    #[inline]
    #[must_use]
    fn append_path(
        &mut self,
        v: T::V,
        iter: impl IntoIterator<Item = (T::EP, T::VP)>,
    ) -> Option<(T::E, T::E)> {
        let mut iter = iter.into_iter();
        let Some((ep, vp)) = iter.next() else {
            return None;
        };
        let (first_e, _first_v) = self.insert_vertex_v(v, vp, ep)?;
        let mut last_e = self.edge_mut(first_e);
        for (ep, vp) in iter.into_iter() {
            last_e = last_e.insert_vertex(vp, ep);
        }
        let Some(valid) = last_e.load() else {
            return None;
        };
        Some((first_e, valid.id()))
    }

    /// Insert a path of vertices and edges starting at `vp`.
    /// Returns the first edge inserted after `vp` as well as the last inserted edge.
    /// Panics if the iterator is empty.
    #[inline]
    fn insert_path(
        &mut self,
        vp: T::VP,
        iter: impl IntoIterator<Item = (T::EP, T::VP)>,
    ) -> (T::E, T::E) {
        let v = self.insert_vertex_id(vp);
        self.append_path(v, iter).unwrap()
    }

    /// Same as [MeshBuilder::insert_path] but closes the path by connecting the last vertex with the first one.
    ///
    /// Returns the first edge (outer boundary chain of the loop when constructed ccw).
    ///
    /// The first edge's target is the first vertex of the loop.
    /// Panics if the iterator has a length of less than 2.
    #[inline]
    fn insert_loop<'a>(
        &'a mut self,
        iter: impl IntoIterator<Item = (T::EP, T::VP)>,
    ) -> ValidEdgeCursorMut<'a, T>
    where
        T: 'a,
    {
        let mut iter = iter.into_iter();
        let (ep, vp) = iter.next().unwrap();
        let (second_e, last_e) = self.insert_path(vp, iter);
        let first_e = self.insert_edge_ee(last_e, second_e, ep).unwrap();
        debug_assert_eq!(
            self.edge(first_e).unwrap().target_id(),
            self.edge(second_e).unwrap().origin_id()
        );
        self.edge_mut(first_e).unwrap()
    }

    /// Same as `insert_loop` but uses the default edge payload.
    #[inline]
    fn insert_loop_default<'a>(
        &'a mut self,
        iter: impl IntoIterator<Item = T::VP>,
    ) -> ValidEdgeCursorMut<'a, T>
    where
        T::EP: DefaultEdgePayload,
        T: 'a,
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
