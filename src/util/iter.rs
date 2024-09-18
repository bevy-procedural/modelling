//! Iterator utility

pub trait IteratorExt: Iterator {
    /// Returns whether there is exactly one element in the iterator.
    fn exactly_one(self) -> bool
    where
        Self: Sized,
    {
        self.take(2).count() == 1
    }
}

impl<I: Iterator> IteratorExt for I {}
