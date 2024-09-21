mod payload;

pub use payload::*;

use super::{MeshType, VertexBasics};
use crate::math::{HasPosition, Scalar};

/// A directed (e.g., halfedge) or undirected edge in a mesh.
pub trait Edge: std::fmt::Debug + Clone + Copy + PartialEq {
    type T: MeshType<Edge = Self>;

    /// Returns the index of the edge
    fn id(&self) -> <Self::T as MeshType>::E;

    /// Returns the source vertex of the edge. If it is not directed, can be either vertex but not the same as the target.
    fn origin<'a>(
        &'a self,
        mesh: &'a <Self::T as MeshType>::Mesh,
    ) -> &'a <Self::T as MeshType>::Vertex;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target<'a>(
        &'a self,
        mesh: &'a <Self::T as MeshType>::Mesh,
    ) -> &'a <Self::T as MeshType>::Vertex;

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    fn is_boundary(&self, mesh: &<Self::T as MeshType>::Mesh) -> bool;

    /// Returns the centroid of the edge.
    fn centroid(&self, mesh: &<Self::T as MeshType>::Mesh) -> <Self::T as MeshType>::Vec
    where
        <Self::T as MeshType>::VP:
            HasPosition<<Self::T as MeshType>::Vec, S = <Self::T as MeshType>::S>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * <Self::T as MeshType>::S::HALF
    }
}
