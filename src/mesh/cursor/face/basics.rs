use crate::{
    math::IndexType,
    mesh::{
        cursor::*, EdgeBasics, EuclideanMeshType, Face, Face3d, FaceBasics, HalfEdge, MeshBasics,
        MeshType, MeshType3D,
    },
};

/// This trait defines the basic functionality for accessing the data fields of a face cursor.
pub trait FaceCursorData<'a, T: MeshType>: CursorData<T = T, I = T::F, S = T::Face> {
    /// The associated vertex cursor type
    type VC: VertexCursorData<'a, T>;

    /// The associated edge cursor type
    type EC: EdgeCursorData<'a, T>;

    /// Derives a new vertex cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new edge cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_edge(self, id: T::E) -> Self::EC;
}

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
        self.unwrap().vertex_ids(self.mesh())
    }

    /// Returns an iterator of edge ids of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn edge_ids<'b>(&'b self) -> impl Iterator<Item = T::E> + 'b
    where
        T: 'b,
    {
        self.unwrap().edge_ids(self.mesh())
    }

    /// Moves the cursor to the representative halfedge of the face.
    /// Returns the void cursor if the face is void or doesn't have a representative halfedge.
    #[inline]
    #[must_use]
    fn edge(self) -> Self::EC {
        if self.is_void() {
            return self.move_to_edge(IndexType::max());
        }
        let id = self.unwrap().edge_id();
        self.move_to_edge(id)
    }

    /// Returns the representative halfedge of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn edge_id(&self) -> T::E {
        self.unwrap().edge_id()
    }
}

/// This trait implements some basic functionality for face cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait FaceCursorHalfedgeBasics<'a, T: MeshType>: FaceCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
}

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

    /// Returns a reference to the payload of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn payload(&self) -> &T::FP {
        self.inner().payload()
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
