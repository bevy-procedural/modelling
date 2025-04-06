use crate::mesh::{cursor::*, FaceBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType};

/// A face cursor pointing to an existing non-deleted face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct ValidFaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: &'a T::Face,
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

impl_debug_eq_cursor!(ValidFaceCursor, face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, ValidFaceCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
    ValidCursor, ImmutableCursor, ValidFaceCursor, FaceCursor, 
    face, F, Face, FP, 
    get_face, get_face_mut, has_face,
   ValidFaceCursorBasics, FaceCursorBasics, FaceCursorHalfedgeBasics, ImmutableFaceCursor
);
