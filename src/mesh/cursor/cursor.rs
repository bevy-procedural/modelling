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

    /// Converts the cursor to a None-cursor
    #[inline]
    fn none(self) -> Self {
        self.move_to(IndexType::max())
    }

    /// Panics if the cursor points to an invalid id.
    /// Returns the same cursor otherwise.
    #[inline]
    fn expect(self, msg: &str) -> Self {
        if self.is_none() {
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
            self.none()
        }
    }

    /// Returns a reference to the instance the cursor points to..
    /// Panics if it does'nt exist or is deleted.
    #[inline]
    fn unwrap<'b>(&'b self) -> &'b Self::S {
        self.get().unwrap()
    }

    /// Whether the cursor points to an invalid id, i.e.,
    /// either having the maximum index or pointing to a deleted instance.
    fn is_none(&self) -> bool;

    /// Returns a reference to the instance if it exists and is not deleted, otherwise `None`.
    fn get<'b>(&'b self) -> Option<&'b Self::S>;

    /// Returns the id the cursor is pointing to.
    fn id(&self) -> Self::I;

    /// Returns a reference to the mesh the cursor points to.
    fn mesh<'b>(&'b self) -> &'b <Self::T as MeshType>::Mesh;

    /// Derives a new cursor pointing to the given id.
    fn move_to(self, id: Self::I) -> Self;
}
