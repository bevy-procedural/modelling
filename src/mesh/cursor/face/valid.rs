use crate::mesh::{
    cursor::*, EdgeBasics, EuclideanMeshType, Face, FaceBasics, MeshType, MeshType3D,
};

pub trait ValidFaceCursorBasics<'a, T: MeshType>:
    ValidCursor<S = T::Face, I = T::F, T = T>
where
    Self::S: FaceBasics<T>,
    T: 'a,
    T::Face: 'a,
{
    /// Returns the number of vertices of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn num_vertices(&self) -> usize {
        self.inner().num_vertices(self.mesh())
    }

    /// Returns the number of edges of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn num_edges(&self) -> usize {
        self.inner().num_edges(self.mesh())
    }

    #[inline]
    #[must_use]
    fn centroid<const D: usize>(&self) -> T::Vec
    where
        T: EuclideanMeshType<D>,
        Self::S: Face,
    {
        self.inner().centroid(self.mesh())
    }

    /// Returns the polygon
    #[inline]
    #[must_use]
    fn as_polygon(&self) -> T::Poly
    where
        T: MeshType3D,
    {
        self.inner().as_polygon(self.mesh())
    }

    #[inline]
    #[must_use]
    fn is_convex(&self) -> bool
    where
        T: MeshType3D,
    {
        self.inner().is_convex(self.mesh())
    }

    #[inline]
    #[must_use]
    fn is_planar2(&self) -> bool
    where
        T: MeshType3D,
    {
        self.inner().is_planar2(self.mesh())
    }

    #[inline]
    #[must_use]
    fn normal(&self) -> T::Vec
    where
        T: MeshType3D,
    {
        self.inner().normal(self.mesh())
    }
}
