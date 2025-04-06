use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshBasics, MeshType},
};

/// An edge cursor pointing to an edge of a mesh with an immutable reference to the mesh.
/// It can be `void`, i.e., point to an invalid or deleted edge.
/// You can move the cursor even if it is void -- it will simply stay void without panicking.
///
/// To access the data of the edge, you have to call `load` on the cursor to get a `ValidEdgeCursor`.
///
/// You should prefer using Cursors over direct access to the mesh data structures whenever possible.
/// You don't have to worry about performance, as the rust compiler will completely optimize them away.
/// Cloning immutable cursors is also optimized away, so feel free to clone them as much as you like.
/// For example, when compiling `cursor.next().next().next().next()`, all function
/// calls will be inlined leading to the same 8 commands for each call to `next`:
/// ```ir
/// getelementptr + load    ; compute address of and load the `id` in the `HalfEdgeImpl` in the `Vec`
/// icmp + br               ; if the `id` is `IndexType::max()`, skip all further blocks (since it is deleted)
/// getelementptr + load    ; compute address of and load the `next_id` in the `HalfEdgeImpl`
/// icmp + br               ; if the `next_id` exceeds the length of the `Vec` or is `IndexType::max()`, skip all further blocks
/// ```
/// (using `cargo rustc -- --emit=llvm-ir -O -C debuginfo=2`)
#[derive(Clone, Eq)]
pub struct EdgeCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> EdgeCursor<'a, T> {
    /// Creates a new edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Creates a new edge cursor pointing nowhere (void).
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self {
            mesh,
            edge: IndexType::max(),
        }
    }

    // TODO: this cannot be called. How to realize this?
    /*#[inline]
    pub fn mutable(self, mesh: &'a mut T::Mesh) -> EdgeCursorMut<'a, T> {
        assert!(self.mesh as *const _ == mesh as *const _);
        EdgeCursorMut::new(mesh, self.edge)
    }*/
}

impl_debug_eq_cursor!(EdgeCursor, edge);

#[rustfmt::skip]
impl_specific_cursor_data!(
    EdgeCursorData, EdgeCursor,
    FC, move_to_face, T::F,FaceCursor,
    VC, move_to_vertex, T::V, VertexCursor
);

#[rustfmt::skip]
impl_cursor_data!(
    MaybeCursor, ImmutableCursor, EdgeCursor, ValidEdgeCursor, 
    edge, load_new, E, Edge, EP, 
    get_edge, has_edge,
    ImmutableEdgeCursor, EdgeCursorBasics, EdgeCursorHalfedgeBasics
);

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_edge_cursor() {
        let mut mesh = Mesh3d64::cube(1.0);
        let e0 = mesh.halfedge_ids().next().unwrap();
        let c1: EdgeCursor<'_, MeshType3d64PNU> = mesh.edge(e0).next();
        let c2 = c1.fork().next();
        let c3 = c1.fork().next().prev().next();
        assert_ne!(c1, c2);
        assert_eq!(c1, c1);
        assert_eq!(c2, c3);

        let _c1: EdgeCursorMut<'_, MeshType3d64PNU> = mesh.edge_mut(e0).next();
        /*c1.next()
        .subdivide(std::iter::empty())
        .next()
        .subdivide(std::iter::empty());*/
    }
}
