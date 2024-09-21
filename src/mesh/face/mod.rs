//mod geometry;
mod basics;
mod face3d;
mod payload;

pub use basics::*;
pub use face3d::*;
pub use payload::*;

use super::{MeshType, VertexBasics};
use crate::math::{HasPosition, VectorIteratorExt};

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait Face: FaceBasics<Self::T> {
    type T: MeshType<Face = Self>;

    /// Naive method to get the center of the face by averaging the vertices.
    fn centroid(&self, mesh: &<Self::T as MeshType>::Mesh) -> <Self::T as MeshType>::Vec
    where
        <Self::T as MeshType>::VP:
            HasPosition<<Self::T as MeshType>::Vec, S = <Self::T as MeshType>::S>,
    {
        self.vertices(mesh).map(|v| v.pos()).stable_mean()
    }

    /// Whether a triangle shares a halfedge with the face.
    ///
    /// If there is no evidence that the triangle is touching the face, return None.
    /// Given that all vertices are part of this face, this implies that the triangle is part of the face.
    fn triangle_touches_boundary(
        &self,
        mesh: &<Self::T as MeshType>::Mesh,
        v0: <Self::T as MeshType>::V,
        v1: <Self::T as MeshType>::V,
        v2: <Self::T as MeshType>::V,
    ) -> Option<bool>;
}
