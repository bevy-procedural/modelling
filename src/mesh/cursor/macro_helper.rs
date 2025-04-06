macro_rules! impl_debug_eq_cursor {
    ($cursor:ident, $id:ident) => {
        impl<'a, T: MeshType> std::fmt::Debug for $cursor<'a, T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($cursor), self.$id.id())
            }
        }

        impl<'a, T: MeshType> PartialEq for $cursor<'a, T> {
            /// same id and pointing to the same mesh instance
            fn eq(&self, other: &Self) -> bool {
                self.$id.id() == other.$id.id() && std::ptr::eq(self.mesh, other.mesh)
            }
        }
    };
}

macro_rules! impl_specific_cursor_data {
    (
        $trait_name:ident,
        $cursor:ident,

        // Associated type name, method name, element type, target cursor
        $assoc1:ident, $method1:ident, $elem1:ty, $target1:ident,
        $assoc2:ident, $method2:ident, $elem2:ty, $target2:ident
    ) => {
        impl<'a, T: MeshType + 'a> $trait_name<'a, T> for $cursor<'a, T> {
            type $assoc1 = $target1<'a, T>;
            type $assoc2 = $target2<'a, T>;

            #[inline]
            fn $method1(self, id: $elem1) -> Self::$assoc1 {
                $target1::new(self.mesh, id)
            }

            #[inline]
            fn $method2(self, id: $elem2) -> Self::$assoc2 {
                $target2::new(self.mesh, id)
            }

            #[inline]
            fn destructure(self) -> (&'a T::Mesh, Self::I) {
                (self.mesh, self.try_id())
            }
        }
    };
}
