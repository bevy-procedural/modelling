use crate::{mesh::MeshType, prelude::ValidEdgeCursor, util::CreateEmptyIterator};

/// A wrapper over an iterator over edge references that knows the parent mesh.
/// It allows mapping each edge reference into a ValidEdgeCursor.
pub struct Edge2ValidEdgeCursorAdapter<'a, T: 'a + MeshType, I: Iterator> {
    mesh: Option<&'a T::Mesh>,
    inner: I,
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Edge>> Edge2ValidEdgeCursorAdapter<'a, T, I> {
    /// Creates a new Edge2ValidEdgeCursorAdapter.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, inner: I) -> Self {
        Edge2ValidEdgeCursorAdapter {
            mesh: Some(mesh),
            inner,
        }
    }
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Edge>> Iterator
    for Edge2ValidEdgeCursorAdapter<'a, T, I>
{
    type Item = ValidEdgeCursor<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mesh = self.mesh?;
        self.inner
            .next()
            .map(|edge| ValidEdgeCursor::new(mesh, edge))
    }
}

impl<'a, T: 'a + MeshType, I: Iterator + CreateEmptyIterator> CreateEmptyIterator
    for Edge2ValidEdgeCursorAdapter<'a, T, I>
{
    #[inline]
    fn create_empty() -> Self {
        Edge2ValidEdgeCursorAdapter {
            mesh: None,
            inner: I::create_empty(),
        }
    }
}
