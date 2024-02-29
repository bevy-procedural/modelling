//! Payloads for vertices in n-dimensional space.

mod vector;
pub use vector::*;

#[cfg(feature = "bevy")]
mod bevy;

/// Trait for the payload of vertices.
pub trait Payload: Clone + Default + PartialEq + Vector<Self::S> + std::fmt::Debug {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;
}
