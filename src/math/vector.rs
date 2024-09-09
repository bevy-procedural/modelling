use super::{kahan_summation, HasZero, Scalar, Transform, Vector2D, Vector3D};

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
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Mul<S, Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + HasZero
    + 'static
{
    /// The 2d vector type used in the coordinates.
    type Vec2: Vector2D<S = S>;

    /// The 3d vector type used in the coordinates.
    type Vec3: Vector3D<S = S>;

    /// The rotation type used in the vector.
    type Trans: Transform<S = S, Vec = Self>;

    /// Returns the number of dimensions.
    fn dimensions() -> usize;

    /// Returns the distance between two points.
    fn distance(&self, other: &Self) -> S;

    /// Returns the squared distance between two points.
    fn distance_squared(&self, other: &Self) -> S;

    /// Returns the dot product of two vectors.
    fn dot(&self, other: &Self) -> S;

    /// Returns the cross product of two vectors.
    fn cross(&self, other: &Self) -> Self;

    /// Returns the x-coordinate.
    fn x(&self) -> S;

    /// Returns the y-coordinate. (or 0 if not present)
    fn y(&self) -> S;

    /// Returns the z-coordinate. (or 0 if not present)
    fn z(&self) -> S;

    /// Returns the w-coordinate. (or 0 if not present)
    fn w(&self) -> S;

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
