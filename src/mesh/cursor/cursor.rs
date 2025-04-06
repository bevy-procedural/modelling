use super::CursorData;
use crate::mesh::MeshType;

/// An immutable cursor is a cursor that doesn't hold a mutable reference to the mesh.
/// This allows the cursor to be freely shared and passed around without worrying about mutable borrow rules.
pub trait ImmutableCursor: CursorData + Clone {
    /// Clones the cursor.
    /// 
    /// This can be used if you want to be explicit about the fact that ownership is forked to a new cursor.
    #[must_use]
    #[inline]
    fn fork(&self) -> Self {
        self.clone()
    }
}

/// A mutable cursor is a cursor that holds a mutable reference to the mesh.
/// This allows the cursor to modify the mesh it points to.
/// However, you cannot clone a mutable cursor.
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

/// A maybe cursor (often just called "cursor") is a cursor that may or may not point to an instance.
/// This is the most common type of cursor and is used in most of the mesh API.
/// A maybe cursor without a valid instance is called "void", otherwise it is called "valid".
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
    fn assert_valid(self) -> Self::Valid {
        assert!(self.is_valid(), "Expected {:?} to be valid", self);
        self.load().unwrap()
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
    fn expect(self, msg: &str) -> Self::Valid {
        if !self.is_valid() {
            panic!("{}", msg);
        }
        self.load().unwrap()
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

/// A valid cursor is gauranteed to point to an existing instance.
/// Hence, queries like `id()`, `next_id()`, or `inner()` always succeed.
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

/// This trait defines methods specific to cursors that are both mutable and valid.
pub trait ValidCursorMut: ValidCursor + MutableCursor {
    /// Returns a mutable reference to the instance's payload.
    #[must_use]
    fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload;

    /// Returns a mutable reference to the instance.
    #[must_use]
    fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S;
}
