use super::{Scalar, Vector};

/// Indicates that the vertex payload has a position vector.
pub trait HasPosition<const D: usize, Vec: Vector<Self::S, D>> {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with fixed point etc...
    type S: Scalar;

    /// Get the position vector of the payload.
    fn pos(&self) -> &Vec;

    /// Creates a payload from a vector.
    fn from_pos(v: Vec) -> Self;

    /// Set the position vector of the payload.
    fn set_pos(&mut self, v: Vec);
}

/// Indicates that the vertex payload has a normal vector.
pub trait HasNormal<const D: usize, Vec: Vector<Self::S, D>> {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with fixed point etc...
    type S: Scalar;

    /// returns the normals of the payload
    fn normal(&self) -> &Vec;

    /// Sets the normals.
    fn set_normal(&mut self, normal: Vec);
}

/// Indicates that the vertex payload has a uv coordinate vector. These coordinates are always 2D.
pub trait HasUV<Vec: Vector<Self::S, 2>> {
    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with fixed point etc...
    type S: Scalar;

    /// returns the uv coordinates of the payload
    fn uv(&self) -> &Vec;

    /// Sets the uv coordinates.
    fn set_uv(&mut self, normal: Vec);
}
