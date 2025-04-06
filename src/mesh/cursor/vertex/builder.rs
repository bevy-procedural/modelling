use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshBuilder, MeshType},
};

/// Methods specific to mutable vertex cursors, i.e., to modify the mesh.
pub trait VertexCursorBuilder<'a, T: MeshType>:
    VertexCursorData<'a, T> + MutableCursor<T = T, I = T::V, S = T::Vertex>
where
    T::Mesh: MeshBasics<T>,
    Self::Valid: MutableCursor<T = T, I = T::V, S = T::Vertex>
        + VertexCursorData<
            'a,
            T,
            EC = Self::EC,
            FC = Self::FC,
            I = Self::I,
            S = Self::S,
            Payload = T::VP,
            Maybe = Self::Maybe,
            Valid = Self::Valid,
        >,
{
    /// Appends multiple edges to the current vertex given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last inserted vertex.
    /// If the iterator is empty, don't move the cursor.
    /// Doesn't do anything if the cursor is void.
    #[inline]
    fn append_path(self, iter: impl IntoIterator<Item = (T::EP, T::VP)>) -> Self {
        self.load_or_nop(|mut valid| {
            let id = valid.id();
            if let Some((_first_e, last_e)) = valid.mesh_mut().append_path(id, iter) {
                let id = valid.mesh().edge(last_e).unwrap().target_id();
                Self::from_maybe(valid.move_to(id))
            } else {
                Self::from_valid(valid)
            }
        })
    }

    /// Inserts a new vertex and a edge connecting the current vertex and the new vertex.
    /// Move to the new vertex.
    ///
    /// Returns void if the insertion wasn't successful.
    /// Will insert a single unconnected vertex if the cursor is void.
    ///
    /// See [MeshBuilder::insert_vertex_v] for more information.
    #[inline]
    #[must_use]
    fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Self::Maybe {
        self.load_or_else(
            |mut cursor| {
                let v = cursor.mesh_mut().insert_vertex(vp.clone());
                cursor.move_to(v)
            },
            |mut valid| {
                let id = valid.id();
                let Some((_e, v)) = valid.mesh_mut().insert_vertex_v(id, vp.clone(), ep) else {
                    return valid.void();
                };
                valid.move_to(v)
            },
        )
    }

    /// Connects the current vertex and the given vertex with an edge.
    ///
    /// Moves the cursor to the newly inserted edge.
    /// Returns void if the cursor is void or the connectivity wasn't clear.
    ///
    /// See [MeshBuilder::insert_edge_vv] for more information.
    #[inline]
    #[must_use]
    fn connect(self, target: T::V, ep: T::EP) -> Self::EC {
        self.load_or_else(
            |cursor| cursor.move_to_edge(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                if let Some(e) = valid.mesh_mut().insert_edge_vv(id, target, ep) {
                    valid.move_to_edge(e)
                } else {
                    valid.move_to_edge(IndexType::max())
                }
            },
        )
    }

    /// Connects the current vertex and the given edge's origin with an edge.
    ///
    /// Moves the cursor to the newly inserted edge leading from the current vertex to the edge's origin.
    /// Returns void if the connectivity wasn't clear or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    fn connect_e(self, target: T::E, ep: T::EP) -> Self::EC
    where
        Self::EC: EdgeCursorData<'a, T> + EdgeCursorHalfedgeBasics<'a, T>,
        <Self::EC as CursorData>::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self>,
        <Self::EC as CursorData>::Maybe:
            EdgeCursorData<'a, T, FC = Self::FC, VC = Self> + EdgeCursorHalfedgeBasics<'a, T>,
        T::Edge: HalfEdge<T>,
    {
        self.load_or_else(
            |cursor| cursor.move_to_edge(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                let target_twin = valid.mesh().edge_ref(target).twin_id();
                if let Some(e) = valid.mesh_mut().insert_edge_ev(target_twin, id, ep) {
                    valid.move_to_edge(e).twin()
                } else {
                    valid.move_to_edge(IndexType::max())
                }
            },
        )
    }

    /// Subdivide the given face by inserting an edge from the current vertex to the given vertex.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns void if the resulting faces would've been invalid or the vertices don't share the given face.
    ///
    /// See [MeshBuilder::subdivide_face_v] for more information.
    #[inline]
    #[must_use]
    fn subdivide_face(self, f: T::F, other: T::V, ep: T::EP, fp: T::FP) -> Self::EC {
        self.load_or_else(
            |cursor| cursor.move_to_edge(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                if let Some(e) = valid.mesh_mut().subdivide_face_v(f, id, other, ep, fp) {
                    valid.move_to_edge(e)
                } else {
                    valid.move_to_edge(IndexType::max())
                }
            },
        )
    }
}
