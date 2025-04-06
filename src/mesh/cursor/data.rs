use super::{MaybeCursor, ValidCursor};
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
    fn stay<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        let id = self.try_id();
        let c = f(self);
        Self::from_maybe(c.move_to(id))
    }

    /// Transfers the ownership of the cursor to the closure and provides the id of the cursor at the start of the closure.
    /// no-op if the cursor is void.
    ///
    /// The closure moves the returned cursor.
    #[inline]
    #[must_use]
    fn with_id<F: FnOnce(Self::Valid, Self::I) -> Self>(self, f: F) -> Self {
        self.load_or_nop(|c| {
            let id = c.try_id();
            f(c, id)
        })
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
    /// If you don't need to use the result but only ensure validity, use [CursorData::ensure].
    #[inline]
    #[must_use]
    fn unwrap(self) -> Self::Valid {
        self.load().unwrap()
    }

    /// Ensures that the cursor points to a valid instance.
    /// Panics if the cursor is void. Does not return a value.
    #[inline]
    fn ensure(self) {
        assert!(self.is_valid(), "Expected {:?} to be valid", self);
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

macro_rules! impl_cursor_data {
    (MaybeCursor, $mutability:ident, $cursor:ident, $valid:ident,
        $try_id:ident, $load:ident, $I:ident, $S:ident, $payload:ident,
        $get_inner:ident, $check_has:ident,
        $mutability_impl:ident, $basics:ident, $halfedge_basics:ident) => {
        impl<'a, T: MeshType + 'a> CursorData for $cursor<'a, T> {
            type I = T::$I;
            type S = T::$S;
            type T = T;
            type Payload = T::$payload;
            type Maybe = Self;
            type Valid = $valid<'a, T>;

            #[inline]
            fn try_id(&self) -> Self::I {
                self.$try_id
            }

            #[inline]
            fn mesh<'b>(&'b self) -> &'b T::Mesh {
                self.mesh
            }

            #[inline]
            fn move_to(self, id: Self::I) -> Self::Maybe {
                Self::new(self.mesh, id)
            }

            #[inline]
            fn load(self) -> Option<Self::Valid> {
                if self.is_void() {
                    None
                } else {
                    Some(Self::Valid::$load(self.mesh, self.try_id()))
                }
            }

            #[inline]
            fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
                self.mesh.$get_inner(self.try_id())
            }

            #[inline]
            fn maybe(self) -> Self::Maybe {
                self
            }

            #[inline]
            fn from_maybe(from: Self::Maybe) -> Self {
                from
            }

            #[inline]
            fn from_valid(from: Self::Valid) -> Self {
                from.maybe()
            }

            #[inline]
            fn is_void(&self) -> bool {
                self.try_id() == IndexType::max() || !self.mesh().$check_has(self.try_id())
            }
        }

        impl<'a, T: MeshType + 'a> MaybeCursor for $cursor<'a, T> {}
        impl<'a, T: MeshType + 'a> $basics<'a, T> for $cursor<'a, T> {}
        impl<'a, T: MeshType + 'a> $halfedge_basics<'a, T> for $cursor<'a, T>
        where
            T::Edge: HalfEdge<T>,
            T::Vertex: HalfEdgeVertex<T>,
        {
        }

        impl_mutability!($mutability, $cursor, $mutability_impl);
    };

    (ValidCursor, $mutability:ident, $cursor:ident, $maybe:ident,
        $try_id:ident, $I:ident, $S:ident, $payload:ident,
        $get_inner:ident, $get_inner_mut:ident, $check_has:ident,
        $mutability_impl:ident, $valid:ident, $basics:ident, $halfedge_basics:ident) => {
        impl<'a, T: MeshType + 'a> CursorData for $cursor<'a, T> {
            type I = T::$I;
            type S = T::$S;
            type T = T;
            type Payload = T::$payload;
            type Maybe = $maybe<'a, T>;
            type Valid = Self;

            #[inline]
            fn try_id(&self) -> Self::I {
                self.$try_id.id()
            }

            #[inline]
            fn mesh<'b>(&'b self) -> &'b T::Mesh {
                self.mesh
            }

            #[inline]
            fn move_to(self, id: Self::I) -> Self::Maybe {
                Self::Maybe::new(self.mesh, id)
            }

            #[inline]
            fn load(self) -> Option<Self::Valid> {
                Some(self)
            }

            #[inline]
            fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
                // TODO: use the cashed inner value for valid immutable cursors
                self.mesh.$get_inner(self.try_id())
            }

            #[inline]
            fn maybe(self) -> Self::Maybe {
                Self::Maybe::new(self.mesh, self.try_id())
            }

            #[inline]
            fn from_maybe(from: Self::Maybe) -> Self {
                from.load().unwrap()
            }

            #[inline]
            fn from_valid(from: Self::Valid) -> Self {
                from
            }

            #[inline]
            fn is_void(&self) -> bool {
                false
            }
        }

        impl<'a, T: MeshType + 'a> ValidCursor for $cursor<'a, T> {
            #[inline]
            fn id(&self) -> Self::I {
                self.try_id()
            }

            #[inline]
            fn inner<'b>(&'b self) -> &'b Self::S {
                self.try_inner().unwrap()
            }

            impl_payload_method!($payload, $cursor);
        }

        impl<'a, T: MeshType + 'a> $valid<'a, T> for $cursor<'a, T> {}
        impl<'a, T: MeshType + 'a> $basics<'a, T> for $cursor<'a, T> {}
        impl<'a, T: MeshType + 'a> $halfedge_basics<'a, T> for $cursor<'a, T>
        where
            T::Edge: HalfEdge<T>,
            T::Vertex: HalfEdgeVertex<T>,
        {
        }

        impl_mutability!($mutability, $cursor, $mutability_impl);
        impl_valid_mut!($payload, $mutability, $cursor, $get_inner_mut);
    };
}

