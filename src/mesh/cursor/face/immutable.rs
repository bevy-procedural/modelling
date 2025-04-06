use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct FaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> PartialEq for FaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face == other.face && self.mesh as *const _ == other.mesh as *const _
    }
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

impl_debug_cursor!(FaceCursor<'a, T: MeshType>, id: face);

#[rustfmt::skip]
impl_specific_cursor_data!(
    FaceCursorData, FaceCursor,
    EC, move_to_edge, T::E, EdgeCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
   MaybeCursor, FaceCursor, ValidFaceCursor, 
   face, load_new, F, Face, FP, 
   get_face, has_face
);

impl<'a, T: MeshType> ImmutableCursor for FaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.face)
    }
}

impl<'a, T: MeshType> MaybeCursor for FaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for FaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for FaceCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> ImmutableFaceCursor<'a, T> for FaceCursor<'a, T> where T: 'a {}
