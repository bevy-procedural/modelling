use crate::{
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdge, MeshType},
    util::CreateEmptyIterator,
};

/// These methods are specific to immutable edge cursors, i.e., they require cloning the edge cursor.
pub trait ImmutableEdgeCursor<'a, T: MeshType + 'a>:
    CursorData<T = T, I = T::E, S = T::Edge> + EdgeCursorBasics<'a, T>
where
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
{
    // TODO: move these to edgecursordata or basics?

    /// Returns face cursors for all faces adjacent to the edge
    /// (including the twin for halfedges and parallel edges' faces if the edge is non-manifold).
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn faces<'b>(&'b self) -> impl Iterator<Item = ValidFaceCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        self.face_ids()
            .map(move |id| ValidFaceCursor::load_new(self.mesh(), id))
    }

    /// Returns face cursors for each edge on the same chain as this edge.
    /// Starts with the current edge.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn chain<'b>(&'b self) -> impl Iterator<Item = ValidEdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.chain(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
        .map(move |e| ValidEdgeCursor::new(self.mesh(), e))
    }

    /// Returns face cursors for each edge on the same chain as this edge.
    /// Starts with the current edge.
    /// Traverses the chain backwards.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn chain_back<'b>(&'b self) -> impl Iterator<Item = ValidEdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.chain_back(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
        .map(move |e| ValidEdgeCursor::new(self.mesh(), e))
    }
}

/// This trait implements some basic functionality for edge cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait EdgeCursorBasics<'a, T: MeshType>: EdgeCursorData<'a, T>
where
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
{
    /// Moves the cursor to the origin vertex of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn origin(self) -> Self::VC {
        let id = if let Some(inner) = self.try_inner() {
            inner.origin_id(self.mesh())
        } else {
            IndexType::max()
        };
        self.move_to_vertex(id)
    }

    /// Moves the cursor to the target vertex of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn target(self) -> Self::VC {
        let id = if let Some(inner) = self.try_inner() {
            inner.target_id(self.mesh())
        } else {
            IndexType::max()
        };
        self.move_to_vertex(id)
    }

    /// Returns the ids of all faces adjacent to the edge
    /// (including the twin for halfedges and parallel edges' faces if the edge is non-manifold).
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn face_ids<'b>(&'b self) -> impl Iterator<Item = T::F> + 'b
    where
        T: 'b,
        'a: 'b,
    {
        if let Some(edge) = self.try_inner() {
            edge.face_ids(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
    }
}

/// This trait implements some basic functionality for edge cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait EdgeCursorHalfedgeBasics<'a, T: MeshType>: EdgeCursorData<'a, T>
where
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
    Self::Maybe:
        EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC> + EdgeCursorHalfedgeBasics<'a, T>,
    T::Edge: HalfEdge<T>,
{
    /// Moves the cursor to the next halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn next(self) -> Self::Maybe {
        self.try_move(|e| e.next_id())
    }

    /// Moves the cursor by calling next `n` times.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn next_n(self, n: usize) -> Self::Maybe {
        let mut cursor = self.maybe();
        for _ in 0..n {
            cursor = cursor.next();
        }
        cursor
    }

    /// Moves the cursor to the previous halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn prev(self) -> Self::Maybe {
        self.try_move(|e| e.prev_id())
    }

    /// Moves the cursor by calling prev `n` times.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn prev_n(self, n: usize) -> Self::Maybe {
        let mut cursor = self.maybe();
        for _ in 0..n {
            cursor = cursor.prev();
        }
        cursor
    }

    /// Moves the cursor to the twin halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn twin(self) -> Self::Maybe {
        self.try_move(|e| e.twin_id())
    }

    /// Moves the cursor to the sibling of the edge, i.e., the twin's next edge.
    /// Calling this repeatedly will return all outgoing halfedges with the same origin.
    /// If the origin is non-manifold, this might not reach all outgoing halfedges but only those in the same wheel.
    /// If you need all wheels, go to the target first. // TODO: Reference
    ///
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn next_sibling(self) -> Self::Maybe {
        self.twin().next()
    }

    /// Moves the cursor to the previous sibling of the edge, i.e., the previous edge's twin.
    ///
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn prev_sibling(self) -> Self::Maybe {
        self.prev().twin()
    }

    /// Moves the cursor to the face of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn face(self) -> Self::FC {
        let id = if let Some(inner) = self.try_inner() {
            inner.face_id()
        } else {
            IndexType::max()
        };
        self.move_to_face(id)
    }

    /// Runs some sanity checks on the edge, i.e., whether the origin and target vertices exist.
    /// Returns false if the edge is void or exists and is malformed.
    /// See [HalfEdge::check] for more information.
    #[inline]
    #[must_use]
    fn check(&self) -> Result<(), String> {
        if let Some(inner) = self.try_inner() {
            inner.check(self.mesh())
        } else {
            Err(format!("Edge {} is invalid", self.id_unchecked()))
        }
    }

    /// Returns an outgoing edge from `v` that is part of the same chain as the edge.
    /// Traverses the chain forwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_chain] for more information.
    #[inline]
    #[must_use]
    fn same_chain(self, v: T::V) -> Option<Self::Maybe> {
        let id = HalfEdge::same_chain(self.try_inner()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }

    /// Returns an outgoing edge from `v` that is part of the same chain as the edge.
    /// Traverses the chain backwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_chain_back] for more information.
    #[inline]
    #[must_use]
    fn same_chain_back(self, v: T::V) -> Option<Self::Maybe> {
        let id = HalfEdge::same_chain_back(self.try_inner()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        extensions::nalgebra::*,
        math::impls::{EU, VU},
        prelude::*,
    };

    fn assert_neighbors_void(cursor: &EdgeCursor<'_, MeshType3d64PNU>) {
        assert!(cursor.faces().next().is_none());
        assert!(cursor.chain().next().is_none());
        assert!(cursor.chain_back().next().is_none());

        assert!(cursor.fork().origin().is_void());
        assert!(cursor.fork().target().is_void());
        assert!(cursor.fork().face_ids().next().is_none());

        assert!(cursor.fork().next().is_void());
        assert!(cursor.fork().prev().is_void());
        for i in 0..10 {
            assert!(cursor.fork().next_n(i).is_void());
            assert!(cursor.fork().prev_n(i).is_void());
        }
        assert!(cursor.fork().twin().is_void());
        assert!(cursor.fork().next_sibling().is_void());
        assert!(cursor.fork().prev_sibling().is_void());
        assert!(cursor.fork().face().is_void());
        assert!(cursor.fork().check().is_err());
        assert!(cursor.fork().same_chain(VU::new(0)).is_none());
        assert!(cursor.fork().same_chain_back(VU::new(0)).is_none());
    }

    #[test]
    fn void_behavior() {
        // given a void cursors, all iterators should be empty and all methods should be void
        let mut mesh = Mesh3d64::new();
        let cursor = mesh.edge(EU::new(0));
        assert!(cursor.is_void());
        assert_neighbors_void(&cursor);

        mesh.insert_vertex(VertexPayloadPNU::from_pos(VecN::zeros()));
        let cursor1 = mesh.edge(EU::new(0));
        assert!(cursor1.is_void());
        assert_neighbors_void(&cursor1);
    }
}
