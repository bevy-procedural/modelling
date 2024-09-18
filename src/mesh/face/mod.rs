//mod geometry;
mod face3d;
mod payload;

pub use face3d::*;
pub use payload::*;

use super::{MeshType, Vertex};
use crate::math::{HasPosition, VectorIteratorExt};

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait Face<T: MeshType<Face = Self>>:
    std::fmt::Display + Clone + Copy + PartialEq + Eq
{
    /// Returns the index of the face.
    fn id(&self) -> T::F;

    /// Returns an edge incident to the face.
    fn edge(&self, mesh: &T::Mesh) -> T::Edge;

    /// Whether the face is allowed to be curved.
    fn may_be_curved(&self) -> bool;

    /// Get the number of edges of the face.
    fn num_edges(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of vertices of the face.
    fn num_vertices(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of triangles of the face. (n-2)*3
    fn num_triangles(&self, mesh: &T::Mesh) -> usize;

    /// Returns the face payload.
    fn payload(&self) -> &T::FP;

    /// Returns a mutable reference to the face payload.
    fn payload_mut(&mut self) -> &mut T::FP;

    /// Iterates all vertices adjacent to the face
    fn vertices<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::Vertex> + 'a + Clone + ExactSizeIterator;

    /// Naive method to get the center of the face by averaging the vertices.
    fn center(&self, mesh: &T::Mesh) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: or centroid?
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

    /// Whether the face has holes.
    /// The data structure (currently!) cannot represent holes, so this is always false.
    fn has_holes(&self) -> bool {
        return false;
    }
}
