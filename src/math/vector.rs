use super::{Scalar, Transform, Vector2D, Vector3D};

/// Trait for coordinates in n-dimensional space.
pub trait Vector<ScalarType: Scalar>:
    Copy
    + Default
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Mul<ScalarType, Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + 'static
{
    /// The 2d vector type used in the coordinates.
    type Vec2: Vector2D<ScalarType>;

    /// The 3d vector type used in the coordinates.
    type Vec3: Vector3D<ScalarType>;

    /// The rotation type used in the vector.
    type Trans: Transform<S = ScalarType, Vec = Self>;

    /// Returns the origin vector.
    fn zero() -> Self;

    /// Returns the number of dimensions.
    fn dimensions() -> usize;

    /// Returns the distance between two points.
    fn distance(&self, other: &Self) -> ScalarType;

    /// Returns the squared distance between two points.
    fn distance_squared(&self, other: &Self) -> ScalarType;

    /// Returns the dot product of two vectors.
    fn dot(&self, other: &Self) -> ScalarType;

    /// Returns the cross product of two vectors.
    fn cross(&self, other: &Self) -> Self;

    /// Returns the x-coordinate.
    fn x(&self) -> ScalarType;

    /// Returns the y-coordinate. (or 0 if not present)
    fn y(&self) -> ScalarType;

    /// Returns the z-coordinate. (or 0 if not present)
    fn z(&self) -> ScalarType;

    /// Returns the w-coordinate. (or 0 if not present)
    fn w(&self) -> ScalarType;

    /// Returns the coordinates as a tuple.
    fn xy(&self) -> Self::Vec2 {
        Self::Vec2::from_xy(self.x(), self.y())
    }

    /// Returns the coordinates as a tuple.
    fn xyz(&self) -> Self::Vec3 {
        Self::Vec3::from_xyz(self.x(), self.y(), self.z())
    }

    /// Normalizes the vector.
    fn normalize(&self) -> Self;
}
