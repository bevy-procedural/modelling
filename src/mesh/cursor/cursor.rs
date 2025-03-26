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

    /// Returns the id the cursor is pointing to. Panics if the cursor is void.
    #[must_use]
    #[inline]
    fn id(&self) -> Self::I {
        // TODO: Returns `None` if the cursor is void or the id points to a deleted instance.
        assert!(self.is_valid(), "Expected {:?} to be valid", self);
        self.try_id()
    }

    /// Returns the id the cursor is pointing to no matter if it is void, deleted, or valid.
    #[must_use]
    fn try_id(&self) -> Self::I;

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
    fn inner<'b>(&'b self) -> Option<&'b Self::S>;

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
    #[must_use]
    fn try_move<F: FnOnce(&Self::S) -> Self::I>(self, f: F) -> Self {
        if let Some(e) = self.inner() {
            let id = f(e);
            self.move_to(id)
        } else {
            self.void()
        }
    }

    /// Applies a closure to the instance if it exists and is not deleted, returning the result or the default.
    /// The default is always evaluated.
    #[inline]
    fn map_or<U, F: FnOnce(&Self::S) -> U>(&self, default: U, f: F) -> U {
        self.inner().map(f).unwrap_or(default)
    }

    /// Applies a closure to the instance if it exists and is not deleted, returning the result or the default.
    /// The default is only evaluated if necessary.
    #[inline]
    fn map_or_else<U, F: FnOnce(&Self::S) -> U, E: FnOnce() -> U>(&self, default: E, f: F) -> U {
        self.inner().map(f).unwrap_or_else(default)
    }

    /// Returns a reference to the instance the cursor points to..
    /// Panics if it does'nt exist or is deleted.
    #[inline]
    fn unwrap<'b>(&'b self) -> &'b Self::S {
        self.inner().unwrap()
    }

    /// Applies the function in the closure to the cursor but return a cursor pointing to the same id as before calling the closure.
    #[inline]
    #[must_use]
    fn stay<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        let id = self.try_id();
        let c = f(self);
        c.move_to(id)
    }
}
