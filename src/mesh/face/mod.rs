//mod geometry;
mod basics;
mod face3d;
mod payload;
mod islands;

pub use basics::*;
pub use face3d::*;
pub use payload::*;
pub use islands::*;

use super::{cursor::*, EuclideanMeshType, MeshType};
use crate::math::VectorIteratorExt;

// TODO: Remove methods in trait Face

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait Face: FaceBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Face = Self>;

    /// Naive method to get the center of the face by averaging the vertices.
    fn centroid<const D: usize>(
        &self,
        mesh: &<Self::T as MeshType>::Mesh,
    ) -> <Self::T as EuclideanMeshType<D>>::Vec
    where
        Self::T: EuclideanMeshType<D>,
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
