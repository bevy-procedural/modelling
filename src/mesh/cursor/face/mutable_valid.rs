use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
pub struct ValidFaceCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> ValidFaceCursorMut<'a, T> {
    /// Creates a new mutable face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, self.try_id())
    }
}

impl_debug_cursor!(ValidFaceCursorMut<'a, T: MeshType>, id: face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, ValidFaceCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, ValidFaceCursorMut, FaceCursorMut, 
    face, F, Face, FP, 
    get_face, has_face
);

impl<'a, T: MeshType> ValidCursor for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.face
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.mesh.get_face(self.face).unwrap()
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.face_ref(self.try_id()).payload()
    }
}

impl<'a, T: MeshType> ValidCursorMut for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload {
        self.mesh.face_ref_mut(self.try_id()).payload_mut()
    }

    #[inline]
    fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S {
        self.mesh.get_face_mut(self.face).unwrap()
    }
}

impl<'a, T: MeshType> MutableCursor for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> ValidFaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType> FaceCursorBuilder<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> ValidFaceCursorBasics<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for ValidFaceCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
