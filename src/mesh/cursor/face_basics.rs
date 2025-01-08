use super::{CursorData, EdgeCursorData, VertexCursorData};
use crate::mesh::{FaceBasics, HalfEdge, MeshType};

/// This trait defines the basic functionality for accessing the data fields of a face cursor.
pub trait FaceCursorData<'a, T: MeshType + 'a>: CursorData<T = T, I = T::F, S = T::Face> {
    /// The associated vertex cursor type
    type VC: VertexCursorData<'a, T>;

    /// The associated edge cursor type
    type EC: EdgeCursorData<'a, T>;

    /// Derives a new vertex cursor pointing to the given vertex id.
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new edge cursor pointing to the given vertex id.
    fn move_to_edge(self, id: T::E) -> Self::EC;
}

/// This trait implements some basic functionality for face cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait FaceCursorBasics<'a, T: MeshType + 'a>: FaceCursorData<'a, T> {}

/// This trait implements some basic functionality for face cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait FaceCursorHalfedgeBasics<'a, T: MeshType + 'a>: FaceCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    /// Moves the cursor to the representative halfedge of the face.
    fn edge(self) -> Self::EC {
        let id = self.unwrap().edge_id();
        self.move_to_edge(id)
    }

    /// Returns the representative halfedge of the face.
    /// Panics if the face is void.
    fn edge_id(&self) -> T::E {
        self.unwrap().edge_id()
    }
}
