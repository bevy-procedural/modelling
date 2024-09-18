//mod geometry;
mod payload;

pub use payload::*;

use super::MeshType;
use crate::math::IndexType;
use std::hash::Hash;

pub trait Face<E: IndexType, F: IndexType, FP: FacePayload>:
    std::fmt::Display + Default + Clone + Copy + PartialEq + Eq + Hash
{
    /// Returns the index of the face.
    fn id(&self) -> F;

    /// Returns an edge incident to the face.
    fn edge<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &T::Mesh) -> T::Edge;

    /// Whether the face is allowed to be curved.
    fn may_be_curved(&self) -> bool;

    /// Get the number of edges of the face.
    fn num_edges<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of vertices of the face.
    fn num_vertices<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of triangles of the face. (n-2)*3
    fn num_triangles<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &T::Mesh) -> usize;

    /// Returns the face payload.
    fn payload(&self) -> &FP;

    /// Returns a mutable reference to the face payload.
    fn payload_mut(&mut self) -> &mut FP;
}
