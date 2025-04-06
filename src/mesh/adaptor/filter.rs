use crate::{
    math::IndexType,
    mesh::MeshType,
    prelude::{ValidEdgeCursor, ValidEdgeCursorBasics},
    util::CreateEmptyIterator,
};

pub struct FilterIdIterator<'a, T: MeshType, I: Iterator<Item = ValidEdgeCursor<'a, T>>>
where
    T: 'a,
{
    iter: I,
    target: T::V,
}

impl<'a, T: MeshType, I: Iterator<Item = ValidEdgeCursor<'a, T>>> FilterIdIterator<'a, T, I>
where
    T: 'a,
{
    pub fn new(iter: I, value: T::V) -> Self {
        Self {
            iter,
            target: value,
        }
    }
}
impl<'a, T: MeshType, I: Iterator<Item = ValidEdgeCursor<'a, T>>> Iterator
    for FilterIdIterator<'a, T, I>
where
    T: 'a,
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

impl<'a, T: MeshType, I: Iterator<Item = ValidEdgeCursor<'a, T>>> CreateEmptyIterator
    for FilterIdIterator<'a, T, I>
where
    I: CreateEmptyIterator,
    T: 'a,
{
    fn create_empty() -> Self {
        Self::new(I::create_empty(), IndexType::max())
    }
}
