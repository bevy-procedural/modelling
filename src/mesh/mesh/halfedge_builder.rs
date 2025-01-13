use crate::mesh::{
    CursorData, EdgeCursorBasics, EdgeCursorHalfedgeBasics, MeshBasics, MeshTypeHalfEdge,
};

/// Some low-level operations to build meshes with halfedges.
pub trait MeshHalfEdgeBuilder<T: MeshTypeHalfEdge<Mesh = Self>>: MeshBasics<T> {
    /// Removes a half-edge from the mesh.
    /// Panics if the half-edge is not found or still connected to a face.
    #[inline]
    fn remove_halfedge(&mut self, e: T::E) {
        assert!(
            self.try_remove_halfedge(e),
            "Could not remove halfedge {}",
            e
        );
    }

    /// Tries to remove a half-edge from the mesh.
    /// Returns `true` if the half-edge was removed.
    /// Won't update the neighbors!
    fn try_remove_halfedge(&mut self, e: T::E) -> bool;

    /// Allocates and inserts a pair of half-edges and returns the ids.
    /// This will not update the neighbors and will not check whether the operation is allowed!
    /// After this operation, the mesh might be in an inconsistent state.
    ///
    /// You can set `to_target`, `to_origin`, `from_target`, and `from_origin` to `IndexType::max()`
    /// to use the resp. twin in theses places.
    ///
    /// Doesn't check whether the operation is allowed.
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

    /// Allocates and inserts a pair of half-edges and returns the ids.
    /// This will not update the neighbors!
    /// After this operation, the mesh might be in an inconsistent state.
    ///
    /// Only on debug builds, this method will check that the operation makes
    /// sense, i.e., the vertices exist and the edges are correctly connected.
    /// Doesn't care about the faces.    
    #[inline]
    fn try_insert_halfedge_pair(
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
    ) -> Result<(T::E, T::E), String> {
        {
            if !self.has_vertex(origin) {
                return Err(format!("Origin Vertex {} does not exist", origin));
            }
            if !self.has_vertex(target) {
                return Err(format!("Target Vertex {} does not exist", target));
            }
            if !self.has_edge(to_origin) {
                return Err(format!("to_origin Edge {} does not exist", to_origin));
            }
            if !self.has_edge(from_origin) {
                return Err(format!("from_origin Edge {} does not exist", from_origin));
            }
            if !self.has_edge(to_target) {
                return Err(format!("to_target Edge {} does not exist", to_target));
            }
            if !self.has_edge(from_target) {
                return Err(format!("from_target Edge {} does not exist", from_target));
            }
            let to_origin = self.edge(to_origin);
            let from_origin = self.edge(from_origin);
            let to_target = self.edge(to_target);
            let from_target = self.edge(from_target);
            if to_origin.next_id() != from_origin.id() {
                return Err(format!(
                    "to_origin.next_id() != from_origin.id(): {} != {}",
                    to_origin.id(),
                    from_origin.id()
                ));
            }
            if to_target.next_id() != from_target.id() {
                return Err(format!(
                    "to_target.next_id() != from_target.id(): {} != {}",
                    to_target.id(),
                    from_target.id()
                ));
            }
            if from_origin.prev_id() != to_origin.id() {
                return Err(format!(
                    "from_origin.prev_id() != to_origin.id(): {} != {}",
                    from_origin.id(),
                    to_origin.id()
                ));
            }
            if from_target.prev_id() != to_target.id() {
                return Err(format!(
                    "from_target.prev_id() != to_target.id(): {} != {}",
                    from_target.id(),
                    to_target.id()
                ));
            }
            if to_origin.target_id() != origin {
                return Err(format!(
                    "to_origin.target_id() != origin: {} != {}",
                    to_origin.target_id(),
                    origin
                ));
            }
            if from_origin.origin_id() != origin {
                return Err(format!(
                    "from_origin.origin_id() != origin: {} != {}",
                    from_origin.origin_id(),
                    origin
                ));
            }
            if to_target.target_id() != target {
                return Err(format!(
                    "to_target.target_id() != target: {} != {}",
                    to_target.target_id(),
                    target
                ));
            }
            if from_target.origin_id() != target {
                return Err(format!(
                    "from_target.origin_id() != target: {} != {}",
                    from_target.origin_id(),
                    target
                ));
            }
        }
        Ok(self.insert_halfedge_pair_forced(
            to_origin,
            origin,
            from_origin,
            to_target,
            target,
            from_target,
            forward_face,
            backward_face,
            ep,
        ))
    }

    /// Allocates and inserts a pair of half-edges and returns the ids.
    /// This will not update the neighbors!
    /// After this operation, the mesh might be in an inconsistent state.
    ///
    /// Only on debug builds, this method will check that the operation makes
    /// sense, i.e., the vertices exist and the edges are correctly connected.
    /// Doesn't care about the faces.
    /// On failure, it will panic.
    #[inline]
    fn insert_halfedge_pair(
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
    ) -> (T::E, T::E) {
        #[cfg(debug_assertions)]
        match self.try_insert_halfedge_pair(
            to_origin,
            origin,
            from_origin,
            to_target,
            target,
            from_target,
            forward_face,
            backward_face,
            ep,
        ) {
            Ok(pair) => pair,
            Err(msg) => panic!("Couldn't insert half-edges: {}", msg),
        }
        #[cfg(not(debug_assertions))]
        self.insert_halfedge_pair_forced(
            to_origin,
            origin,
            from_origin,
            to_target,
            target,
            from_target,
            forward_face,
            backward_face,
            ep,
        )
    }

    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    /// Returns the newly inserted edge.
    fn subdivide_halfedge(&mut self, e: T::E, vp: T::VP, ep: T::EP) -> T::E;

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    fn subdivide_halfedge_try_fixup(&mut self, e: T::E, ep: T::EP) -> Option<T::E>;
}
