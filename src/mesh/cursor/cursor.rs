use crate::{math::IndexType, mesh::MeshType};
use std::fmt::Debug;

/// This trait defines the basic functionality for accessing the data fields of a cursor.
pub trait CursorData: Sized + Debug {
    /// The associated index type
    type I: IndexType;

    /// The associated index type
    type S: Sized;

    /// The associated mesh type
    type T: MeshType;

    /// Returns the id the cursor is pointing to.
    #[must_use]
    fn id(&self) -> Self::I;

    /// Whether the cursor points to an invalid id, i.e.,
    /// either having the maximum index or pointing to a deleted instance.
    #[must_use]
    fn is_void(&self) -> bool;

    /// Whether the cursor points to a valid id, i.e.,
    /// not having the maximum index and pointing to an existing instance.
    #[must_use]
    #[inline]
    fn is_valid(&self) -> bool {
        !self.is_void()
    }

    /// Returns a reference to the instance if it exists and is not deleted, otherwise `void`.
    #[must_use]
    fn get<'b>(&'b self) -> Option<&'b Self::S>;

    /// Returns a reference to the mesh the cursor points to.
    #[must_use]
    fn mesh<'b>(&'b self) -> &'b <Self::T as MeshType>::Mesh;

    /// Derives a new cursor pointing to the given id.
    #[must_use]
    fn move_to(self, id: Self::I) -> Self;

    /// Converts the cursor to a void-cursor
    #[inline]
    #[must_use]
    fn void(self) -> Self {
        self.move_to(IndexType::max())
    }

    /// Asserts some condition on the cursor.
    /// This is a wrapper around `assert!` to use the chain syntax.
    #[inline]
    fn assert<F: FnOnce(&Self) -> bool>(self, f: F) -> Self {
        assert!(f(&self));
        self
    }

    /// Asserts that the cursor points to a valid id.
    #[inline]
    fn assert_valid(self) -> Self {
        assert!(self.is_valid(), "Expected {:?} to be valid", self);
        self
    }

    /// Asserts that the cursor points to an invalid id.
    #[inline]
    fn assert_void(self) -> Self {
        assert!(self.is_void(), "Expected {:?} to be void", self);
        self
    }

    /// Debug-asserts some condition on the cursor.
    /// This is a wrapper around `debug_assert!` to use the chain syntax.
    #[inline]
    fn debug_assert<F: FnOnce(&Self) -> bool>(self, f: F) -> Self {
        debug_assert!(f(&self));
        self
    }

    /// Panics if the cursor points to an invalid id.
    /// Returns the same cursor otherwise.
    #[inline]
    fn expect(self, msg: &str) -> Self {
        if !self.is_valid() {
            panic!("{}", msg);
        }
        self
    }

    /// Panics if the cursor *doesn't* point to an invalid id.
    /// Returns the same cursor otherwise.
    #[inline]
    fn expect_void(self, msg: &str) -> Self {
        if !self.is_void() {
            panic!("{}", msg);
        }
        self
    }

    /// Applies a closure to the instance if it exists and is not deleted, moving the cursor to the returned id.
    #[inline]
    fn map<F: FnOnce(&Self::S) -> Self::I>(self, f: F) -> Self {
        if let Some(e) = self.get() {
            let id = f(e);
            self.move_to(id)
        } else {
            self.void()
        }
    }

    /// Returns a reference to the instance the cursor points to..
    /// Panics if it does'nt exist or is deleted.
    #[inline]
    fn unwrap<'b>(&'b self) -> &'b Self::S {
        self.get().unwrap()
    }
}
