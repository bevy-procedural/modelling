use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
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

impl_debug_eq_cursor!(ValidFaceCursorMut, face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, ValidFaceCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, MutableCursor, ValidFaceCursorMut, FaceCursorMut, 
    face, F, Face, FP, 
    get_face, get_face_mut, has_face,
    ValidFaceCursorBasics, FaceCursorBasics, FaceCursorHalfedgeBasics, MutableCursor
);

impl<'a, T: MeshType> ValidFaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType + 'a> FaceCursorBuilder<'a, T> for ValidFaceCursorMut<'a, T> {}
