use crate::mesh::{cursor::*, HalfEdge, MeshType, Vertex};

pub trait VertexCursorBuilder<'a, T: MeshType>: VertexCursorData<'a, T> {
    /// Appends multiple edges to the current vertex given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last inserted vertex.
    /// If the iterator is empty, don't move the cursor.
    /// Panics if the cursor is void.
    ///
    #[inline]
    fn append_path(self, iter: impl IntoIterator<Item = (T::EP, T::VP)>) -> Self {
        if let Some((_first_e, last_e)) = self.mesh.append_path(self.id(), iter) {
            let id = self.mesh.edge_ref(last_e).target_id(self.mesh());
            self.move_to(id)
        } else {
            self
        }
    }

    /// Inserts a new vertex and a edge connecting the current vertex and the new vertex.
    /// Move to the new vertex.
    ///
    /// Returns `None` if the insertion wasn't successful.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_vertex_v] for more information.
    #[inline]
    #[must_use]
    fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Option<Self> {
        let (_e, v) = self.mesh.insert_vertex_v(self.id(), vp, ep)?;
        Some(self.move_to(v))
    }

    /// Connects the current vertex and the given vertex with an edge.
    ///
    /// Moves the cursor to the newly inserted edge.
    /// Returns void if the cursor is void or the connectivity wasn't clear.
    ///
    /// See [MeshBuilder::insert_edge_vv] for more information.
    #[inline]
    #[must_use]
    fn connect(self, target: T::V, ep: T::EP) -> EdgeCursorMut<'a, T> {
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(e) = valid.mesh().insert_edge_vv(valid.id(), target, ep) else {
            return valid.void();
        };
        valid.move_to_edge(e)
    }

    /// Connects the current vertex and the given edge's origin with an edge.
    ///
    /// Moves the cursor to the newly inserted edge leading from the current vertex to the edge's origin.
    /// Returns `None` if the connectivity wasn't clear.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    fn connect_e(self, target: T::E, ep: T::EP) -> Option<EdgeCursorMut<'a, T>>
    where
        T::Edge: HalfEdge<T>,
    {
        let target_twin = self.mesh().edge_ref(target).twin_id();
        let e = self.mesh().insert_edge_ev(target_twin, self.id(), ep)?;
        Some(self.move_to_edge(e).twin())
    }

    /// Subdivide the given face by inserting an edge from the current vertex to the given vertex.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns `None` if the resulting faces would've been invalid or the vertices don't share the given face.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::subdivide_face_v] for more information.
    #[inline]
    #[must_use]
    fn subdivide_face(
        self,
        f: T::F,
        other: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<EdgeCursorMut<'a, T>> {
        let e = self.mesh.subdivide_face_v(f, self.id(), other, ep, fp)?;
        Some(self.move_to_edge(e))
    }
}
