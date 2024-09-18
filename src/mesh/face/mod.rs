//mod geometry;
mod payload;

pub use payload::*;

use super::MeshType;

pub trait Face<T: MeshType>: std::fmt::Display + Clone + Copy + PartialEq + Eq {
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
}
