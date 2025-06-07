use super::{kahan_summation, HasZero, Scalar, Vector2D};

/// Trait for coordinates in n-dimensional space.
pub trait Vector<S: Scalar, const D: usize>:
    Copy
    + PartialEq
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    //+ std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<S, Output = Self>
    //+ std::ops::MulAssign
    + std::ops::Div<S, Output = Self>
    + std::ops::Neg<Output = Self>
    + HasZero
    + 'static
{
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
    fn vec2<Vec2: Vector2D<S=S>>(&self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }

    /// Create a vector from one coordinate
    fn from_x(x: S) -> Self;

    /// Create a vector from two coordinates. Drops the y-coordinate if not present.
    fn from_xy(x: S, y: S) -> Self;

    /// Create a vector from three coordinates. Drops the y- and z-coordinate if not present.
    fn from_xyz(x: S, y: S, z: S) -> Self;

    /// Normalizes the vector. Panics if the vector is the zero vector.
    fn normalize(&self) -> Self;

    /// Creates a vector with all the same coordinates.
    fn splat(value: S) -> Self;

    /// Calculate the sum of an iterator of vectors using some numerically stable algorithm.
    fn stable_sum(iter: impl IntoIterator<Item = Self>) -> Self {
        kahan_summation(iter).0
    }

    /// Calculate the mean of an iterator of vectors using some numerically stable algorithm.
    fn stable_mean(iter: impl IntoIterator<Item = Self>) -> Self {
        let (sum, count) = kahan_summation(iter);
        sum / S::from_usize(count)
    }

    /// Returns the angle of rotation (in radians) from self to rhs in the range [0, +π] resp. [-π, +π] for 2d.
    fn angle_between(&self, other: Self) -> S {
        let len_self = self.length();
        let len_other = other.length();

        if len_self.is_about(S::ZERO, S::EPS) || len_other.is_about(S::ZERO, S::EPS) {
            // Angle is undefined for zero-length vectors; handle as needed
            return S::ZERO;
        }

        if D == 2 {
            let det = self.x() * other.y() - self.y() * other.x();
            det.atan2(self.dot(&other))
        } else {                
            let cos_theta = self.dot(&other) / (len_self * len_other);

            // Clamp cos_theta to [-1, 1] to handle numerical inaccuracies
            cos_theta.clamp(-S::ONE, S::ONE).acos()
        }
    }

    /// Check if two vectors are approximately equal.
    fn is_about(&self, other: &Self, epsilon: S) -> bool;

    /// Returns the zero vector.
    fn zero() -> Self {
        Self::splat(S::ZERO)
    }
    
    /*/// Returns the components of the vector as a slice.
    fn as_slice(&self) -> &[S; D];

    /// Build a vector from a slice.
    fn from_slice(slice: &[S; D]) -> Self;

    /// Cast the components to another type.
    fn cast<T: Scalar + From<S>, Output: Vector<T, D>>(self) -> Output {
        Output::from_slice(&self.as_slice().map(|x| x.into()))
    }*/
}

/// Additional methods for vector iterators.
pub trait VectorIteratorExt<S: Scalar, const D: usize, V: Vector<S, D>>:
    Iterator<Item = V>
{
    /// Calculate the sum of an iterator of vectors using some numerically stable algorithm.
    fn stable_sum(self) -> Self::Item
    where
        Self: Sized,
    {
        V::stable_sum(self)
    }

    /// Calculate the mean of an iterator of vectors using some numerically stable algorithm.
    fn stable_mean(self) -> Self::Item
    where
        Self: Sized,
    {
        V::stable_mean(self)
    }
}

impl<I: Iterator<Item = V>, S: Scalar, const D: usize, V: Vector<S, D>> VectorIteratorExt<S, D, V>
    for I
{
}
