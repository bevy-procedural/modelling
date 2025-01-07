use super::{
    CursorData, FaceCursor, FaceCursorData, FaceCursorMut, VertexCursor, VertexCursorData,
    VertexCursorMut,
};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge},
};
use std::fmt::Debug;

/// An edge cursor pointing to an edge of a mesh with an immutable reference to the mesh.
///
/// You should prefer using Cursors over direct access to the mesh data structures whenever possible.
/// You don't have to worry about performance, as the rust compiler will completely optimize them away.
/// For example, when compiling `cursor.next().next().next().next()`, all function
/// calls will be inlined leading to the same 8 commands for each call to `next`:
/// ```ir
/// getelementptr + load    ; compute address of and load the `id` in the `HalfEdgeImpl` in the `Vec`
/// icmp + br               ; if the `id` is `IndexType::max()`, skip all further blocks (since it is deleted)
/// getelementptr + load    ; compute address of and load the `next_id` in the `HalfEdgeImpl`
/// icmp + br               ; if the `next_id` exceeds the length of the `Vec` or is `IndexType::max()`, skip all further blocks
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
pub trait EdgeCursorData<'a, T: MeshType + 'a>: CursorData<T = T, I = T::E, S = T::Edge> {
    /// The associated vertex cursor type
    type VC: VertexCursorData<'a, T>;

    /// The associated face cursor type
    type FC: FaceCursorData<'a, T>;

    /// Derives a new vertex cursor pointing to the given vertex id.
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new face cursor pointing to the given face id.
    fn move_to_face(self, id: T::F) -> Self::FC;
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
    fn is_none(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_edge(self.id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Edge> {
        self.mesh().get_edge(self.id())
    }

    #[inline]
    fn id(&self) -> T::E {
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

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursorMut<'a, T> {
    type VC = VertexCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for EdgeCursorMut<'a, T> {
    type I = T::E;
    type S = T::Edge;
    type T = T;

    #[inline]
    fn is_none(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_edge(self.id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Edge> {
        self.mesh().get_edge(self.id())
    }

    #[inline]
    fn id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::E) -> EdgeCursorMut<'a, T> {
        Self::new(self.mesh, id)
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
        self.move_to_vertex(id)
    }

    /// Moves the cursor to the target vertex of the edge.
    #[inline]
    fn target(self) -> Self::VC {
        let id = if let Some(e) = self.get() {
            e.target_id(self.mesh())
        } else {
            IndexType::max()
        };
        self.move_to_vertex(id)
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

    /// Moves the cursor to the face of the edge.
    #[inline]
    fn face(self) -> Self::FC {
        let id = self.face_id();
        self.move_to_face(id)
    }

    /// Moves the cursor to the face of the edge.
    #[inline]
    fn face_id(&self) -> T::F {
        self.get().map_or(IndexType::max(), |e| e.face_id())
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
        self.move_to(e)
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
    /// See [MeshBuilder::insert_vertex_e] for more information.
    #[inline]
    pub fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Self {
        let old_target = self.target_id();
        if let Some((e, _v)) = self.mesh.insert_vertex_e(self.edge, vp, ep) {
            let c = self.move_to(e);
            debug_assert!(old_target == c.origin_id());
            c
        } else {
            self.none()
        }
    }

    /// Connects the current halfedge to the given halfedge.
    /// On error, the resulting cursor will be `None`.
    /// See [MeshBuilder::insert_edge_ee] for more information.
    #[inline]
    pub fn connect(self, other: T::E, ep: T::EP) -> Self {
        if let Some(e) = self.mesh.insert_edge_ee(self.edge, other, ep) {
            self.move_to(e)
        } else {
            self.none()
        }
    }

    /// Connects the current halfedge to the given vertex.
    /// On error, the resulting cursor will be `None`.
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    pub fn connect_v(self, other: T::V, ep: T::EP) -> Self {
        if let Some(e) = self.mesh.insert_edge_ev(self.edge, other, ep) {
            self.move_to(e)
        } else {
            self.none()
        }
    }

    /// Inserts a face in the boundary of the current halfedge and move the cursor to the new face.
    /// If the face already exists, move there and make the current edge the representative edge.
    /// If there was another error, the resulting cursor will be `None`.
    /// See [MeshBuilder::insert_face] for more information.
    #[inline]
    pub fn insert_face(self, fp: T::FP) -> FaceCursorMut<'a, T>
    where
        // TODO: We should remove this bound by implementing face_id for all edges
        T: MeshTypeHalfEdge,
    {
        if let Some(f) = self.mesh.insert_face(self.edge, fp) {
            self.move_to_face(f)
        } else {
            self.face()
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
            c = f(c).move_to(c_id).next_sibling();
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_edge_cursor() {
        let mut mesh = Mesh3d64::cube(1.0);
        let e0 = mesh.edge_ids().next().unwrap();
        let c1: EdgeCursor<'_, MeshType3d64PNU> = mesh.edge_cursor(e0).next();
        let c2 = c1.clone().next();
        let c3 = c1.clone().next().prev().next();
        assert_ne!(c1, c2);
        assert_eq!(c1, c1);
        assert_eq!(c2, c3);

        let c1: EdgeCursorMut<'_, MeshType3d64PNU> = mesh.edge_cursor_mut(e0).next();
        /*c1.next()
        .subdivide(std::iter::empty())
        .next()
        .subdivide(std::iter::empty());*/
    }
}
