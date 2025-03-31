use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType},
    util::CreateEmptyIterator,
};

pub trait ImmutableFaceCursor<'a, T: MeshType>:
    CursorData<T = T, I = T::F, S = T::Face> + ImmutableCursor + FaceCursorBasics<'a, T>
where
    T: 'a,
    T::Mesh: MeshBasics<T>,
{
    /// Returns an iterator of the face's vertices.
    /// Panics if the face is void.
    /// See [FaceBasics::vertex_ids] for more information.
    #[inline]
    #[must_use]
    fn vertices(&'a self) -> impl Iterator<Item = ValidVertexCursor<'a, T>> {
        self.vertex_ids()
            .map(move |v| ValidVertexCursor::load_new(self.mesh(), v))
    }

    /// Returns an iterator of the face's edges.
    /// Panics if the face is void.
    /// See [FaceBasics::edge_ids] for more information.
    #[inline]
    #[must_use]
    fn edges(&'a self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>> {
        self.edge_ids()
            .map(move |e| ValidEdgeCursor::load_new(self.mesh(), e))
    }
}

/// This trait implements some basic functionality for face cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait FaceCursorBasics<'a, T: MeshType>: FaceCursorData<'a, T> {
    /// Returns an iterator of vertex ids of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn vertex_ids<'b>(&'b self) -> impl Iterator<Item = T::V> + 'b
    where
        T: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.vertex_ids(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
    }

    /// Returns an iterator of edge ids of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn edge_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b
    where
        T: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.edge_ids(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
    }

    /// Moves the cursor to the representative halfedge of the face.
    /// Returns the void cursor if the face is void or doesn't have a representative halfedge.
    #[inline]
    #[must_use]
    fn edge(self) -> Self::EC {
        // TODO: make it return a valid edge cursor if self is valid!
        let id = if let Some(inner) = self.try_inner() {
            inner.edge_id()
        } else {
            IndexType::max()
        };
        self.move_to_edge(id)
    }
}

/// This trait implements some basic functionality for face cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait FaceCursorHalfedgeBasics<'a, T: MeshType>: FaceCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
}
