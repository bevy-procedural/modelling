use super::{
    CursorData, EdgeCursorMut, FaceCursor, FaceCursorBasics, FaceCursorData,
    FaceCursorHalfedgeBasics, VertexCursorMut,
};
use crate::{
    math::IndexType,
    mesh::{FaceBasics, HalfEdge, MeshBasics, MeshBuilder, MeshType},
};
use std::fmt::Debug;

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
#[derive(Debug)]
pub struct FaceCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Creates a new mutable face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    /// Returns an immutable clone pointing to the same face.
    #[inline]
    #[must_use]
    pub fn immutable(&'a self) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, self.face)
    }

    /// Returns a mutable reference to the payload of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    pub fn payload_mut(&mut self) -> &mut T::FP {
        self.mesh.face_ref_mut(self.face).payload_mut()
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
        self.mesh().get_face(self.try_id())
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_face(self.try_id())
    }

    #[inline]
    fn try_id(&self) -> T::F {
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

impl<'a, T: MeshType + 'a> FaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType + 'a> FaceCursorBasics<'a, T> for FaceCursorMut<'a, T> {}
impl<'a, T: MeshType + 'a> FaceCursorHalfedgeBasics<'a, T> for FaceCursorMut<'a, T> where
    T::Edge: HalfEdge<T>
{
}

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
