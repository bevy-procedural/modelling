use super::EdgeCursorData;
use crate::{
    math::IndexType,
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, EdgeBasics, HalfEdge, MeshBuilder,
        MeshType, MeshTypeHalfEdge,
    },
    prelude::{MeshExtrude, MeshLoft},
};
use std::ops::Not;

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the edge etc. using a chaining syntax.
pub trait EdgeCursorBuilder<'a, T: MeshType>:
    EdgeCursorData<'a, T> + MutableCursor<T = T, I = T::E, S = T::Edge>
where
    T::Mesh: MeshBuilder<T>,
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>
        + MutableCursor<T = T, I = T::E, S = T::Edge>
        + ValidEdgeCursorBasics<'a, T>
        + ValidCursorMut<T = T, I = T::E, S = T::Edge>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>
        + MutableCursor<T = T, I = T::E, S = T::Edge>,
{
    /// Tries to remove the current edge.
    ///
    /// If the edge was successfully removed or didn't exist, returns void.
    /// Otherwise, returns an edge cursor still pointing to the same edge.
    ///
    /// See [MeshBuilder::try_remove_edge] for more information.
    #[inline]
    #[must_use]
    fn remove(self) -> Self::Maybe {
        self.load_move_or_void(|valid, id| valid.mesh_mut().try_remove_edge(id).not().then(|| id))
    }
    /// Inserts a new vertex and half-edge pair. The halfedge leading to the
    /// new vertex will become the "next" of the current edge and the cursor will move
    /// to this newly created halfedge.
    ///
    /// Returns the void cursor if the cursor was void.
    /// Panics if the insertion was not successful (which cannot happend on half-edge meshes since the connectivity is unambiguous).
    ///
    /// See [MeshBuilder::insert_vertex_e] for more information.
    #[inline]
    #[must_use]
    fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Self {
        self.load_or_nop(|mut valid: Self::Valid| {
            let old_target = valid.target_id();
            let id = valid.id();
            let (e, _v) = valid
                .mesh_mut()
                .insert_vertex_e(id, vp, ep)
                .expect("Failed to insert vertex in half-edge mesh.");
            let c = Self::from_maybe(valid.move_to(e));
            debug_assert!(old_target == c.try_inner().unwrap().origin_id(c.mesh()));
            c
        })
    }

    /// Connects the current halfedge to the given halfedge.
    /// Returns the void cursor if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ee] for more information.
    #[inline]
    #[must_use]
    fn connect(self, other: T::E, ep: T::EP) -> Self::Maybe {
        self.load_move_or_void(|valid, id| valid.mesh_mut().insert_edge_ee(id, other, ep))
    }

    /// Connects the current halfedge to the given vertex.
    /// Returns the void cursor if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    fn connect_v(self, other: T::V, ep: T::EP) -> Self::Maybe {
        self.load_move_or_void(|valid, id| valid.mesh_mut().insert_edge_ev(id, other, ep))
    }

    /// Removes the face from the edge.
    /// Won't update the neighbors - so the mesh will be invalid after this operation.
    ///
    /// Doesn't do anything if the cursor is void.
    ///
    /// Doesn't delete the face itself from the mesh.
    /// Use `c.face().remove()` to delete the face from the mesh and remove it from all adjacent edges.
    #[inline]
    fn remove_face(self) -> Self
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        self.load_or_nop(|mut valid| {
            valid.inner_mut().remove_face();
            Self::from_valid(valid)
        })
    }

    /// Insert an edge to the given vertex and move the cursor to that new edge.
    /// Close the resulting face with the given face payload.
    ///
    /// Return the void cursor if the insertion failed or the cursor is void.
    ///
    /// See [MeshBuilder::close_face_ev] for more information.
    #[inline]
    #[must_use]
    fn close_face_v(self, v: T::V, ep: T::EP, fp: T::FP) -> Self::Maybe {
        self.load_move_or_void(|valid, id| {
            valid
                .mesh_mut()
                .close_face_ev(id, v, ep, fp)
                .map(|(e, _f)| e)
        })
    }

    /// Insert an edge to the given edge's input and move the cursor to that new edge.
    /// Close the resulting face with the given face payload.
    ///
    /// Return the void cursor if the insertion failed or the cursor is void.
    ///
    /// See [MeshBuilder::close_face_ee] for more information.
    #[inline]
    #[must_use]
    fn close_face(self, e: T::E, ep: T::EP, fp: T::FP) -> Self::Maybe {
        self.load_move_or_void(|mut valid, id| {
            valid
                .mesh_mut()
                .close_face_ee(id, e, ep, fp)
                .map(|(e, _f)| e)
        })
    }

    /// Subdivides the given edge by inserting a vertex in the middle, using
    /// that vertex as the new target and inserting a new edge from the middle vertex
    /// to the original target.
    ///
    /// Moves the cursor to the new edge (the original edge will be the `prev` of the new edge).
    ///
    /// See [MeshBuilder::subdivide_edge] for more information.
    #[inline]
    #[must_use]
    fn subdivide(self, vp: T::VP, ep: T::EP) -> Self {
        self.load_or_nop(|mut valid| {
            let id = valid.id();
            let e = valid.mesh_mut().subdivide_edge(id, vp, ep);
            Self::from_maybe(valid.move_to(e))
        })
    }

    /// Collapses the edge with the next edge.
    /// Keeps the payload of the current edge.
    ///
    /// If the target of the current edge doesn't have a degree of 2, the operation will fail and return the void cursor.
    ///
    /// Doesn't move the cursor.
    ///
    /// See [MeshBuilder::collapse_edge] for more information.
    #[inline]
    #[must_use]
    fn collapse(self) -> Self::Maybe {
        self.load_move_or_void(|valid, id| valid.mesh_mut().collapse_edge(id))
    }

    /// Subdivide the adjacent face by inserting an edge from the current target to the given other edge's origin.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    ///
    /// Returns the void cursor if the other edge is not adjacent to the same face or the resulting faces would've been invalid.
    ///
    /// See [MeshBuilder::subdivide_face] for more information.
    #[inline]
    #[must_use]
    fn subdivide_face(self, output: T::E, ep: T::EP, fp: T::FP) -> Self::Maybe
    where
        T::Edge: HalfEdge<T>,
    {
        self.load_move_or_void(|valid: &mut _, id| {
            valid.mesh_mut().subdivide_face(id, output, ep, fp)
        })
    }

    /// Appends multiple edges to the current edge given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last edge (such that the last inserting vertex is the target).
    /// If the iterator is empty, don't move the cursor.
    /// If the cursor is void, do nothing and return void.
    #[inline]
    #[must_use]
    fn append_path(self, iter: impl IntoIterator<Item = (T::VP, T::EP)>) -> Self
    where
        Self::Valid: EdgeCursorBuilder<'a, T>,
    {
        self.load_or_nop(|valid| {
            let mut c = valid;
            for (vp, ep) in iter {
                c = c.insert_vertex(vp, ep);
            }
            Self::from_valid(c)
        })
    }

    /// Applies `crochet(current_edge, n, m, true, false, false, vp)`.
    /// See [MeshLoft::crochet] for more information.
    ///
    /// Moves to the first edge of the new boundary.
    #[inline]
    #[must_use]
    fn loft(self, n: usize, m: usize, vp: impl IntoIterator<Item = T::VP>) -> Self
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshLoft<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        // TODO: is self from_maybe ok?
        Self::from_maybe(self.load_move_or_void(|valid: &mut Self::Valid, id| {
            (valid.mesh_mut().crochet(id, n, m, false, true, false, vp) as Option<_>)
                .map(|(first, _last)| first)
        }))
    }

    /// Applies `self.crochet(start, n, m, true, true, false, vp)`.
    /// See [MeshLoft::crochet] for more information.
    ///
    /// Moves to the first edge of the new boundary.
    #[inline]
    #[must_use]
    fn loft_back(self, n: usize, m: usize, vp: impl IntoIterator<Item = T::VP>) -> Self
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshLoft<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        // TODO: is self from_maybe ok?
        Self::from_maybe(self.load_move_or_void(|valid, id| {
            (valid.mesh_mut().crochet(id, n, m, true, true, false, vp) as Option<_>)
                .map(|(first, _last)| first)
        }))
    }

    /// See [MeshExtrude::windmill].
    /// Doesn't move the cursor.
    /// Returns the void cursor if the operation failed or the cursor was void.
    #[inline]
    #[must_use]
    fn windmill(self, vp: T::VP) -> Self::Maybe
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshExtrude<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        // TODO: Return valid cursor?
        self.load_move_or_void(|valid, id| valid.mesh_mut().windmill(id, vp).map(|_| id))
    }

    /// See [MeshExtrude::windmill_back].
    /// Doesn't move the cursor.
    /// Returns the void cursor if the operation failed or the cursor was void.
    #[inline]
    #[must_use]
    fn windmill_back(self, vp: T::VP) -> Self::Maybe
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshExtrude<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        // TODO: Return valid cursor?
        self.load_move_or_void(|valid, id| valid.mesh_mut().windmill_back(id, vp).map(|_| id))
    }
}

