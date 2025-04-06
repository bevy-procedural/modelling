macro_rules! impl_debug_cursor {
    // Case 1: The cursor stores a direct "id" field
    ($cursor:ident <$lt:lifetime, $ty:ident : MeshType>, id: $id:ident) => {
        impl<$lt, $ty: MeshType> std::fmt::Debug for $cursor<$lt, $ty> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($cursor), self.$id)
            }
        }
    };
    // Case 2: The cursor stores an "instance" that has an `.id()` method
    ($cursor:ident <$lt:lifetime, $ty:ident : MeshType>, instance: $inst:ident) => {
        impl<$lt, $ty: MeshType> std::fmt::Debug for $cursor<$lt, $ty> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($cursor), self.$inst.id())
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
        impl<'a, T: MeshType> $trait_name<'a, T> for $cursor<'a, T>
        where
            T: 'a,
        {
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
