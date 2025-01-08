use super::{
    CursorData, EdgeCursor, FaceCursorBasics, FaceCursorData, FaceCursorHalfedgeBasics,
    VertexCursor,
};
use crate::{
    math::IndexType,
    mesh::{FaceBasics, HalfEdge, MeshBasics, MeshType},
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
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    /// Returns a reference to the payload of the face.
    /// Panics if the face is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &T::FP {
        self.unwrap().payload()
    }
}

impl<'a, T: MeshType> PartialEq for FaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face == other.face && self.mesh as *const _ == other.mesh as *const _
    }
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
    fn move_to(self, id: T::F) -> FaceCursor<'a, T> {
        Self::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> FaceCursorBasics<'a, T> for FaceCursor<'a, T> {}
impl<'a, T: MeshType + 'a> FaceCursorHalfedgeBasics<'a, T> for FaceCursor<'a, T> where
    T::Edge: HalfEdge<T>
{
}
