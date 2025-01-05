use std::fmt::Debug;

use crate::mesh::{MeshBasics, MeshType, MeshTypeHalfEdge};

use super::{EdgeCursor, EdgeCursorData, EdgeCursorMut};

#[derive(Clone, Debug)]
pub struct VertexCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursor<'a, T> {
    pub fn new(mesh: &'a T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    pub fn vertex(&self) -> T::V {
        self.vertex
    }
}

#[derive(Debug)]
pub struct VertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    pub fn vertex(&self) -> T::V {
        self.vertex
    }
}

pub trait VertexCursorData<'a, T: MeshType + 'a>: Sized + Debug {
    type EC: EdgeCursorData<'a, T>;
    fn id(&self) -> T::V;
    fn vertex<'b>(&'b self) -> &'b T::Vertex;
    fn mesh<'b>(&'b self) -> &'b T::Mesh;
    fn derive(self, id: T::V) -> Self;
    fn derive_ec(self, id: T::E) -> Self::EC;
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursor<'a, T> {
    type EC = EdgeCursor<'a, T>;

    #[inline(always)]
    fn id(&self) -> T::V {
        self.vertex
    }

    #[inline(always)]
    fn vertex<'b>(&'b self) -> &'b T::Vertex {
        self.mesh.vertex(self.vertex)
    }

    #[inline(always)]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline(always)]
    fn derive(self, id: T::V) -> VertexCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline(always)]
    fn derive_ec(self, id: T::E) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> VertexCursorData<'a, T> for VertexCursorMut<'a, T> {
    type EC = EdgeCursorMut<'a, T>;

    #[inline(always)]
    fn id(&self) -> T::V {
        self.vertex
    }

    #[inline(always)]
    fn vertex<'b>(&'b self) -> &'b T::Vertex {
        self.mesh.vertex(self.vertex)
    }

    #[inline(always)]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline(always)]
    fn derive(self, id: T::V) -> VertexCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline(always)]
    fn derive_ec(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

pub trait VertexCursorBasics<'a, T: MeshTypeHalfEdge + 'a>: VertexCursorData<'a, T> {
    fn outgoing_edge(&self) -> Self::EC {
        let edge = todo!();
        self.derive_ec(edge)
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
