mod iterator;
//pub use iterator::*;
//mod interpolator;
pub mod payload;

// pub use interpolator::*;

use super::MeshType;
use crate::math::{HasPosition, Scalar, Transformable, Vector};

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

    /// Returns whether the vertex is a boundary vertex
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;

    /// Returns whether the vertex has only one edge incident to it
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool;

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Vertex> + 'a;

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Face> + 'a
    where
        T: 'a;

    /// Transforms the payload.
    fn transform(&mut self, transform: &T::Trans)
    where
        T::VP: Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>;

    /// Translates the payload.
    fn translate(&mut self, transform: &T::Vec)
    where
        T::VP: Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>;

    /// Rotates the payload.
    fn rotate(&mut self, transform: &T::Rot)
    where
        T::VP: Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>;

    /// Scales the payload.
    fn scale(&mut self, transform: &T::Vec)
    where
        T::VP: Transformable<Trans = T::Trans, Rot = T::Rot, Vec = T::Vec, S = T::S>;
}
