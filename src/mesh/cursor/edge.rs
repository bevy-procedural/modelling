use super::{
    CursorData, EdgeCursorBasics, EdgeCursorData, EdgeCursorHalfedgeBasics, FaceCursor,
    VertexCursor,
};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics, MeshType},
};

/// An edge cursor pointing to an edge of a mesh with an immutable reference to the mesh.
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

impl<'a, T: MeshType> std::fmt::Debug for EdgeCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EdgeCursor({:?})", self.edge)
    }
}

impl<'a, T: MeshType> EdgeCursor<'a, T> {
    /// Creates a new edge cursor pointing to the given edge.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Clones the cursor.
    #[inline]
    #[must_use]
    pub fn fork(&self) -> Self {
        Self::new(self.mesh, self.edge)
    }

    /// Creates a new void edge cursor.
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self::new(mesh, IndexType::max())
    }

    // TODO: this cannot be called. How to realize this?
    /*#[inline]
    pub fn mutable(self, mesh: &'a mut T::Mesh) -> EdgeCursorMut<'a, T> {
        assert!(self.mesh as *const _ == mesh as *const _);
        EdgeCursorMut::new(mesh, self.edge)
    }*/

    /// Returns a reference to the payload of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &'a T::EP {
        self.mesh.edge_payload(self.edge)
    }

    /// Returns face cursors for all faces adjacent to the edge
    /// (including the twin for halfedges and parallel edges' faces if the edge is non-manifold).
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    pub fn faces<'b>(&'b self) -> impl Iterator<Item = FaceCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        self.unwrap()
            .face_ids(self.mesh())
            .map(move |id| FaceCursor::new(self.mesh, id))
    }

    /// Returns face cursors for each edge on the same boundary as this edge.
    /// Starts with the current edge.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    pub fn boundary<'b>(&'b self) -> impl Iterator<Item = EdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        self.unwrap()
            .boundary(self.mesh())
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }

    /// Returns face cursors for each edge on the same boundary as this edge.
    /// Starts with the current edge.
    /// Traverses the boundary backwards.
    /// Returns an empty iterator if the edge is void.
    #[inline]
    #[must_use]
    pub fn boundary_back<'b>(&'b self) -> impl Iterator<Item = EdgeCursor<'b, T>>
    where
        T::Edge: 'b,
        'a: 'b,
    {
        self.unwrap()
            .boundary_back(self.mesh())
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }
}

impl<'a, T: MeshType> PartialEq for EdgeCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same edge id and pointing to the same mesh instance
        self.edge == other.edge && self.mesh as *const _ == other.mesh as *const _
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursor<'a, T> {
    type VC = VertexCursor<'a, T>;
    type FC = FaceCursor<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for EdgeCursor<'a, T> {
    type I = T::E;
    type S = T::Edge;
    type T = T;

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_edge(self.try_id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Edge> {
        self.mesh().get_edge(self.try_id())
    }

    #[inline]
    fn try_id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> EdgeCursor<'a, T> {
        Self::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursor<'a, T> {}
impl<'a, T: MeshType + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursor<'a, T> where
    T::Edge: HalfEdge<T>
{
}

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
