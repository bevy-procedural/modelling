use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, HasIslands, MeshBasics, MeshType},
    util::CreateEmptyIterator,
};

/// Methods specific to immutable face cursors, i.e., they require cloning the face cursor.
pub trait ImmutableFaceCursor<'a, T: MeshType + 'a>:
    CursorData<T = T, I = T::F, S = T::Face> + ImmutableCursor + FaceCursorBasics<'a, T>
where
    T::Mesh: MeshBasics<T>,
{
    // TODO: Can I move some of these to FaceCursorBasics?

    /// Returns an iterator of the face's vertices.
    /// Returns an empty iterator if the face is void.
    ///
    /// Ignores islands and holes.
    #[inline]
    #[must_use]
    fn vertices(&'a self) -> impl Iterator<Item = ValidVertexCursor<'a, T>> {
        self.vertex_ids()
            .map(move |v| ValidVertexCursor::load_new(self.mesh(), v))
    }

    /// Returns an iterator of the face's edges.
    /// Returns an empty iterator if the face is void.
    ///
    /// Ignores islands and holes.
    #[inline]
    #[must_use]
    fn edges(&'a self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>> {
        self.edge_ids()
            .map(move |e| ValidEdgeCursor::load_new(self.mesh(), e))
    }

    /// Returns an iterator with an edge cursor for each edge chain adjacent of the face.
    /// The first edge cursor is on the outer edge chain of the face.
    /// Returns an empty iterator if the face is void.
    #[inline]
    #[must_use]
    fn islands(&'a self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>>
    where
        T::Face: HasIslands<T>,
    {
        self.island_ids()
            .map(move |e| ValidEdgeCursor::load_new(self.mesh(), e))
    }
}

/// Methods that work with all kinds of face cursors, including mutable, immutable, valid and maybe ones.
pub trait FaceCursorBasics<'a, T: MeshType>: FaceCursorData<'a, T> {
    /// Returns an iterator of vertex ids of the face.
    /// Returns an empty iterator if the face is void.
    ///
    /// Ignores islands and holes.
    ///
    /// See [FaceBasics::vertex_ids] for more information.
    #[inline]
    #[must_use]
    fn vertex_ids<'b>(&'b self) -> impl Iterator<Item = T::V> + CreateEmptyIterator
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
    /// Returns an empty iterator if the face is void.
    ///
    /// Ignores islands and holes.
    ///
    /// See [FaceBasics::edge_ids] for more information.
    #[inline]
    #[must_use]
    fn edge_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + CreateEmptyIterator
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

    /// Returns an iterator with an edge id for each edge chain adjacent of the face.
    /// The first edge id is for the outer edge chain of the face.
    /// Returns an empty iterator if the face is void.
    ///
    /// See [HasIslands::islands] for more information.
    #[inline]
    #[must_use]
    fn island_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + CreateEmptyIterator
    where
        T::Face: HasIslands<T>,
        T: 'b,
    {
        if let Some(inner) = self.try_inner() {
            inner.islands(self.mesh())
        } else {
            CreateEmptyIterator::create_empty()
        }
    }
}

/// Methods specific to face cursors on halfedge meshes, both valid and maybe ones.
pub trait FaceCursorHalfedgeBasics<'a, T: MeshType>: FaceCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
}
