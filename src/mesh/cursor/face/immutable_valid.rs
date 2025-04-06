use crate::mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType};

/// A face cursor pointing to an existing non-deleted face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct ValidFaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: &'a T::Face,
}

impl<'a, T: MeshType> PartialEq for ValidFaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face.id() == other.face.id() && self.mesh as *const _ == other.mesh as *const _
    }
}

impl<'a, T: MeshType> ValidFaceCursor<'a, T> {
    /// Creates a new face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, face: &'a T::Face) -> Self {
        Self { mesh, face }
    }

    /// Creates a new face cursor pointing to the given face.
    /// Panics if the face does not exist in the mesh.
    #[inline]
    #[must_use]
    pub fn load_new(mesh: &'a T::Mesh, face: T::F) -> Self {
        Self::new(mesh, mesh.face_ref(face))
    }
}

impl_debug_cursor!(ValidFaceCursor<'a, T: MeshType>, id: face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, ValidFaceCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, ValidFaceCursor, FaceCursor, 
    face, F, Face, FP, 
    get_face, has_face
);

impl<'a, T: MeshType> ImmutableCursor for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.face)
    }
}

impl<'a, T: MeshType> ValidCursor for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.face.id()
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.face
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.face_ref(self.try_id()).payload()
    }
}

impl<'a, T: MeshType> ValidFaceCursorBasics<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for ValidFaceCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> ImmutableFaceCursor<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
