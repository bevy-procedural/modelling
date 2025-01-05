use crate::mesh::{
    EdgeBasics, HalfEdge, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge, VertexBasics,
};
use std::fmt::Debug;

use super::{VertexCursor, VertexCursorData, VertexCursorMut};

#[derive(Clone, Debug, Eq)]
pub struct EdgeCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> EdgeCursor<'a, T> {
    #[inline(always)]
    pub fn new(mesh: &'a T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    #[inline(always)]
    pub fn edge(&self) -> T::E {
        self.edge
    }
}

impl<'a, T: MeshType> PartialEq for EdgeCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.edge == other.edge && self.mesh as *const _ == other.mesh as *const _
    }
}

#[derive(Debug)]
pub struct EdgeCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    edge: T::E,
}

impl<'a, T: MeshType> EdgeCursorMut<'a, T> {
    #[inline(always)]
    pub fn new(mesh: &'a mut T::Mesh, edge: T::E) -> Self {
        Self { mesh, edge }
    }

    #[inline(always)]
    pub fn edge(&self) -> T::E {
        self.edge
    }
}

pub trait EdgeCursorData<'a, T: MeshType + 'a>: Sized + Debug {
    type VC: VertexCursorData<'a, T>;

    fn id(&self) -> T::E;
    fn edge<'b>(&'b self) -> &'b T::Edge;
    fn mesh<'b>(&'b self) -> &'b T::Mesh;
    fn derive(self, id: T::E) -> Self;
    fn derive_vc(self, id: T::V) -> Self::VC;
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursor<'a, T> {
    type VC = VertexCursor<'a, T>;

    #[inline(always)]
    fn id(&self) -> T::E {
        self.edge
    }

    #[inline(always)]
    fn edge<'b>(&'b self) -> &'b T::Edge {
        self.mesh.edge(self.edge)
    }

    #[inline(always)]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline(always)]
    fn derive(self, id: T::E) -> EdgeCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline(always)]
    fn derive_vc(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorData<'a, T> for EdgeCursorMut<'a, T> {
    type VC = VertexCursorMut<'a, T>;

    #[inline(always)]
    fn id(&self) -> T::E {
        self.edge
    }

    #[inline(always)]
    fn edge<'b>(&'b self) -> &'b T::Edge {
        self.mesh.edge(self.edge)
    }

    #[inline(always)]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline(always)]
    fn derive(self, id: T::E) -> EdgeCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline(always)]
    fn derive_vc(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }
}

pub trait EdgeCursorBasics<'a, T: MeshType + 'a>: EdgeCursorData<'a, T> {
    #[inline(always)]
    fn origin(self) -> Self::VC {
        let id = self.edge().origin(self.mesh()).id();
        self.derive_vc(id) // TODO: Use origin_id instead of origin
    }

    #[inline(always)]
    fn target(self) -> Self::VC {
        let id = self.edge().target(self.mesh()).id();
        self.derive_vc(id)
    }
}

pub trait EdgeCursorHalfedgeBasics<'a, T: MeshTypeHalfEdge + 'a>: EdgeCursorData<'a, T> {
    #[inline(always)]
    fn next(self) -> Self {
        let id = self.edge().next_id();
        self.derive(id)
    }

    #[inline(always)]
    fn prev(self) -> Self {
        let id = self.edge().prev_id();
        self.derive(id)
    }

    #[inline(always)]
    fn twin(self) -> Self {
        let id: <T as MeshType>::E = self.edge().twin_id();
        self.derive(id)
    }
}

impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursor<'a, T> {}
impl<'a, T: MeshType + 'a> EdgeCursorBasics<'a, T> for EdgeCursorMut<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursor<'a, T> {}
impl<'a, T: MeshTypeHalfEdge + 'a> EdgeCursorHalfedgeBasics<'a, T> for EdgeCursorMut<'a, T> {}

impl<'a, T: MeshType + 'a> EdgeCursorMut<'a, T> {
    pub fn subdivide<I: Iterator<Item = (T::EP, T::VP)>>(self, vs: I) -> Self {
        let e = self.mesh.subdivide_edge::<I>(self.edge, vs);
        self.derive(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_cursor() {
        let mut mesh = Mesh3d64::cube(1.0);
        let c1: EdgeCursor<'_, MeshType3d64PNU> =
            EdgeCursor::new(&mesh, mesh.edge_ids().next().unwrap()).next();
        let c2 = c1.clone().next();
        let c3 = c1.clone().next().prev().next();
        assert!(c1 != c2);
        assert!(c1 == c3);
    }
}
