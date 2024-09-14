//! Iterator utility

/*
pub trait IteratorExt: Iterator {
    /// Returns whether all elements in the iterator are unique.
    fn is_unique(self) -> bool
    where
        Self: Sized,
        Self::Item: Eq + std::hash::Hash,
    {
        let mut seen = HashSet::new();
        self.all(move |item| seen.insert(item))
    }


    /// Returns whether there is exactly one element in the iterator.
    fn exactly_one(self) -> bool
    where
        Self: Sized,
    {
        self.take(2).count() == 1
    }
}

impl<I: Iterator> IteratorExt for I {}
*/
