//! Iterator utility

/// Returns whether there is exactly one element in the iterator that satisfies the predicate
pub fn contains_exactly_one<I, P>(iter: I, predicate: P) -> bool
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    let mut filtered = iter.filter(predicate);
    filtered.by_ref().take(2).count() == 1
}
