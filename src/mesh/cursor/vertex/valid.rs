use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::{cursor::*, Face3d, FaceBasics, HalfEdge, HalfEdgeVertex, MeshType, VertexBasics},
};

pub trait ValidVertexCursorBasics<'a, T: MeshType>: VertexCursorData<'a, T> + ValidCursor {
    fn shortest_path(self, other: T::V) -> Option<(T::E, T::E, usize)>
    where
        T::Edge: HalfEdge<T>,
        Self::S: HalfEdgeVertex<T>,
    {
        self.inner().shortest_path(self.mesh(), other)
    }

    /// Returns the id of a representative edge incident to the vertex, `IndexType::max()` if it has none, or panic if the vertex is void.
    #[inline]
    #[must_use]
    fn edge_id(&self) -> T::E {
        self.inner().edge_id(self.mesh())
    }

    /// Whether the vertex is isolated.
    /// Panics if the vertex is void.
    /// See [VertexBasics::is_isolated] for more information.
    #[inline]
    #[must_use]
    fn is_isolated(&self) -> bool {
        self.inner().is_isolated(self.mesh())
    }

    /// Returns the vertex position.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn pos<S: Scalar, const D: usize, Vec: Vector<S, D>>(&self) -> Vec
    where
        T::VP: HasPosition<D, Vec, S = S>,
    {
        self.inner().pos()
    }

    /// Returns the vertex degree.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn degree(&self) -> usize {
        self.inner().degree(self.mesh())
    }

    /// Whether the vertex is manifold.
    /// See [VertexBasics::is_manifold] for more information.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    fn is_manifold(&self) -> bool {
        self.inner().is_manifold(self.mesh())
    }
}
