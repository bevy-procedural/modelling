//mod geometry;
mod basics;
mod face3d;
mod payload;

pub use basics::*;
pub use face3d::*;
pub use payload::*;

use super::{MeshType, Vertex};
use crate::math::{HasPosition, VectorIteratorExt};

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait Face<T: MeshType<Face = Self>>: FaceBasics<T> {
    /// Naive method to get the center of the face by averaging the vertices.
    fn centroid(&self, mesh: &T::Mesh) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices(mesh).map(|v| v.pos()).stable_mean()
    }

    /// Whether a triangle shares a halfedge with the face.
    ///
    /// If there is no evidence that the triangle is touching the face, return None.
    /// Given that all vertices are part of this face, this implies that the triangle is part of the face.
    fn triangle_touches_boundary(
        &self,
        mesh: &T::Mesh,
        v0: T::V,
        v1: T::V,
        v2: T::V,
    ) -> Option<bool>;
}
