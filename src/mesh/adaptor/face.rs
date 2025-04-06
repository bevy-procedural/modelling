use crate::{mesh::MeshType, prelude::ValidFaceCursor, util::CreateEmptyIterator};

/// A wrapper over an iterator over face references that knows the parent mesh.
/// It allows mapping each face reference into a ValidFaceCursor.
pub struct Face2ValidFaceCursorAdapter<'a, T: MeshType + 'a, I: Iterator> {
    mesh: Option<&'a T::Mesh>,
    inner: I,
}

impl<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::Face>> Face2ValidFaceCursorAdapter<'a, T, I> {
    /// Creates a new Face2ValidFaceCursorAdapter.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, inner: I) -> Self {
        Face2ValidFaceCursorAdapter {
            mesh: Some(mesh),
            inner,
        }
    }
}

impl<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::Face>> Iterator
    for Face2ValidFaceCursorAdapter<'a, T, I>
{
    type Item = ValidFaceCursor<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mesh = self.mesh?;
        self.inner
            .next()
            .map(|face| ValidFaceCursor::new(mesh, face))
    }
}

impl<'a, T: MeshType + 'a, I: Iterator + CreateEmptyIterator> CreateEmptyIterator
    for Face2ValidFaceCursorAdapter<'a, T, I>
{
    #[inline]
    fn create_empty() -> Self {
        Face2ValidFaceCursorAdapter {
            mesh: None,
            inner: I::create_empty(),
        }
    }
}
