//! Payloads for vertices in n-dimensional space.

mod vector;
pub use vector::*;

#[cfg(feature = "bevy")]
pub mod bevy;

/// Trait for the payload of vertices.
pub trait Payload: Clone + Default + PartialEq + std::fmt::Debug {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// The vector type used in the payload.
    type Vec: Vector<Self::S>;

    /// Returns a translated clone of the payload.
    fn translate(&self, v: &Self::Vec) -> Self;

    /// returns the coordinates of the payload as an array
    fn vertex(&self) -> &Self::Vec;

    /// Creates a payload from a vector.
    fn from_vec(v: Self::Vec) -> Self;
}
