use super::{VertexCursor, VertexCursorData, VertexCursorMut};
use crate::{
    extensions::nalgebra::Mesh3d64,
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge},
};
use std::fmt::Debug;

/// An edge cursor pointing to an edge of a mesh with an immutable reference to the mesh.
///
/// In my experiments, the rust compiler optimizes EdgeCursors very well.
/// For example, when invoking `cursor.next().next().next().next()`, all function
/// calls will be inlined leading to similar blocks each consisting of nothing else but
/// ```ir
/// getelementptr   ; compute address of the `id` in the `HalfEdgeImpl` in the `Vec`
/// load            ; retrieve the `id` of the halfedge
/// icmp + br       ; if the `id` is `IndexType::max()`, skip all further blocks (since it is deleted)
/// getelementptr   ; compute address of the `next_id` in the `HalfEdgeImpl`
/// load            ; retrieve the next `next_id`
/// icmp + br       ; if the `next_id` exceeds the length of the `Vec` or is `IndexType::max()`, skip all further blocks
/// ```
/// (using `cargo rustc -- --emit=llvm-ir -O -C debuginfo=2`)
#[derive(Clone, Debug, Eq)]
pub struct EdgeCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> EdgeCursor<'a, T> {
    /// Creates a new edge cursor pointing to the given edge.
    #[inline]
    pub fn new(mesh: &'a T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    // TODO: this cannot be called. How to realize this?
    /*#[inline]
    pub fn mutable(self, mesh: &'a mut T::Mesh) -> EdgeCursorMut<'a, T> {
        assert!(self.mesh as *const _ == mesh as *const _);
        EdgeCursorMut::new(mesh, self.edge)
    }*/
}

impl<'a, T: MeshType> PartialEq for EdgeCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same edge id and pointing to the same mesh instance
        self.edge == other.edge && self.mesh as *const _ == other.mesh as *const _
    }
}

/// An edge cursor pointing to an edge of a mesh with a mutable reference to the mesh.
#[derive(Debug)]
pub struct EdgeCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> EdgeCursorMut<'a, T> {
    /// Creates a new mutable edge cursor pointing to the given edge.
    #[inline]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    /// Returns an immutable clone pointing to the same edge.
    #[inline]
    pub fn immutable(&'a self) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, self.edge)
    }
}

/// This trait defines the basic functionality for accessing the data fields of an edge cursor.
pub trait EdgeCursorData<'a, T: MeshType + 'a>: Sized + Debug {
    /// The associated vertex cursor type
    type VC: VertexCursorData<'a, T>;

    /// Returns the id of the edge the cursor points to.
    fn id(&self) -> T::E;

    /// Returns a reference to the edge the cursor points to..
    /// Panics if the edge does not exist or is deleted.
    #[inline]
    fn unwrap<'b>(&'b self) -> &'b T::Edge
    where
        'a: 'b,
    {
        MeshBasics::edge(self.mesh(), self.id())
    }

    /// Whether the cursor points to an invalid edge, i.e.,
    /// either having the maximum index or pointing to a deleted (half)edge.
    #[inline]
    fn is_none(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_edge(self.id())
    }

    /// Converts the cursor to a None-cursor
    #[inline]
    fn none(self) -> Self {
        self.derive(IndexType::max())
    }

    /// Panics if the cursor points to an invalid edge.
    /// Returns the same cursor otherwise.
    #[inline]
    fn expect(self, msg: &str) -> Self {
        if self.is_none() {
            panic!("{}", msg);
        }
        self
    }

    /// Returns a reference to the edge if it exists and is not deleted, otherwise `None`.
    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Edge>
    where
        'a: 'b,
    {
        self.mesh().get_edge(self.id())
    }

    /// Applies a closure to the edge if it exists and is not deleted, moving the cursor to the returned edge id.
    #[inline]
    fn map<F: FnOnce(&T::Edge) -> T::E>(self, f: F) -> Self {
        if let Some(e) = self.get() {
            let id = f(e);
            self.derive(id)
        } else {
            self.none()
        }
    }

    /// Returns a reference to the mesh the cursor points to.
    fn mesh<'b>(&'b self) -> &'b T::Mesh;

    /// Derives a new edge cursor pointing to the given edge id.
    fn derive(self, id: T::E) -> Self;

    /// Derives a new vertex cursor pointing to the given vertex id.
    fn derive_vc(self, id: T::V) -> Self::VC;
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursor<'a, T> {
    type VC = VertexCursor<'a, T>;

    #[inline]
    fn id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn derive(self, id: T::E) -> EdgeCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn derive_vc(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursorMut<'a, T> {
    type VC = VertexCursorMut<'a, T>;

    #[inline]
    fn id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn derive(self, id: T::E) -> EdgeCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn derive_vc(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }
}

/// This trait implements some basic functionality for edge cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait EdgeCursorBasics<'a, T: MeshType + 'a>: EdgeCursorData<'a, T> {
    /// Moves the cursor to the origin vertex of the edge.
    #[inline]
    fn origin(self) -> Self::VC {
        let id = if let Some(e) = self.get() {
            e.origin_id(self.mesh())
        } else {
            IndexType::max()
        };
        self.derive_vc(id)
    }

    /// Moves the cursor to the target vertex of the edge.
    #[inline]
    fn target(self) -> Self::VC {
        let id = if let Some(e) = self.get() {
            e.target_id(self.mesh())
        } else {
            IndexType::max()
        };
        self.derive_vc(id)
    }

    /// Returns the id of the origin vertex of the edge.
    #[inline]
    fn origin_id(&self) -> T::V {
        self.get()
            .map_or(IndexType::max(), |e| e.origin_id(self.mesh()))
    }

    /// Returns the id of the target vertex of the edge.
    #[inline]
    fn target_id(&self) -> T::V {
        self.get()
            .map_or(IndexType::max(), |e| e.target_id(self.mesh()))
    }
}

