mod basics;
mod payload;

pub use basics::*;
pub use payload::*;

use super::{MeshType, VertexBasics};
use crate::math::{HasPosition, Scalar};

/// A directed (e.g., halfedge) or undirected edge in a mesh.
pub trait Edge: EdgeBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Edge = Self>;

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
