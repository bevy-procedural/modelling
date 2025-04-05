use crate::{
    mesh::{EdgeBasics, MeshType},
    util::CreateEmptyIterator,
};

/// A wrapper over an iterator over edge references that maps each edge to its target vertex reference.
pub struct EdgeRef2TargetRefAdapter<'a, T: 'a + MeshType, I> {
    mesh: Option<&'a T::Mesh>,
    inner: I,
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Edge>> EdgeRef2TargetRefAdapter<'a, T, I> {
    /// Creates a new EdgeRef2TargetRefAdapter.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, inner: I) -> Self {
        EdgeRef2TargetRefAdapter {
            mesh: Some(mesh),
            inner,
        }
    }
}

impl<'a, T: 'a + MeshType, I: Iterator<Item = &'a T::Edge>> Iterator
    for EdgeRef2TargetRefAdapter<'a, T, I>
{
    type Item = &'a T::Vertex;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mesh = self.mesh?;
        self.inner.next().map(|edge| edge.target(mesh))
    }
}

impl<'a, T: 'a + MeshType, I: Iterator + CreateEmptyIterator> CreateEmptyIterator
    for EdgeRef2TargetRefAdapter<'a, T, I>
{
    #[inline]
    fn create_empty() -> Self {
        EdgeRef2TargetRefAdapter {
            mesh: None,
            inner: I::create_empty(),
        }
    }
}
