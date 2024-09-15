//! Payloads for vertices in n-dimensional space.

use crate::math::{Rotator, Scalar, Transform, Vector};

#[cfg(feature = "bevy")]
pub mod bevy;

// TODO: remove the `Default` similar to the `DefaultEdgePayload`
/// A trait that defines how the payload of a vertex should behave.
pub trait VertexPayload: Clone + PartialEq + std::fmt::Debug {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;
}

/// The default vertex payload can be safely constructed with a default constructor.
/// For vertex payloads this is usually not the case when meaningful positions are required.
pub trait DefaultVertexPayload: VertexPayload + Default {}

/// A trait that defines how a vertex payload can be linearly transformed.
pub trait Transformable {
    /// The transformation type used in the payload.
    type Trans: Transform<S = Self::S, Vec = Self::Vec>;

    /// The rotation type used in the payload.
    type Rot: Rotator<Self::Vec>;

    /// The vector type used in the payload.
    type Vec: Vector<Self::S, Trans = Self::Trans>;

    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// Returns the coordinates of the payload as a reference.
    fn transform(&self, t: &Self::Trans) -> Self;

    /// Returns a translated clone of the payload.
    fn translate(&self, v: &Self::Vec) -> Self;

    /// Returns the scaled clone of the payload.
    fn scale(&self, s: &Self::Vec) -> Self;

    /// Returns the rotated clone of the payload.
    fn rotate(&self, r: &Self::Rot) -> Self;

    /// Interpolates between two payloads.
    fn lerp(&self, other: &Self, t: Self::S) -> Self;
}

/// Indicates that the vertex payload has a position vector.
pub trait HasPosition<Vec: Vector<Self::S>> {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// Get the position vector of the payload.
    fn pos(&self) -> &Vec;

    /// Creates a payload from a vector.
    fn from_pos(v: Vec) -> Self;

    /// Set the position vector of the payload.
    fn set_pos(&mut self, v: Vec);
}

/// Indicates that the vertex payload has a normal vector.
pub trait HasNormal<Vec: Vector<Self::S>> {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// returns the normals of the payload as an array
    fn normal(&self) -> &Vec;

    /// Sets the normals.
    fn set_normal(&mut self, normal: Vec);
}

// TODO: use this whenever it is required for the position to be euclidean.
//pub trait IsEuclidean {}

/// An empty vertex payload if you don't need any vertex information.
/// Notice that your mesh will behave more like a graph without any payload.
// TODO: implement this. Requires the VertexPayload to be weaker and use a separate, stronger trait (e.g., `EuclideanVertexPayload`) for the full payload.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct EmptyVertexPayload;
