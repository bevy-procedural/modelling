use crate::{math::IndexType, mesh::MeshType};
use std::fmt::Debug;

/// This trait defines the basic functionality for accessing the data fields of a cursor.
pub trait CursorData: Sized + Debug {
    /// The associated index type
    type I: IndexType;

    /// The associated value type
    type S: Sized;

    /// The associated mesh type
    type T: MeshType;

    /// The associated payload type
    type Payload: Sized;

    /// The associated maybe cursor type
    type Maybe: MaybeCursor<
        I = Self::I,
        S = Self::S,
        T = Self::T,
        Maybe = Self::Maybe,
        Valid = Self::Valid,
        Payload = Self::Payload,
    >;

    /// The associated valid cursor type
    type Valid: ValidCursor<
        I = Self::I,
        S = Self::S,
        T = Self::T,
        Maybe = Self::Maybe,
        Valid = Self::Valid,
        Payload = Self::Payload,
    >;

    /*
    /// Returns the id the cursor is pointing to. Panics if the cursor is void.
    #[must_use]
    #[inline]
    fn id(&self) -> Self::I {
        // TODO: Returns `None` if the cursor is void or the id points to a deleted instance.
        assert!(self.is_valid(), "Expected {:?} to be valid", self);
        self.try_id()
    }

    */

    /// Returns the id the cursor is pointing to no matter if it is
    /// - void (i.e., equals `IndexType::max()`, indicating the cursor was moved to a nonsensical position),
    /// - deleted (i.e., was valid in the past but has been deleted from the mesh by now), or
    /// - valid (i.e., points to an existing non-deleted instance).
    #[must_use]
    fn try_id(&self) -> Self::I;

    /// Returns a reference to the mesh the cursor points to.
    #[must_use]
    fn mesh<'b>(&'b self) -> &'b <Self::T as MeshType>::Mesh;

    /// Derives a new cursor pointing to the given id.
    #[must_use]
    fn move_to(self, id: Self::I) -> Self::Maybe;

    /// Converts the cursor to a void-cursor
    #[inline]
    #[must_use]
    fn void(self) -> Self::Maybe {
        self.move_to(IndexType::max())
    }

    /// Asserts some condition on the cursor.
    /// This is a wrapper around `assert!` to use the chain syntax.
    #[inline]
    fn assert<F: FnOnce(&Self) -> bool>(self, f: F) -> Self {
        assert!(f(&self));
        self
    }

    /// Debug-asserts some condition on the cursor.
    /// This is a wrapper around `debug_assert!` to use the chain syntax.
    #[inline]
    fn debug_assert<F: FnOnce(&Self) -> bool>(self, f: F) -> Self {
        debug_assert!(f(&self));
        self
    }

    /// Applies the function in the closure to the cursor but return a cursor pointing to the same id as before calling the closure.
    #[inline]
    #[must_use]
    fn stay<F: FnOnce(Self) -> Self>(self, f: F) -> Self::Maybe {
        let id = self.try_id();
        let c = f(self);
        c.move_to(id)
    }

    /// If the cursor is valid, it will be loaded and returned.
    /// If the cursor is void, `None` will be returned.
    #[must_use]
    fn load(self) -> Option<Self::Valid>;

    /// Load the cursor and apply the function in the closure to it or return the default value if it is void.
    ///
    /// WARNING: The default value is always evaluated.
    #[inline]
    #[must_use]
    fn load_or<Res, F: FnOnce(Self::Valid) -> Res>(self, default: Res, f: F) -> Res {
        if let Some(c) = self.load() {
            f(c)
        } else {
            default
        }
    }

    /// Load the cursor and apply the function in the closure to it or return the default value if it is void.
    /// The default is evaluated only if the cursor is void.
    #[inline]
    #[must_use]
    fn load_or_else<Res, Default: FnOnce(Self) -> Res, F: FnOnce(Self::Valid) -> Res>(
        self,
        default: Default,
        f: F,
    ) -> Res {
        if self.is_valid() {
            f(self.unwrap())
        } else {
            default(self)
        }
    }

    /// Load the cursor and apply the function in the closure to it or return `void` if it is void already.
    #[inline]
    #[must_use]
    fn load_or_void<F: FnOnce(Self::Valid) -> Self::Maybe>(self, f: F) -> Self::Maybe {
        if self.is_valid() {
            f(self.unwrap())
        } else {
            self.void()
        }
    }

    /// Load the cursor, apply the function in the closure, and move to the returned id,
    /// or return `void` if it is void already.
    ///
    /// Unlike `load_or_void`, this function will not transfer ownership of the cursor to the closure.
    #[inline]
    #[must_use]
    fn load_move_or_void<F: FnOnce(&mut Self::Valid, Self::I) -> Option<Self::I>>(
        self,
        f: F,
    ) -> Self::Maybe {
        if self.is_valid() {
            let mut valid = self.unwrap();
            let id = valid.id();
            if let Some(id) = f(&mut valid, id) {
                valid.move_to(id)
            } else {
                valid.void()
            }
        } else {
            self.void()
        }
    }

    /// Load the cursor and apply the function in the closure to it or return the cursor as-is if it is void.
    #[inline]
    #[must_use]
    fn load_or_nop<F: FnOnce(Self::Valid) -> Self>(self, f: F) -> Self {
        if self.is_valid() {
            f(self.unwrap())
        } else {
            self
        }
    }

    /// Loads the cursor and panics if it is void.
    /// For valid cursors, this is a no-op.
    #[inline]
    #[must_use]
    fn unwrap(self) -> Self::Valid {
        self.load().unwrap()
    }

    /// Returns a reference to the instance if it exists and is not deleted, otherwise `void`.
    #[must_use]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S>;

    /// Applies a closure to the instance if it exists and is not deleted, moving the cursor to the returned id.
    #[inline]
    #[must_use]
    fn try_move<F: FnOnce(&Self::S) -> Self::I>(self, f: F) -> Self::Maybe {
        if let Some(e) = self.try_inner() {
            let id = f(e);
            self.move_to(id)
        } else {
            self.void()
        }
    }

    /// Converts the cursor to a maybe cursor.
    /// For valid cursors, the cursor will be unloaded.
    /// For maybe cursors, this is a no-op.
    #[must_use]
    fn maybe(self) -> Self::Maybe;

    /// Converts a maybe cursor to the current cursor type.
    /// For valid cursors, this will load the given cursor and panic if it is void.
    /// For maybe cursors, this is a no-op.
    #[must_use]
    fn from_maybe(from: Self::Maybe) -> Self;

    /// Converts a valid cursor to the current cursor type.
    /// For valid cursors, this is a no-op.
    /// For maybe cursors, this will unload the given cursor.
    #[must_use]
    fn from_valid(from: Self::Valid) -> Self;

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
}

