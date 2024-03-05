//! Payloads for vertices in n-dimensional space.

use crate::math::{Scalar, Transform, Vector, Vector2D, Vector3D};

#[cfg(feature = "bevy")]
pub mod bevy;

/// Trait for the payload of vertices.
pub trait Payload: Clone + Default + PartialEq + std::fmt::Debug {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// The vector type used in the payload.
    type Vec: Vector<Self::S, Vec2D = Self::Vec2, Vec3D = Self::Vec3, Transform = Self::Trans>;

    /// The 2d vector type used in the payload.
    type Vec2: Vector2D<Self::S>;

    /// The 3d vector type used in the payload.
    type Vec3: Vector3D<Self::S>;

    /// The transformation type used in the payload.
    type Trans: Transform<S = Self::S, Vec = Self::Vec>;

    /// Returns a translated clone of the payload.
    fn translate(&self, v: &Self::Vec) -> Self;

    /// Returns the coordinates of the payload as a reference.
    fn transform(&self, t: &<Self::Vec as Vector<Self::S>>::Transform) -> Self;

    /// returns the coordinates of the payload as an array
    fn vertex(&self) -> &Self::Vec;

    /// returns the normals of the payload as an array
    fn normal(&self) -> &Self::Vec;

    /// Sets the normals.
    fn set_normal(&mut self, normal: Self::Vec);

    /// Creates a payload from a vector.
    fn from_vec(v: Self::Vec) -> Self;
}