pub trait EdgeCursorHalfedgeBuilder<'a, T: MeshType>:
    EdgeCursorData<'a, T> + MutableCursor<T = T, I = T::E, S = T::Edge>
where
    T::Mesh: MeshBuilder<T>,
    T::Edge: HalfEdge<T>,
    // TODO: We should remove or reduce this bound by implementing face_id for all edges
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>
        + MutableCursor<T = T, I = T::E, S = T::Edge>
        + ValidEdgeCursorBasics<'a, T>
        + EdgeCursorHalfedgeBasics<'a, T>
        + ValidEdgeCursorHalfedgeBasics<'a, T>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>
        + MutableCursor<T = T, I = T::E, S = T::Edge>
        + EdgeCursorHalfedgeBasics<'a, T>,
{
    /// "Recursively" removes the edge and all adjacent faces.
    /// If you want to preserve the faces, use [EdgeCursorMut::collapse] instead.
    /// Won't do anything if the cursor is void.
    ///
    /// Moves the cursor to the next edge.
    /// If the next edge is the same as the current twin, the cursor will be void, even if the edge was removed successfully.
    ///
    /// See [MeshBuilder::remove_edge_r] for more information.
    #[inline]
    #[must_use]
    fn remove_r(self) -> Self::Maybe {
        self.load_or_void(|valid: Self::Valid| {
            let id = valid.id();
            let mut c = if valid.next_id() == valid.twin_id() {
                valid.void()
            } else {
                valid.next()
            };
            c.mesh_mut().remove_edge_r(id);
            c
        })
    }

    /// Inserts a face in the boundary of the current halfedge and move the cursor to the new face.
    ///
    /// If the face already exists, overwrite it with the new payload.
    ///
    /// If the cursor was void or an error occurs, return the void cursor.
    ///
    /// See [MeshBuilder::insert_face] for more information.
    #[inline]
    #[must_use]
    fn insert_face(self, fp: T::FP) -> Self::FC
    where
        // TODO: can we avoid this constraint?
        <<Self::Valid as EdgeCursorData<'a, T>>::FC as CursorData>::Valid:
            ValidFaceCursorBasics<'a, T> + ValidCursorMut,
        T: 'a,
    {
        self.load_or_else(
            |c| c.move_to_face(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                if valid.has_face() {
                    // Replace the face payload
                    let mut vfc = valid.face().unwrap();
                    *vfc.payload_mut() = fp;
                    vfc.maybe()
                } else if let Some(f) = valid.mesh_mut().insert_face(id, fp) {
                    valid.move_to_face(f)
                } else {
                    valid.face().void()
                }
            },
        )
    }

    /// Sets the face of the edge in the mesh even if it already has a face.
    /// Calling this method with `IndexType::max()` will remove the face.
    ///
    /// Doesn't do anything if the cursor is void.
    #[inline]
    fn replace_face(self, face: T::F) -> Self
    where
        // TODO: can we avoid this constraint?
        Self::Valid: ValidEdgeCursorBasics<'a, T> + ValidCursorMut,
        T: 'a,
    {
        self.load_or_nop(|mut valid| {
            if valid.has_face() {
                valid.inner_mut().remove_face();
            }
            if face != IndexType::max() {
                valid.inner_mut().set_face(face);
            }
            Self::from_valid(valid)
        })
    }

    /// Subdivide the adjacent face by inserting an edge from the current target to the given vertex.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns the void cursor if the resulting faces would've been invalid.
    ///
    /// See [MeshBuilder::subdivide_face_v] for more information.
    #[inline]
    #[must_use]
    fn subdivide_face_v(self, v: T::V, ep: T::EP, fp: T::FP) -> Self::Maybe
    where
        T::Edge: HalfEdge<T>,
    {
        self.load_move_or_void(|valid, _id| {
            let target_id = valid.target_id();
            let face_id = valid.face_id();
            valid
                .mesh_mut()
                .subdivide_face_v(face_id, target_id, v, ep, fp)
        })
    }
}
