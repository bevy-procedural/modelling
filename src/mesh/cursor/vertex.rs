use std::fmt::Debug;

use crate::{
    math::IndexType,
    mesh::{MeshBasics, MeshType, MeshTypeHalfEdge},
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
    fn id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_vertex(self.id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.id())
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
    fn id(&self) -> T::V {
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
    fn is_none(&self) -> bool {
        self.id() == IndexType::max() || !self.mesh().has_vertex(self.id())
    }

    #[inline]
    fn get<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.id())
    }
}

pub trait VertexCursorBasics<'a, T: MeshTypeHalfEdge + 'a>: VertexCursorData<'a, T> {
    fn outgoing_edge(&self) -> Self::EC {
        let edge = todo!();
        self.move_to_edge(edge)
    }
}

impl<'a, T: MeshTypeHalfEdge + 'a> VertexCursorBasics<'a, T> for VertexCursor<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> VertexCursorBasics<'a, T> for VertexCursorMut<'a, T> {}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_cursor() {
        let mut mesh = Mesh3d64::cube(1.0);
        let mut cursor: VertexCursor<'_, MeshType3d64PNU> =
            VertexCursor::new(&mesh, mesh.vertex_ids().next().unwrap());
    }
}
