use super::{Scalar, Vector3D, Vector4D};

/// Trait for quarternions.
pub trait Quarternion: Clone + Copy + Default + std::fmt::Debug + 'static {
    /// The scalar type of the coordinates and angles used in the rotation.
    type S: Scalar;

    /// The 3d vector type used in the rotation.
    type Vec3: Vector3D<S = Self::S>;

    /// The 4d vector type used in the rotation.
    type Vec4: Vector4D<S = Self::S>;

    /// Returns the identity rotation.
    fn identity() -> Self;

    /// Returns a rotation from a rotation arc.
    fn from_rotation_arc(from: Self::Vec3, to: Self::Vec3) -> Self;

    /// Returns a rotation from an axis and an angle.
    fn from_axis_angle(axis: Self::Vec3, angle: Self::S) -> Self;

    /// Returns the axis and angle of the rotation.
    fn axis_angle(&self) -> (Self::Vec3, Self::S);

    /// Returns the matrix representation of the rotation.
    fn vec4(&self) -> Self::Vec4;
}
