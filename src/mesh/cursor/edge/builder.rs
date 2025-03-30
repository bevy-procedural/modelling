use super::EdgeCursorData;
use crate::{
    math::IndexType,
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, HalfEdge, MeshType, MeshTypeHalfEdge,
    },
    prelude::{MeshExtrude, MeshLoft},
};

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the edge etc. using a chaining syntax.
pub trait EdgeCursorBuilder<'a, T: MeshType>: EdgeCursorData<'a, T> {
    /// Tries to remove the current edge.
    ///
    /// If the edge was successfully removed or didn't exist, returns `None`.
    /// Otherwise, returns an edge cursor still pointing to the same edge.
    ///
    /// See [MeshBuilder::try_remove_edge] for more information.
    #[inline]
    #[must_use]
    fn remove(self) -> Option<Self> {
        let Some(valid) = self.load() else {
            return self.void();
        };

        if valid.mesh().try_remove_edge(valid.id()) {
            None
        } else {
            Some(valid)
        }
    }

    /// "Recursively" removes the edge and all adjacent faces.
    /// If you want to preserve the faces, use [EdgeCursorMut::collapse] instead.
    /// Won't do anything if the cursor is void.
    ///
    /// Moves the cursor to the next edge.
    /// If the next edge is the same as the current twin, the cursor will be void.
    ///
    /// See [MeshBuilder::remove_edge_r] for more information.
    #[inline]
    #[must_use]
    fn remove_r(self) -> Self::Maybe
    where
        T::Edge: HalfEdge<T>,
    {
        let Some(valid) = self.load() else {
            return self.void();
        };

        let id = valid.id();
        let c = if valid.next_id() == valid.twin_id() {
            valid.void()
        } else {
            valid.next()
        };
        c.mesh.remove_edge_r(id);
        c
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
        let Some(valid) = self.load() else {
            return self;
        };

        let old_target = valid.target_id();
        let (e, _v) = valid
            .mesh()
            .insert_vertex_e(valid.id(), vp, ep)
            .expect("Failed to insert vertex in half-edge mesh.");
        let c = valid.move_to(e).load().unwrap();
        debug_assert!(old_target == c.origin_id());
        c
    }

    /// Connects the current halfedge to the given halfedge.
    /// Returns the void cursor if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ee] for more information.
    #[inline]
    #[must_use]
    fn connect(self, other: T::E, ep: T::EP) -> Self::Maybe {
        let Some(valid) = self.load() else {
            return self.void();
        };

        let Some(e) = valid.mesh().insert_edge_ee(valid.id(), other, ep) else {
            return valid.void();
        };
        valid.move_to(e)
    }

    /// Connects the current halfedge to the given vertex.
    /// Returns the void cursor if the connection was not successful or the cursor was void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    fn connect_v(self, other: T::V, ep: T::EP) -> Self::Maybe {
        let Some(valid) = self.load() else {
            return self.void();
        };

        let Some(e) = valid.mesh().insert_edge_ev(valid.id(), other, ep) else {
            return valid.void();
        };
        valid.move_to(e)
    }

    /// Inserts a face in the boundary of the current halfedge and move the cursor to the new face.
    /// If the face already exists, move there and return that cursor instead.
    ///
    /// If the cursor was void or an error occurs, return the void cursor.
    ///
    /// See [MeshBuilder::insert_face] for more information.
    #[inline]
    #[must_use]
    fn insert_face(self, fp: T::FP) -> FaceCursorMut<'a, T>
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        let Some(valid) = self.load() else {
            return self.face().void();
        };

        if valid.has_face() {
            valid.face()
        } else if let Some(f) = valid.mesh().insert_face(valid.id(), fp) {
            valid.move_to_face(f)
        } else {
            valid.face().void()
        }
    }

    /// Sets the face of the edge in the mesh even if it already has a face.
    /// Calling this method with `IndexType::max()` will remove the face.
    ///
    /// Doesn't do anything if the cursor is void.
    #[inline]
    fn replace_face(self, face: T::F) -> Self
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T::Edge: HalfEdge<T>,
    {
        let Some(valid) = self.load() else {
            return self;
        };

        let f = if valid.has_face() {
            valid.remove_face()
        } else {
            valid
        };
        if face != IndexType::max() {
            f.set_face(face);
        }

        self
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
        let Some(valid) = self.load() else {
            return self;
        };

        valid.mesh().edge_ref_mut(valid.id()).remove_face();
        self
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
        let Some(valid) = self.load() else {
            return self.void();
        };

        let Some((e, _f)) = valid.mesh().close_face_ev(valid.id(), v, ep, fp) else {
            return valid.void();
        };
        valid.move_to(e)
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
        let Some(valid) = self.load() else {
            return self.void();
        };

        let Some((e, _f)) = valid.mesh().close_face_ee(valid.id(), e, ep, fp) else {
            return valid.void();
        };
        valid.move_to(e)
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
        let Some(valid) = self.load() else {
            return self;
        };
        let e = valid.mesh().subdivide_edge(valid.id(), vp, ep);
        valid.move_to(e).load().unwrap()
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
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(e) = valid.mesh().collapse_edge(valid.id()) else {
            return self.void();
        };
        valid.move_to(e)
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
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(e) = valid.mesh().subdivide_face(valid.id(), output, ep, fp) else {
            return valid.void();
        };
        valid.move_to(e)
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
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(e) = valid
            .mesh()
            .subdivide_face_v(valid.face_id(), valid.target_id(), v, ep, fp)
        else {
            return valid.void();
        };
        valid.move_to(e)
    }

    /// Appends multiple edges to the current edge given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last edge (such that the last inserting vertex is the target).
    /// If the iterator is empty, don't move the cursor.
    /// Panics if the cursor is void.
    #[inline]
    #[must_use]
    fn append_path(self, iter: impl IntoIterator<Item = (T::VP, T::EP)>) -> Self {
        let Some(valid) = self.load() else {
            return self;
        };

        let mut c = valid;
        for (vp, ep) in iter {
            c = c.insert_vertex(vp, ep);
        }
        c
    }

    /// Applies `crochet(current_edge, n, m, true, false, false, vp)`.
    /// See [MeshLoft::crochet] for more information.
    ///
    /// Moves to the first edge of the new boundary.
    #[inline]
    #[must_use]
    fn loft(self, n: usize, m: usize, vp: impl IntoIterator<Item = T::VP>) -> Self::Maybe
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshLoft<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some((first, _last)) = valid
            .mesh()
            .crochet(valid.id(), n, m, false, true, false, vp)
        else {
            return valid.void();
        };
        valid.move_to(first)
    }

    /// Applies `self.crochet(start, n, m, true, true, false, vp)`.
    /// See [MeshLoft::crochet] for more information.
    ///
    /// Moves to the first edge of the new boundary.
    #[inline]
    #[must_use]
    fn loft_back(self, n: usize, m: usize, vp: impl IntoIterator<Item = T::VP>) -> Self::Maybe
    where
        T: MeshTypeHalfEdge,
        T::Mesh: MeshLoft<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
    {
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some((first, _last)) = valid
            .mesh()
            .crochet(valid.id(), n, m, true, true, false, vp)
        else {
            return valid.void();
        };
        valid.move_to(first)
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
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(_) = valid.mesh().windmill(valid.id(), vp) else {
            return valid.void();
        };
        valid
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
        let Some(valid) = self.load() else {
            return self.void();
        };
        let Some(_) = valid.mesh().windmill_back(valid.id(), vp) else {
            return self.void();
        };
        self
    }
}
