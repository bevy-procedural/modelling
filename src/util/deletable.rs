//! A module for soft-deletable elements.

use super::CreateEmptyIterator;
use crate::math::IndexType;

/// A trait for soft-deletable elements.
pub trait Deletable<I> {
    /// Returns whether the element is deleted.
    #[must_use]
    fn is_deleted(&self) -> bool;

    /// Marks the element as deleted.
    fn delete(&mut self);

    /// Sets the id of the element (un-deletes it).
    fn set_id(&mut self, id: I);

    /// Allocates a new, "deleted" instance (it isn't valid)
    #[must_use]
    fn allocate() -> Self;
}

pub type DeletableVectorIter<'a, T> = std::iter::Filter<std::slice::Iter<'a, T>, fn(&&T) -> bool>;

impl<'a, T> CreateEmptyIterator for DeletableVectorIter<'a, T> {
    #[inline]
    fn create_empty() -> Self {
        (&[] as &[T]).iter().filter(|_| false)
    }
}

impl<'a, T, V: Default> CreateEmptyIterator
    for std::iter::Map<DeletableVectorIter<'a, T>, fn(&'a T) -> V>
{
    fn create_empty() -> Self {
        DeletableVectorIter::<'a, T>::create_empty().map(|_| V::default())
    }
}

impl<'a, T> CreateEmptyIterator for std::iter::Filter<DeletableVectorIter<'a, T>, fn(&&T) -> bool> {
    fn create_empty() -> Self {
        DeletableVectorIter::<'a, T>::create_empty().filter(|_| false)
    }
}

impl<'a, T, V: Default> CreateEmptyIterator
    for std::iter::Map<
        std::iter::Filter<DeletableVectorIter<'a, T>, fn(&&T) -> bool>,
        fn(&'a T) -> V,
    >
where
    T: 'a,
{
    #[inline]
    fn create_empty() -> Self {
        <std::iter::Filter<DeletableVectorIter<'a, T>, fn(&&T) -> bool> as CreateEmptyIterator>::create_empty().map(|_| V::default())
    }
}

/// A vector that also keeps track of deleted elements to reallocate them.
#[derive(Debug, Clone)]
pub struct DeletableVector<T: Deletable<I>, I: IndexType> {
    data: Vec<T>,
    deleted: Vec<I>,
}

impl<T: Deletable<I>, I: IndexType> DeletableVector<T, I> {
    /// Creates a new empty vector.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            deleted: Vec::new(),
        }
    }

    /// Deletes all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.deleted.clear();
    }

    /// Returns an iterator over the non-deleted elements.
    #[inline]
    pub fn iter(&self) -> DeletableVectorIter<T> {
        self.data.iter().filter(|f| !f.is_deleted())
    }

    /// Returns a mutable iterator over the non-deleted elements.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter(|f| !f.is_deleted())
    }

    /// Returns the requested element or `None` if it doesn't exist or is deleted.
    #[inline]
    #[must_use]
    pub fn get(&self, index: I) -> Option<&T> {
        // PERF: We could add `unlikely` to these two conditions, but the compiler does a good job already.
        let i = index.index();
        if i >= self.data.len() {
            return None;
        }
        let v = &self.data[i];
        if v.is_deleted() {
            return None;
        }
        Some(v)
    }

    /// Returns whether the element exists and is not deleted.
    #[inline]
    #[must_use]
    pub fn has(&self, index: I) -> bool {
        let i = index.index();
        i < self.data.len() && !self.data[i].is_deleted()
    }

    /// Returns the requested element mutably or `None` if it doesn't exist or is deleted.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        let i = index.index();
        if i >= self.data.len() {
            return None;
        }
        let v = &mut self.data[i];
        if v.is_deleted() {
            return None;
        }
        Some(v)
    }

    /// Returns the number of non-deleted elements.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len() - self.deleted.len()
    }

    /// Returns the maximum index of the non-deleted elements.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Allocates a new element, moves the given to that index, sets the new id, and returns the index.
    #[inline]
    pub fn push(&mut self, mut v: T) -> I {
        assert!(
            v.is_deleted(),
            "Tried to push an element that already has an id"
        );
        if let Some(index) = self.deleted.pop() {
            v.set_id(index);
            self.data[index.index()] = v;
            index
        } else {
            let index = I::new(self.data.len());
            v.set_id(index);
            self.data.push(v);
            index
        }
    }

    /// Move the element at the given index. Assumes that the position is allocated and free, i.e., the contents are deleted.
    #[inline]
    pub fn set(&mut self, index: I, mut v: T) {
        assert!(
            self.data[index.index()].is_deleted(),
            "Tried to overwrite a non-deleted element at {}",
            index
        );
        assert!(
            v.is_deleted(),
            "Tried to set an element that already has an id"
        );
        let l = self.deleted.len();
        debug_assert!(
            !((l >= 1 && self.deleted[l - 1] == index)
                || (l >= 2 && self.deleted[l - 2] == index)
                || (l >= 3 && self.deleted[l - 3] == index)),
            // self.deleted.contains(&index), // this would be too slow even in debug mode
            "Tried to set an element without allocating an index first"
        );
        v.set_id(index);
        self.data[index.index()] = v;
    }

    /// Marks the element as deleted and remembers it for reallocation.
    #[inline]
    pub fn delete(&mut self, f: I) {
        self.data[f.index()].delete();
        self.deleted.push(f);
    }

    /// Returns the next free index or allocates a new one.
    /// The element is not deleted anymore, but it is not valid until it is overwritten.
    /// TODO: How can we force the user to overwrite it afterwards? Not writing to it is a memory leak.
    #[inline]
    #[must_use]
    pub fn allocate(&mut self) -> I {
        if let Some(index) = self.deleted.pop() {
            index
        } else {
            let t = T::allocate();
            debug_assert!(t.is_deleted());
            self.data.push(t);
            I::new(self.data.len() - 1)
        }
    }
}
