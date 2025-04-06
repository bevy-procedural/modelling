use crate::{
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdge, MeshBasics, MeshType},
    util::CreateEmptyIterator,
};

/// These methods are specific to immutable edge cursors, i.e., they require cloning the edge cursor.
pub trait ImmutableEdgeCursor<'a, T: MeshType>:
    CursorData<T = T, I = T::E, S = T::Edge> + EdgeCursorBasics<'a, T>
where
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
    T: 'a,
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

    /// Returns face cursors for each edge on the same boundary as this edge.
    /// Starts with the current edge.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn boundary<'b>(&'b self) -> impl Iterator<Item = ValidEdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.boundary(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
        .map(move |e| ValidEdgeCursor::new(self.mesh(), e))
    }

    /// Returns face cursors for each edge on the same boundary as this edge.
    /// Starts with the current edge.
    /// Traverses the boundary backwards.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    fn boundary_back<'b>(&'b self) -> impl Iterator<Item = ValidEdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.boundary_back(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
        .map(move |e| ValidEdgeCursor::new(self.mesh(), e))
    }

    /// Returns a reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn payload<'b>(&'b self) -> &'b T::EP
    where
        'a: 'b,
    {
        self.mesh().edge_payload(self.try_id())
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
    /// See [Mesh::validate_edge] for more information.
    #[inline]
    #[must_use]
    fn check(&self) -> Result<(), String> {
        if let Some(inner) = self.try_inner() {
            inner.check(self.mesh())
        } else {
            Err(format!("Edge {} is invalid", self.try_id()))
        }
    }

    /// Returns an outgoing edge from `v` that is part of the same boundary as the edge.
    /// Traverses the boundary forwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_boundary] for more information.
    #[inline]
    #[must_use]
    fn same_boundary(self, v: T::V) -> Option<Self::Maybe> {
        let id = HalfEdge::same_boundary(self.try_inner()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }

    /// Returns an outgoing edge from `v` that is part of the same boundary as the edge.
    /// Traverses the boundary backwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_boundary_back] for more information.
    #[inline]
    #[must_use]
    fn same_boundary_back(self, v: T::V) -> Option<Self::Maybe> {
        let id = HalfEdge::same_boundary_back(self.try_inner()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }
}
