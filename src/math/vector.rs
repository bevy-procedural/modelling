use super::{kahan_summation, HasZero, Scalar, Transform, Vector2D, Vector3D, Vector4D};

/// Trait for coordinates in n-dimensional space.
pub trait Vector<S: Scalar>:
    Copy
    + Default
    + PartialEq
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<S, Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Self, Output = Self>
    + std::ops::Div<S, Output = Self>
    + std::ops::Neg<Output = Self>
    + HasZero
    + 'static
{
    /// The associated 2d vector type
    type Vec2: Vector2D<S = S>;

    /// The associated 3d vector type
    type Vec3: Vector3D<S = S>;

    /// The associated 4d vector type
    type Vec4: Vector4D<S = S>;

    /// The data structure used for linear transformations of this vector.
    type Trans: Transform<S = S, Vec = Self>;

    /// Returns the number of dimensions.
    fn dimensions() -> usize;

    /// Returns the distance between two points.
    fn distance(&self, other: &Self) -> S;

    /// Returns the squared distance between two points.
    fn distance_squared(&self, other: &Self) -> S;

    /// Length of the vector
    fn length(&self) -> S;

    /// Squared length of the vector
    fn length_squared(&self) -> S;

    /// Returns the dot product of two vectors.
    fn dot(&self, other: &Self) -> S;

    /// Returns the x-coordinate.
    fn x(&self) -> S;

    /// Returns the y-coordinate. (or 0 if not present)
    fn y(&self) -> S;

    /// Returns the z-coordinate. (or 0 if not present)
    fn z(&self) -> S;

    /// Returns the w-coordinate. (or 0 if not present)
    fn w(&self) -> S;

    /// Returns the coordinates as a tuple.
    fn vec2(&self) -> Self::Vec2 {
        <Self::Vec2 as Vector2D>::new(self.x(), self.y())
    }

    /// Returns the coordinates as a tuple.
    fn vec3(&self) -> Self::Vec3 {
        <Self::Vec3 as Vector3D>::new(self.x(), self.y(), self.z())
    }

    /// Create a vector from one coordinate
    fn from_x(x: S) -> Self;

    /// Create a vector from two coordinates. Drops the y-coordinate if not present.
    fn from_xy(x: S, y: S) -> Self;

    /// Create a vector from three coordinates. Drops the y- and z-coordinate if not present.
    fn from_xyz(x: S, y: S, z: S) -> Self;

    /// Normalizes the vector.
    fn normalize(&self) -> Self;

    /// Creates a vector with all the same coordinates.
    fn splat(value: S) -> Self;

    /// Sum of vectors, ideally numerically stable.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        kahan_summation(iter).0
    }

    /// Mean of vectors, ideally numerically stable.
    fn mean<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (sum, count) = kahan_summation(iter);
        sum / Self::splat(S::from_usize(count))
    }
}
