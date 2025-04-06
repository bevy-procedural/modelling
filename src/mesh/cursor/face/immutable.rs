use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
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

    /// Creates a new face cursor pointing nowhere (void).
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self {
            mesh,
            face: IndexType::max(),
        }
    }
}

impl_debug_eq_cursor!(FaceCursor, face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, FaceCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, ImmutableCursor, FaceCursor, ValidFaceCursor, 
   face, load_new, F, Face, FP, 
   get_face, has_face,
   ImmutableFaceCursor, FaceCursorBasics, FaceCursorHalfedgeBasics
);
