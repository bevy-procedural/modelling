use std::fmt::Debug;

use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdgeVertex, MeshBasics, MeshType, MeshTypeHalfEdge, VertexBasics},
};

use super::{
    CursorData, EdgeCursor, EdgeCursorData, EdgeCursorMut, FaceCursor, FaceCursorData,
    FaceCursorMut,
};

/// A vertex cursor pointing to a vertex of a mesh with an immutable reference to the mesh.
#[derive(Clone, Debug)]
pub struct VertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursor<'a, T> {
    /// Creates a new vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Returns an iterator of edge cursors pointing to the outgoing halfedges of the vertex.
    /// Panics if the vertex is void.
    /// TODO: would be nice to return an empty iterator if the vertex is void instead?
    /// See [VertexBasics::edges_out] for more information.
    pub fn edges_out(&'a self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.unwrap()
            .edges_out(self.mesh)
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }

    /// Returns an iterator of edge cursors pointing to the incoming halfedges of the vertex.
    /// Panics if the vertex is void.
    /// TODO: would be nice to return an empty iterator if the vertex is void instead?
    /// See [VertexBasics::edges_in] for more information.
    pub fn edges_in(&'a self) -> impl Iterator<Item = EdgeCursor<'a, T>> {
        self.unwrap()
            .edges_in(self.mesh)
            .map(move |e| EdgeCursor::new(self.mesh, e.id()))
    }

    /// Returns a reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&self) -> &T::VP {
        self.unwrap().payload()
    }
}

impl<'a, T: MeshTypeHalfEdge> VertexCursor<'a, T> {
    //
}

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
#[derive(Debug)]
pub struct VertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Returns a mutable reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::VP {
        self.mesh.vertex_ref_mut(self.vertex).payload_mut()
    }
}

/// This trait defines the basic functionality for accessing the data fields of a vertex cursor.
pub trait VertexCursorData<'a, T: MeshType + 'a>:
    CursorData<T = T, I = T::V, S = T::Vertex>
{
    /// The associated face cursor type
    type FC: FaceCursorData<'a, T>;

    /// The associated edge cursor type
    type EC: EdgeCursorData<'a, T>;

    /// Derives a new face cursor pointing to the given face id.
    fn move_to_face(self, id: T::F) -> Self::FC;

    /// Derives a new edge cursor pointing to the given vertex id.
    fn move_to_edge(self, id: T::E) -> Self::EC;
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursor<'a, T> {
    type EC = EdgeCursor<'a, T>;
    type FC = FaceCursor<'a, T>;

    #[inline]
    fn move_to_face(self, id: T::F) -> Self::FC {
        FaceCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for VertexCursor<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursorMut<'a, T> {
    type EC = EdgeCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> CursorData for VertexCursorMut<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

/// This trait implements some basic functionality for vertex cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait VertexCursorBasics<'a, T: MeshType + 'a>: VertexCursorData<'a, T> {
    /// Returns an edge cursor pointing to a representative edge incident to the vertex.
    #[inline]
    #[must_use]
    fn edge(self) -> Self::EC {
        let edge = self.unwrap().edge_id(self.mesh());
        self.move_to_edge(edge)
    }

    /// Returns the id of a representative edge incident to the vertex, `IndexType::max()` if it has none, or panic if the vertex is void.
    #[inline]
    #[must_use]
    fn edge_id(&self) -> T::E {
        self.unwrap().edge_id(self.mesh())
    }

    /// Whether the vertex is isolated.
    /// Panics if the vertex is void.
    /// See [VertexBasics::is_isolated] for more information.
    #[inline]
    #[must_use]
    fn is_isolated(&self) -> bool {
        self.unwrap().is_isolated(self.mesh())
    }
}

/// This trait implements some basic functionality for vertex cursors that works with half edge meshes and both mutable and immutable cursors.
pub trait VertexCursorHalfedgeBasics<'a, T: MeshTypeHalfEdge + 'a>:
    VertexCursorData<'a, T>
{
    /*/// Returns an edge cursor pointing to an outgoing halfedge incident to the vertex.
    /// If the vertex is void, the edge cursor is void. Won't panic.
    #[inline]
    #[must_use]
    fn outgoing_edge(self) -> Self::EC {
        let edge = todo!();
        self.move_to_edge(edge)
    }*/

    /// Returns an ingoing boundary edge incident to the vertex.
    /// Panics if the vertex is void.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn ingoing_boundary_edge(&self) -> Option<T::E> {
        HalfEdgeVertex::ingoing_boundary_edge(self.unwrap(), self.mesh())
    }

    /// Returns an outgoing boundary edge incident to the vertex.
    /// Panics if the vertex is void.
    /// See [HalfEdgeVertex::ingoing_boundary_edge] for more information.
    #[inline]
    #[must_use]
    fn outgoing_boundary_edge(&self) -> Option<T::E> {
        HalfEdgeVertex::outgoing_boundary_edge(self.unwrap(), self.mesh())
    }
}

impl<'a, T: MeshType + 'a> VertexCursorBasics<'a, T> for VertexCursor<'a, T> {}
impl<'a, T: MeshType + 'a> VertexCursorBasics<'a, T> for VertexCursorMut<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> VertexCursorHalfedgeBasics<'a, T> for VertexCursor<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> VertexCursorHalfedgeBasics<'a, T> for VertexCursorMut<'a, T> {}