pub trait ImmutableCursor: CursorData {
    /// Clones the cursor.
    #[must_use]
    fn fork(&self) -> Self;
}

pub trait MutableCursor: CursorData {
    /// Returns a mutable reference to the mesh the cursor points to.
    ///
    /// You might want to consider using instead
    /// [CursorData::load],
    /// [CursorData::load_or],
    /// [CursorData::load_or_else],
    /// [CursorData::load_or_void],
    /// [CursorData::load_or_nop],
    /// etc. since these methods provide more transparent ownership semantics.
    #[must_use]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh;
}

pub trait MaybeCursor: CursorData {
    /*/// Returns a reference to the instance if it exists and is not deleted, otherwise `void`.
    #[must_use]
    fn inner<'b>(&'b self) -> Option<&'b Self::S>;

    /// Returns a reference to the instance the cursor points to..
    /// Panics if it does'nt exist or is deleted.
    #[inline]
    fn unwrap<'b>(&'b self) -> &'b Self::S {
        self.try_inner().unwrap()
    }*/

    // TODO: Move more functions down here

    /// Returns the id the cursor is pointing to.
    #[must_use]
    #[inline]
    fn id(&self) -> Option<Self::I> {
        if self.is_void() {
            None
        } else {
            Some(self.try_id())
        }
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

    /// Applies a closure to the instance if it exists and is not deleted, returning the result or the default.
    /// The default is always evaluated.
    #[inline]
    fn map_or<U, F: FnOnce(&Self::S) -> U>(&self, default: U, f: F) -> U {
        // TODO: Maybe not S but ValidCursor?
        self.try_inner().map(f).unwrap_or(default)
    }

    /// Applies a closure to the instance if it exists and is not deleted, returning the result or the default.
    /// The default is only evaluated if necessary.
    #[inline]
    fn map_or_else<U, F: FnOnce(&Self::S) -> U, E: FnOnce() -> U>(&self, default: E, f: F) -> U {
        self.try_inner().map(f).unwrap_or_else(default)
    }
}

pub trait ValidCursor: CursorData {
    /// Returns the id the cursor is pointing to.
    #[must_use]
    #[inline]
    fn id(&self) -> Self::I {
        self.try_id()
    }

    /// Returns a reference to the instance if it exists and is not deleted, otherwise `void`.
    #[must_use]
    fn inner<'b>(&'b self) -> &'b Self::S;

    /// Returns a reference to the payload of the face.
    /// Panics if the face is void.
    #[must_use]
    fn payload<'b>(&'b self) -> &'b Self::Payload;
}

pub trait ValidCursorMut: ValidCursor + MutableCursor {
    #[must_use]
    fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload;

    #[must_use]
    fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S;
}
