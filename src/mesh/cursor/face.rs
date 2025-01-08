use super::{
    CursorData, EdgeCursor, EdgeCursorData, EdgeCursorMut, VertexCursor, VertexCursorData,
    VertexCursorMut,
};
use crate::{
    math::IndexType,
    mesh::{FaceBasics, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge},
};
use std::fmt::Debug;

/// A face cursor pointing to a face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Debug, Eq)]
pub struct FaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> FaceCursor<'a, T> {
    /// Creates a new face cursor pointing to the given face.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }
}

impl<'a, T: MeshType> PartialEq for FaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face == other.face && self.mesh as *const _ == other.mesh as *const _
    }
}

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
#[derive(Debug)]
pub struct FaceCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Creates a new mutable face cursor pointing to the given face.
    #[inline]
    pub fn new(mesh: &'a mut T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    /// Returns an immutable clone pointing to the same face.
    #[inline]
    pub fn immutable(&'a self) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, self.face)
    }
}

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

impl<'a, T: MeshType + 'a> FaceCursorData<'a, T> for FaceCursor<'a, T> {
    type VC = VertexCursor<'a, T>;
    type EC = EdgeCursor<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for FaceCursor<'a, T> {
    type I = T::F;
    type S = T::Face;
    type T = T;

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Face> {
        self.mesh().get_face(self.id())
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_face(self.id())
    }

    #[inline]
    fn id(&self) -> T::F {
        self.face
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::F) -> FaceCursor<'a, T> {
        Self::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> FaceCursorData<'a, T> for FaceCursorMut<'a, T> {
    type VC = VertexCursorMut<'a, T>;
    type EC = EdgeCursorMut<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for FaceCursorMut<'a, T> {
    type I = T::F;
    type S = T::Face;
    type T = T;

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Face> {
        self.mesh().get_face(self.id())
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_face(self.id())
    }

    #[inline]
    fn id(&self) -> T::F {
        self.face
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::F) -> FaceCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }
}

/// This trait implements some basic functionality for face cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait FaceCursorBasics<'a, T: MeshType + 'a>: FaceCursorData<'a, T> {}

/// This trait implements some basic functionality for face cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait FaceCursorHalfedgeBasics<'a, T: MeshTypeHalfEdge + 'a>: FaceCursorData<'a, T> {
    /// Moves the cursor to the representative halfedge of the face.
    fn edge(self) -> Self::EC {
        let id = self.unwrap().edge_id();
        self.move_to_edge(id)
    }
}

impl<'a, T: MeshType + 'a> FaceCursorBasics<'a, T> for FaceCursor<'a, T> {}
impl<'a, T: MeshType + 'a> FaceCursorBasics<'a, T> for FaceCursorMut<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> FaceCursorHalfedgeBasics<'a, T> for FaceCursor<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> FaceCursorHalfedgeBasics<'a, T> for FaceCursorMut<'a, T> {}

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the face etc. using a chaining syntax.
impl<'a, T: MeshType + 'a> FaceCursorMut<'a, T> {
    /// Removes the face the cursor is pointing to.
    /// Returns an empty cursor if the face was removed successfully or didn't exist.
    /// Returns the same cursor if the face couldn't be removed and still exists.
    #[inline]
    pub fn remove(self) -> Self {
        if self.mesh.try_remove_face(self.face) {
            self.void()
        } else if self.mesh.has_face(self.face) {
            self
        } else {
            self.void()
        }
    }
}