/// This trait implements some basic functionality for edge cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait EdgeCursorHalfedgeBasics<'a, T: MeshTypeHalfEdge + 'a>: EdgeCursorData<'a, T> {
    /// Moves the cursor to the next halfedge of the edge.
    #[inline]
    fn next(self) -> Self {
        self.map(|e| e.next_id())
    }

    /// Moves the cursor to the previous halfedge of the edge.
    #[inline]
    fn prev(self) -> Self {
        self.map(|e| e.prev_id())
    }

    /// Moves the cursor to the twin halfedge of the edge.
    #[inline]
    fn twin(self) -> Self {
        self.map(|e| e.twin_id())
    }

    /// Returns the id of the next halfedge of the edge.
    #[inline]
    fn next_id(&self) -> T::E {
        self.get().map_or(IndexType::max(), |e| e.next_id())
    }

    /// Returns the id of the previous halfedge of the edge.
    #[inline]
    fn prev_id(&self) -> T::E {
        self.get().map_or(IndexType::max(), |e| e.prev_id())
    }

    /// Returns the id of the twin halfedge of the edge.
    #[inline]
    fn twin_id(&self) -> T::E {
        self.get().map_or(IndexType::max(), |e| e.twin_id())
    }

    /// Moves the cursor to the sibling of the edge, i.e., the twin's next edge.
    /// Calling this repeatedly will return all outgoing halfedges with the same origin.
    /// If the origin is non-manifold, this might not reach all outgoing halfedges but only those in the same wheel.
    /// If you need all wheels, go to the target first. // TODO: Reference
    #[inline]
    fn next_sibling(self) -> Self {
        self.twin().next()
    }

    /// Moves the cursor to the previous sibling of the edge, i.e., the previous edge's twin.
    #[inline]
    fn prev_sibling(self) -> Self {
        self.prev().twin()
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursor<'a, T> {}
impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursorMut<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursor<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursorMut<'a, T> {}

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the edge etc. using a chaining syntax.
impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T> {
    #[inline]
    pub fn subdivide<I: Iterator<Item = (T::EP, T::VP)>>(self, vs: I) -> Self {
        let e = self.mesh.subdivide_edge::<I>(self.edge, vs);
        self.derive(e)
    }

    /// Tries to remove the current edge.
    /// If the edge was successfully removed or didn't exist, returns `None`.
    /// Otherwise, returns an cursor still pointing to the same edge.
    #[inline]
    pub fn remove(self) -> Option<Self> {
        if self.mesh.try_remove_edge(self.edge) {
            None
        } else if self.is_none() {
            None
        } else {
            Some(self)
        }
    }

    /// Inserts a new vertex and half-edge pair. The halfedge leading to the
    /// new vertex will become the "next" of the current edge and the cursor will move
    /// to this newly created halfedge.
    /// Returns the none cursor if the insertion was not successful.
    #[inline]
    pub fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Self {
        let old_target = self.target_id();
        if let Some((e, _v)) = self.mesh.insert_vertex_e(self.edge, vp, ep) {
            let c = self.derive(e);
            debug_assert!(old_target == c.origin_id());
            c
        } else {
            self.none()
        }
    }
}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorMut<'a, T> {
    /// Runs the closure on all outgoing halfedges of the target.
    pub fn all_next<F: Fn(Self) -> Self>(self, f: F) -> Self {
        let twin = self.twin();
        let id = twin.id();
        let mut c = twin.next_sibling();
        while c.id() != id {
            let c_id = c.id();
            // execute closure, reset to the original edge and continue with the next sibling
            c = f(c).derive(c_id).next_sibling();
        }
        c
    }
}

#[inline(never)]
pub fn marker1<'a>() -> (usize, Mesh3d64) {
    use crate::{extensions::nalgebra::*, prelude::*};
    let mesh = Mesh3d64::cube(1.0);
    let e0 = mesh.edge_ids().next().unwrap();
    (e0, mesh)
}

#[inline(never)]
pub fn marker2(x: bool) {
    assert!(x, "test message 2");
}

#[inline(never)]
pub fn test_inlining() {
    use crate::prelude::*;

    let (e0, mesh) = marker1();
    let c1 = mesh
        .edge_cursor(e0)
        .next()
        .next()
        .next()
        .next()
        .next()
        .next()
        .next()
        .next()
        .next()
        .next()
        .next();
    marker2(true);

    assert_eq!(c1.origin_id(), c1.next_id());
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_edge_cursor() {
        let mut mesh = Mesh3d64::cube(1.0);
        let e0 = mesh.edge_ids().next().unwrap();
        let c1: EdgeCursor<'_, MeshType3d64PNU> = EdgeCursor::new(&mesh, e0).next();
        let c2 = c1.clone().next();
        let c3 = c1.clone().next().prev().next();
        assert_ne!(c1, c2);
        assert_eq!(c1, c1);
        assert_eq!(c2, c3);

        let c1: EdgeCursorMut<'_, MeshType3d64PNU> = EdgeCursorMut::new(&mut mesh, e0).next();
        /*c1.next()
        .subdivide(std::iter::empty())
        .next()
        .subdivide(std::iter::empty());*/
    }
}
