use super::{Rotator, Scalar, Transform, Vector};

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
