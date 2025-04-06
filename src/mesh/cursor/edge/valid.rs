use crate::{
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, EuclideanMeshType, HalfEdge, MeshType},
};

/// Methods specific for edge cursors that are known to point to an existing edge.
pub trait ValidEdgeCursorBasics<'a, T: MeshType>:
    ValidCursor<T = T, I = T::E, S = T::Edge>
{
    /// Whether the edge (or its halfedgetwin) is boundary.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn is_boundary(&self) -> bool {
        self.inner().is_boundary(self.mesh())
    }

    /// Whether the edge is manifold.
    /// See [EdgeBasics::is_manifold] for more information.
    #[inline]
    #[must_use]
    fn is_manifold(&self) -> bool {
        self.inner().is_manifold(self.mesh())
    }

    /// Returns the centroid of the edge, i.e., the average of the origin and target vertices.
    #[inline]
    #[must_use]
    fn centroid<const D: usize>(&self) -> T::Vec
    where
        T: EuclideanMeshType<D>,
    {
        self.inner().centroid(self.mesh())
    }

    /// Returns the id of the origin vertex of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn origin_id(&self) -> T::V {
        // TODO: self.get().map(|e| e.origin_id(self.mesh()))
        self.inner().origin_id(self.mesh())
    }

    /// Returns the id of the target vertex of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn target_id(&self) -> T::V {
        self.inner().target_id(self.mesh())
    }
}

/// Methods specific for edge cursors on halfedge meshes that are known to point to an existing edge.
pub trait ValidEdgeCursorHalfedgeBasics<'a, T: MeshType>:
    EdgeCursorData<'a, T> + ValidCursor<T = T, I = T::E, S = T::Edge>
where
    T::Edge: HalfEdge<T>,
    Self::Valid: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC> + ValidEdgeCursorBasics<'a, T>,
    Self::Maybe: EdgeCursorData<'a, T, FC = Self::FC, VC = Self::VC>,
{
    /// Returns the id of the next halfedge of the edge.
    #[inline]
    #[must_use]
    fn next_id(&self) -> T::E {
        self.inner().next_id()
    }

    /// Returns the id of the previous halfedge of the edge.
    #[inline]
    #[must_use]
    fn prev_id(&self) -> T::E {
        self.inner().prev_id()
    }

    /// Returns the id of the twin halfedge of the edge.
    #[inline]
    #[must_use]
    fn twin_id(&self) -> T::E {
        self.inner().twin_id()
    }

    /// Returns the id of the face of the edge.
    #[inline]
    #[must_use]
    fn face_id(&self) -> T::F {
        self.inner().face_id()
    }

    /// Returns whether the edge has a face.
    #[inline]
    #[must_use]
    fn has_face(&self) -> bool {
        self.face_id() != IndexType::max()
    }

    /// Returns whether the edge is a boundary edge itself.
    /// See [HalfEdge::is_boundary_self] for more information.
    #[inline]
    #[must_use]
    fn is_boundary_self(&self) -> bool {
        self.inner().is_boundary_self()
    }
}
