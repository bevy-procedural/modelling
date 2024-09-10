//! Payloads for vertices in n-dimensional space.

use crate::math::{Quarternion, Scalar, Transform, Vector, Vector2D, Vector3D, Vector4D};

#[cfg(feature = "bevy")]
pub mod bevy;

// TODO: remove the `Default` similar to the `DefaultEdgePayload`
/// A trait that defines how the payload of a vertex should behave.
pub trait VertexPayload: Clone + Default + PartialEq + std::fmt::Debug + std::fmt::Display {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// The vector type used in the payload.
    type Vec: Vector<Self::S, Vec2 = Self::Vec2, Vec3 = Self::Vec3, Trans = Self::Trans>;

    /// The 2d vector type used in the payload.
    type Vec2: Vector2D<S = Self::S>;

    /// The 3d vector type used in the payload.
    type Vec3: Vector3D<S = Self::S>;
    
    /// The 4d vector type used in the payload.
    type Vec4: Vector4D<S = Self::S>;

    /// The transformation type used in the payload.
    type Trans: Transform<S = Self::S, Vec = Self::Vec>;

    /// The quarternion type used in the payload.
    type Quat: Quarternion<S = Self::S, Vec3 = Self::Vec3>;

    /// Returns a translated clone of the payload.
    fn translate(&self, v: &Self::Vec) -> Self;

    /// Returns the coordinates of the payload as a reference.
    fn transform(&self, t: &Self::Trans) -> Self;

    /// Returns the rotated clone of the payload.
    fn rotate(&self, r: &Self::Quat) -> Self;

    /// returns the coordinates of the payload as an array
    fn pos(&self) -> &Self::Vec;

    /// returns the normals of the payload as an array
    fn normal(&self) -> &Self::Vec;

    /// Sets the normals.
    fn set_normal(&mut self, normal: Self::Vec);

    /// Creates a payload from a vector.
    fn from_pos(v: Self::Vec) -> Self;

    /// Set the vector of the payload.
    fn set_pos(&mut self, v: Self::Vec);
}

/// An empty vertex payload if you don't need any vertex information.
/// Notice that your mesh will behave more like a graph without any payload.
// TODO: implement this. Requires the VertexPayload to be weaker and use a separate, stronger trait (e.g., `EuclideanVertexPayload`) for the full payload.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct EmptyVertexPayload;
