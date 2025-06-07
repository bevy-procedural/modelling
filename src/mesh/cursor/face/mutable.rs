use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
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
        FaceCursor::new(self.mesh, self.id_unchecked())
    }
}

impl_debug_eq_cursor!(FaceCursorMut, face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, FaceCursorMut,
    EC, move_to_edge, T::E, EdgeCursorMut,
    VC, move_to_vertex, T::V, VertexCursorMut
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, MutableCursor, FaceCursorMut, ValidFaceCursorMut, 
   face, new, F, Face, FP, 
   get_face, has_face,
   FaceCursorBuilder, FaceCursorBasics, FaceCursorHalfedgeBasics
);

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        // TODO: don't panic
        self.mesh.face_ref_mut(self.id_unchecked()).set_edge(edge);
    }
}