macro_rules! impl_payload_method {
    (EP, $cursor:ident) => {
        fn payload<'b>(&'b self) -> &'b Self::Payload {
            self.mesh.edge_payload(self.try_id())
        }
    };

    ($_:ident, $cursor:ident) => {
        #[inline]
        fn payload<'b>(&'b self) -> &'b Self::Payload {
            self.inner().payload()
        }
    };
}

macro_rules! impl_mutability {
    (ImmutableCursor, $cursor:ident, $immutable:ident) => {
        impl<'a, T: MeshType + 'a> ImmutableCursor for $cursor<'a, T> {}

        impl<'a, T: MeshType + 'a> $immutable<'a, T> for $cursor<'a, T> {}
    };

    (MutableCursor, $cursor:ident, $mutable:ident) => {
        impl<'a, T: MeshType + 'a> MutableCursor for $cursor<'a, T> {
            #[inline]
            fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
                self.mesh
            }
        }

        impl<'a, T: MeshType + 'a> $mutable<'a, T> for $cursor<'a, T> {}
    };
}

macro_rules! impl_valid_mut {
    (EP, MutableCursor, $cursor:ident, $get_inner_mut:ident) => {
        impl<'a, T: MeshType + 'a> ValidCursorMut for $cursor<'a, T> {
            #[inline]
            fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload {
                self.mesh.edge_payload_mut(self.try_id())
            }

            #[inline]
            fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S {
                self.mesh.$get_inner_mut(self.try_id()).unwrap()
            }
        }
    };

    ($_:ident, MutableCursor, $cursor:ident, $get_inner_mut:ident) => {
        impl<'a, T: MeshType + 'a> ValidCursorMut for $cursor<'a, T> {
            #[inline]
            fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload {
                self.inner_mut().payload_mut()
                // self.mesh.edge_payload_mut(self.edge)
            }

            #[inline]
            fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S {
                self.mesh.$get_inner_mut(self.try_id()).unwrap()
            }
        }
    };

    ($_:ident, ImmutableCursor, $cursor:ident, $get_inner_mut:ident) => {};
}
