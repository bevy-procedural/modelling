use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::EuclideanMeshType,
};

/// Iterator ccw over a regular polygon with `n` sides and radius `r`.
pub fn circle_iter<const D: usize, T: EuclideanMeshType<D>>(
    n: usize,
    r: T::S,
    shift: T::S,
) -> impl Iterator<Item = T::VP> {
    let npi2: T::S = T::S::TWO / T::S::from_usize(n) * T::S::PI;
    (0..n).map(move |i| {
        T::VP::from_pos(T::Vec::from_xy(
            ((T::S::from_usize(i) + shift) * npi2).sin() * r,
            ((T::S::from_usize(i) + shift) * npi2).cos() * r,
        ))
    })
}

/// Iterator cw over a regular polygon with `n` sides and radius `r`.
pub fn circle_iter_back<const D: usize, T: EuclideanMeshType<D>>(
    n: usize,
    r: T::S,
    shift: T::S,
) -> impl Iterator<Item = T::VP> {
    let npi2: T::S = -T::S::TWO / T::S::from_usize(n) * T::S::PI;
    (0..n).map(move |i| {
        T::VP::from_pos(T::Vec::from_xy(
            ((T::S::from_usize(i) + shift) * npi2).sin() * r,
            ((T::S::from_usize(i) + shift) * npi2).cos() * r,
        ))
    })
}

/// Trait for iterators that can be created empty.
pub trait CreateEmptyIterator {
    /// Creates an empty iterator.
    #[must_use]
    fn create_empty() -> Self;
}

impl<T> CreateEmptyIterator for std::vec::IntoIter<T> {
    #[inline]
    fn create_empty() -> Self {
        Vec::new().into_iter()
    }
}

/*impl<'a, Input, Output, Iter: CreateEmptyIterator + Iterator<Item = Input>> CreateEmptyIterator
    for std::iter::Map<Iter, fn(Input) -> Output>
{
    fn create_empty() -> Self {
        Iter::create_empty().map(|_| unreachable!())
    }
}*/

/*impl<
        'a,
        F: FnMut(Input) -> Output + Default,
        Input,
        Output,
        Iter: CreateEmptyIterator + Iterator<Item = Input>,
    > CreateEmptyIterator for std::iter::Map<Iter, F>
{
    fn create_empty() -> Self {
        Iter::create_empty().map(F::default())
    }
}*/

impl<I, In, Out> CreateEmptyIterator for std::iter::Map<I, fn(In) -> Out>
where
    I: CreateEmptyIterator + Iterator<Item = In>,
{
    fn create_empty() -> Self {
        I::create_empty().map(|_| unreachable!())
    }
}

/*
impl<
        'a,
        F: FnMut(Input) -> Option<Output> + Default,
        Input,
        Output,
        Iter: CreateEmptyIterator + Iterator<Item = Input>,
    > CreateEmptyIterator for std::iter::FilterMap<Iter, F>
{
    fn create_empty() -> Self {
        Iter::create_empty().filter_map(F::default())
    }
}

impl<
        'a,
        F: FnMut(&Input) -> bool + Default,
        Input,
        Iter: CreateEmptyIterator + Iterator<Item = Input>,
    > CreateEmptyIterator for std::iter::Filter<Iter, F>
{
    fn create_empty() -> Self {
        Iter::create_empty().filter(F::default())
    }
}*/


impl<I, In, Out> CreateEmptyIterator
    for std::iter::FilterMap<I, fn(In) -> Option<Out>>
where
    I: CreateEmptyIterator + Iterator<Item = In>,
{
    fn create_empty() -> Self {
        I::create_empty().filter_map(|_| None)
    }
}

impl<I, In> CreateEmptyIterator for std::iter::Filter<I, fn(&In) -> bool>
where
    I: CreateEmptyIterator + Iterator<Item = In>,
{
    fn create_empty() -> Self {
        I::create_empty().filter(|_| false)
    }
}
