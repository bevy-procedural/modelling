use crate::{
    mesh::{
        cursor::{ValidEdgeCursor, ValidFaceCursor, ValidVertexCursor},
        MeshType,
    },
    util::CreateEmptyIterator,
};

macro_rules! define_valid_cursor_adapter {
    ($AdapterName:ident, $MeshItem:ident, $ValidCursor:ident) => {
        #[doc = concat!(
                                    "A wrapper over an iterator of `T::", stringify!($MeshItem),
                                    "` references that knows the parent mesh.\n",
                                    "It allows mapping each `T::", stringify!($MeshItem),
                                    "` reference into a [`", stringify!($ValidCursor), "`]."
                                )]
        pub struct $AdapterName<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::$MeshItem>> {
            mesh: Option<&'a T::Mesh>,
            inner: I,
        }

        impl<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::$MeshItem>> $AdapterName<'a, T, I> {
            #[doc = concat!("Creates a new [`", stringify!($AdapterName), "`].")]
            #[inline]
            #[must_use]
            pub fn new(mesh: &'a T::Mesh, inner: I) -> Self {
                $AdapterName {
                    mesh: Some(mesh),
                    inner,
                }
            }
        }

        impl<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::$MeshItem>> Iterator
            for $AdapterName<'a, T, I>
        {
            type Item = $ValidCursor<'a, T>;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let mesh = self.mesh?;
                self.inner.next().map(|item| $ValidCursor::new(mesh, item))
            }
        }

        impl<'a, T: MeshType + 'a, I: Iterator<Item = &'a T::$MeshItem> + CreateEmptyIterator>
            CreateEmptyIterator for $AdapterName<'a, T, I>
        {
            #[inline]
            fn create_empty() -> Self {
                $AdapterName {
                    mesh: None,
                    inner: I::create_empty(),
                }
            }
        }
    };
}

define_valid_cursor_adapter!(Edge2ValidEdgeCursorAdapter, Edge, ValidEdgeCursor);
define_valid_cursor_adapter!(Face2ValidFaceCursorAdapter, Face, ValidFaceCursor);
define_valid_cursor_adapter!(Vertex2ValidVertexCursorAdapter, Vertex, ValidVertexCursor);
