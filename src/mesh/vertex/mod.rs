mod iterator;
//pub use iterator::*;
//mod interpolator;
pub mod payload;

// pub use interpolator::*;

use super::MeshType;
use crate::math::{HasPosition, Scalar, Vector};

/// A vertex in a mesh.
pub trait Vertex<T: MeshType<Vertex = Self>>: std::fmt::Display + Clone + PartialEq {
    /// Returns the index of the vertex
    fn id(&self) -> T::V;

    /// Returns the payload of the vertex
    fn payload(&self) -> &T::VP;

    /// Returns the vertex coordinates of the payload
    fn pos<Vec: Vector<S>, S: Scalar>(&self) -> Vec
    where
        T::VP: HasPosition<Vec, S = S>,
    {
        *self.payload().pos()
    }

    /// Returns a mutable reference to the payload of the vertex
    fn payload_mut(&mut self) -> &mut T::VP;

    /// Returns an outgoing edge incident to the vertex
    fn edge(&self, mesh: &T::Mesh) -> T::Edge;

    /*
    /// Returns whether the vertex is a boundary vertex
    fn is_boundary<T: MeshType<Vertex = Self>>(&self, mesh: &T::Mesh) -> bool;
    */

    /// Returns whether the vertex has only one edge incident to it
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool;
}
