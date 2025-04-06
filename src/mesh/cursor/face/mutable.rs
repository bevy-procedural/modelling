use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
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

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, self.try_id())
    }
}

impl_debug_cursor!(FaceCursorMut<'a, T: MeshType>, id: face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, FaceCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, FaceCursorMut, ValidFaceCursorMut, 
   face, new, F, Face, FP, 
   get_face, has_face
);

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType> MutableCursor for FaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> FaceCursorBuilder<'a, T> for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> MaybeCursor for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for FaceCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
