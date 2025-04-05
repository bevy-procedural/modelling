use crate::{mesh::MeshType, prelude::ValidVertexCursor, util::CreateEmptyIterator};

/// A wrapper over an iterator over vertex references that knows the parent mesh.
/// It allows mapping each vertex reference into a ValidVertexCursor.
pub struct Vertex2ValidVertexCursorAdapter<'a, T: 'a + MeshType, I: Iterator> {
    mesh: Option<&'a T::Mesh>,
    inner: I,
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Vertex>>
    Vertex2ValidVertexCursorAdapter<'a, T, I>
{
    /// Creates a new Vertex2ValidVertexCursorAdapter.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, inner: I) -> Self {
        Vertex2ValidVertexCursorAdapter {
            mesh: Some(mesh),
            inner,
        }
    }
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Vertex>> Iterator
    for Vertex2ValidVertexCursorAdapter<'a, T, I>
{
    type Item = ValidVertexCursor<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mesh = self.mesh?;
        self.inner
            .next()
            .map(|vertex| ValidVertexCursor::new(mesh, vertex))
    }
}

impl<'a, T: 'a + MeshType, I: Iterator + CreateEmptyIterator> CreateEmptyIterator
    for Vertex2ValidVertexCursorAdapter<'a, T, I>
{
    #[inline]
    fn create_empty() -> Self {
        Vertex2ValidVertexCursorAdapter {
            mesh: None,
            inner: I::create_empty(),
        }
    }
}
