use crate::mesh::{EdgeBasics, VertexBasics};

use super::{MeshBasics, MeshType};

/// Some basic operations to build meshes.
pub trait MeshBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// add a new vertex and return it's id
    fn insert_vertex(&mut self, vp: T::VP) -> T::V;

    /// Removes the vertex `v` and returns its payload.
    /// Panics if the vertex doesn't exist or if it is not isolated.
    #[inline(always)]
    fn remove_vertex(&mut self, v: T::V) -> T::VP {
        self.try_remove_vertex(v).unwrap()
    }

    /// Tries to remove the vertex `v` and returns its payload if successful.
    /// Fails if the vertex is not isolated.
    fn try_remove_vertex(&mut self, v: T::V) -> Option<T::VP>;

    /// Inserts vertices a and b and adds an isolated edge between a and b.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from `a` to `b`. This is also the half-edge that is returned.
    #[inline(always)]
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
    #[inline(always)]
    fn insert_vertex_v(&mut self, v: T::V, vp: T::VP, ep: T::EP) -> Option<(T::E, T::V)> {
        let w = self.insert_vertex(vp);
        if let Some(e) = self.insert_edge_vv(v, w, ep) {
            Some((e, w))
        } else {
            self.remove_vertex(w);
            None
        }
    }

    /// Creates a new vertex based on `vp` and connects it to edge `e`.
    /// Returns the new edge and vertex id.
    ///
    /// Shouldn't fail for half-edge meshes since the connectivity is never ambiguous.
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the current vertex to the new vertex. This is also the half-edge that is returned.
    #[inline(always)]
    fn insert_vertex_e(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> Option<(T::E, T::V)> {
        let v = self.insert_vertex(vp);
        self.insert_edge_ev(e, v, ep).map(|e| (e, v))
    }

    /// Connects the vertices `a` and `b` with an edge and returns the edge id.
    /// This will not close any face! The method will not check whether the vertices
    /// are in different connected components, so, you can generate non-manifold meshes
    /// using this method.
    ///
    /// Fails if the connectivity is ambiguous, i.e., if `a` and `b` both have edges but
    /// are connected by more than exactly one boundary, e.g., when they are in different
    /// connected components such that chirality is ambiguous or when there is more than one
    /// boundary cycle passing through both vertices.
    ///
    /// Notice that this boundary checks can be costly if you have large faces!
    ///
    /// For half-edge meshes, the payload will be added to the outgoing half-edge
    /// from the `a` to `b`. This is also the half-edge that is returned.
    fn insert_edge_vv(&mut self, a: T::V, b: T::V, ep: T::EP) -> Option<T::E>;

    /// Inserts an edge from the target of `input` to the origin of `output`.
    ///
    /// Connectivity is inferred from the graph. In case of half-edge meshes, the
    /// method should never fail since connectivity is never ambiguous.
    ///
    /// For half-edge meshes, the payload will be added to the half-edge
    /// from the `input` to `output`. This is also the half-edge that is returned.
    fn insert_edge_ee(&mut self, input: T::E, output: T::E, ep: T::EP) -> Option<T::E>;

    /// Inserts an edge from the target of `input` to the origin of `output`.
    ///
    /// It behaves similar to `insert_edge_ee` but will not run any checks to see if the insertion is valid!
    /// This allows you to run this method on invalid meshes given that the `next` of the two edges is valid.
    fn insert_edge_ee_forced(&mut self, input: T::E, output: T::E, ep: T::EP) -> T::E;

    /// Inserts an edge from the target of `e` to the vertex `v`.
    ///
    /// Connectivity at `v` is inferred from the graph. Behaves similar to `insert_edge_vv`.
    fn insert_edge_ev(&mut self, e: T::E, v: T::V, ep: T::EP) -> Option<T::E>;

    /// Removes the edge `e` and returns its payload.
    /// Panics if the edge doesn't exist or there are adjacent faces.
    #[inline(always)]
    fn remove_edge(&mut self, e: T::E) -> T::EP {
        self.try_remove_edge(e).unwrap()
    }

    /// Tries to remove the edge `e` and returns its payload if successful.
    /// Fails if there are adjacent faces.
    fn try_remove_edge(&mut self, e: T::E) -> Option<T::EP>;

    ////////////////////////////////////////////////////////////////////////////////////
    //************************ Face-related ******************************************//

    /// Removes the face `f` and returns its payload if successful.
    /// Fails only if the face doesn't exist.
    fn remove_face(&mut self, f: T::F) -> Option<T::FP>;

    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    /// Fails if there is already a face using the boundary.
    fn insert_face(&mut self, e: T::E, fp: T::FP) -> Option<T::F>;

    /// Close the given boundary by inserting an edge from `from.target` to
    /// `to.origin` and insert a face.
    ///
    /// There must be exactly one boundary path from `to` to `from` without a face.
    /// This boundary will be used to construct the face.
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from.target` to `to.origin`.
    fn close_face_ee(&mut self, from: T::E, to: T::E, ep: T::EP, fp: T::FP)
        -> Option<(T::E, T::F)>;

    /// Close the given boundary by inserting an edge from `from` to `to` and insert a face.
    ///
    /// There must be exactly one edge chain from `to` to `from` without a face.
    /// Otherwise, the method will return `None`.
    ///
    /// Returns the new face and edge id. For half-edge meshes, this should be the half-edge
    /// on the inside of the face, i.e., the half-edge directed from `from` to `to`.
    fn close_face_vv(&mut self, from: T::V, to: T::V, ep: T::EP, fp: T::FP)
        -> Option<(T::E, T::F)>;

    ////////////////////////////////////////////////////////////////////////////////////
    //************************ More complex operations *******************************//

    /// Subdivide the given edge (resp. half-edge pair) by appending multiple edges
    /// and vertices given by the iterator to the edge's origin vertex and connecting them
    /// with the edge. The edge stays connected to it's original target vertex.
    /// Returns the (half)edge starting in the previous origin
    fn subdivide_edge<I: Iterator<Item = (T::EP, T::VP)>>(&mut self, e: T::E, vs: I) -> T::E;

    /// Append a chain of edges to the vertex `v` from the finite iterator of vertices and edges.
    /// Returns the first edge inserted after `v` as well as the last vertex's id.
    /// If the iterator is empty, the method will return only the vertex `v`.
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
    fn insert_loop(&mut self, iter: impl IntoIterator<Item = (T::EP, T::VP)>) -> T::E {
        let mut iter = iter.into_iter();
        let (ep, vp) = iter.next().unwrap();
        let (e, last_v) = self.insert_path(vp, iter);
        self.insert_edge_vv(last_v, self.edge(e).origin(self).id(), ep)
            .unwrap()
    }
}

/// Some low-level operations to build meshes with halfedges.
pub trait MeshHalfEdgeBuilder<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Inserts a single half-edge with the given id.
    /// This will not update the neighbors and will not check whether the operation is allowed!
    /// After this operation, the mesh might be in an inconsistent state.
    fn insert_halfedge_forced(
        &mut self,
        edge: T::E,
        origin: T::V,
        face: T::F,
        prev: T::E,
        twin: T::E,
        next: T::E,
        ep: Option<T::EP>,
    );

    /// Allocates and inserts a pair of half-edges and returns the ids.
    /// This will not update the neighbors and will not check whether the operation is allowed!
    /// After this operation, the mesh might be in an inconsistent state.
    fn insert_halfedge_pair_forced(
        &mut self,
        to_origin: T::E,
        origin: T::V,
        from_origin: T::E,
        to_target: T::E,
        target: T::V,
        from_target: T::E,
        forward_face: T::F,
        backward_face: T::F,
        ep: T::EP,
    ) -> (T::E, T::E);

    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    fn subdivide_halfedge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E;

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    fn subdivide_halfedge_try_fixup(&mut self, e: T::E, ep: T::EP) -> Option<T::E>;
}
