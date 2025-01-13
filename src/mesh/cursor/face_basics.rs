use super::{CursorData, EdgeCursorData, VertexCursorData};
use crate::mesh::{Face3d, FaceBasics, HalfEdge, MeshType, MeshType3D};

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

/// This trait implements some basic functionality for face cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait FaceCursorBasics<'a, T: MeshType>: FaceCursorData<'a, T> {
    /// Returns the number of vertices of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn num_vertices(&self) -> usize {
        self.unwrap().num_vertices(self.mesh())
    }

    /// Returns the number of edges of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    fn num_edges(&self) -> usize {
        self.unwrap().num_edges(self.mesh())
    }

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

    /// Returns the polygon
    #[inline]
    #[must_use]
    fn as_polygon(&self) -> T::Poly
    where
        T: MeshType3D,
    {
        self.unwrap().as_polygon(self.mesh())
    }

    /// Moves the cursor to the representative halfedge of the face.
    #[inline]
    #[must_use]
    fn edge(self) -> Self::EC {
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
pub trait FaceCursorHalfedgeBasics<'a, T: MeshType >: FaceCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
}
