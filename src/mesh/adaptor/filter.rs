use crate::{
    math::IndexType,
    mesh::MeshType,
    prelude::{ValidEdgeCursor, ValidEdgeCursorBasics},
    util::CreateEmptyIterator,
};

/// Iterator adaptor for filtering edges by their target vertex id.
pub struct FilterTargetIdIterator<'a, T: MeshType + 'a, I: Iterator<Item = ValidEdgeCursor<'a, T>>>
{
    iter: I,
    target: T::V,
}

impl<'a, T: MeshType + 'a, I: Iterator<Item = ValidEdgeCursor<'a, T>>>
    FilterTargetIdIterator<'a, T, I>
{
    /// Creates a new iterator that filters edges by their target vertex id.
    pub fn new(iter: I, value: T::V) -> Self {
        Self {
            iter,
            target: value,
        }
    }
}
impl<'a, T: MeshType + 'a, I: Iterator<Item = ValidEdgeCursor<'a, T>>> Iterator
    for FilterTargetIdIterator<'a, T, I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if item.target_id() == self.target {
                return Some(item);
            }
        }
        None
    }
}

impl<'a, T: MeshType + 'a, I: CreateEmptyIterator + Iterator<Item = ValidEdgeCursor<'a, T>>>
    CreateEmptyIterator for FilterTargetIdIterator<'a, T, I>
{
    fn create_empty() -> Self {
        Self::new(I::create_empty(), IndexType::max())
    }
}
