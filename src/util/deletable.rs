use crate::math::IndexType;

/// A trait for soft-deletable elements.
pub trait Deletable<I> {
    /// Returns whether the element is deleted.
    fn is_deleted(&self) -> bool;

    /// Marks the element as deleted.
    fn delete(&mut self);

    /// Sets the id of the element (un-deletes it).
    fn set_id(&mut self, id: I);
}

/// A vector that also keeps track of deleted elements to reallocate them.
#[derive(Debug, Clone)]
pub struct DeletableVector<T: Deletable<I> + Default, I: IndexType> {
    data: Vec<T>,
    deleted: Vec<I>,
}

impl<T: Deletable<I> + Default, I: IndexType> DeletableVector<T, I> {
    /// Creates a new empty vector.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            deleted: Vec::new(),
        }
    }

    /// Returns an iterator over the non-deleted elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter(|f| !f.is_deleted())
    }
    
    /// Returns a mutable iterator over the non-deleted elements.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter(|f| !f.is_deleted())
    }

    /// Returns the requested element. Panics if it doesn't exist or is deleted.
    pub fn get(&self, index: I) -> &T {
        let v = &self.data[index.index()];
        assert!(
            !v.is_deleted(),
            "Tried to access deleted element at {}",
            index
        );
        v
    }

    /// Returns the requested element mutably. Panics if it doesn't exist or is deleted.
    pub fn get_mut(&mut self, index: I) -> &mut T {
        let v = &mut self.data[index.index()];
        assert!(
            !v.is_deleted(),
            "Tried to mutably access deleted element at {}",
            index
        );
        v
    }

    /// Returns the number of non-deleted elements.
    pub fn len(&self) -> usize {
        self.data.len() - self.deleted.len()
    }

    /// Returns the maximum index of the non-deleted elements.
    pub fn max_ind(&self) -> usize {
        self.data.len()
    }

    /// Allocates a new element, moves the given to that index, sets the new id, and returns the index.
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
        v.set_id(index);
        self.data[index.index()] = v;
    }

    /// Returns the next free index or allocates a new one.
    pub fn allocate(&mut self) -> I {
        if let Some(index) = self.deleted.pop() {
            index
        } else {
            self.data.push(T::default());
            I::new(self.data.len() - 1)
        }
    }

    /// Marks the element as deleted and remembers it for reallocation.
    pub fn delete_internal(&mut self, f: I) {
        self.data[f.index()].delete();
        self.deleted.push(f);
    }
}
