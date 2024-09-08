/// To be used as a scalar in n-dimensional space.
pub trait Scalar:
    Copy
    + Default
    + PartialEq
    + PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + From<f32>
    + 'static
{
    /// The value of Ludolph's number.
    const PI: Self;

    /// The value of the machine epsilon.
    const EPS: Self;

    /// A value of zero.
    const ZERO: Self;
    
    /// A value of one.
    const ONE: Self;

    /// Returns whether the scalar is strictly positive.
    fn is_positive(self) -> bool;

    /// Returns whether the scalar is strictly negative.
    fn is_negative(self) -> bool;

    /// Converts the scalar to a 64-bit floating point number.
    fn to_f64(self) -> f64;

    /// Converts a usize to the scalar.
    fn from_usize(value: usize) -> Self;

    /// Returns the absolute value of the scalar.
    fn abs(self) -> Self {
        if self.is_positive() {
            self
        } else {
            -self
        }
    }

    /// Returns the arcus cosine of the scalar.
    fn acos(self) -> Self;

    /// Returns the maximum of two scalars.
    fn max(self: &Self, b: Self) -> Self;

    /// Returns the minimum of two scalars.
    fn min(self: &Self, b: Self) -> Self;

    /// Returns the square root of the scalar.
    fn sqrt(self) -> Self;

    /// Whether the scalar is finite.
    fn is_finite(self) -> bool;

    /// Whether the scalar is NaN.
    fn is_nan(self) -> bool;

    /// Returns the determinant of a 3x3 matrix.
    fn det3(
        a: Self,
        b: Self,
        c: Self,
        d: Self,
        e: Self,
        f: Self,
        g: Self,
        h: Self,
        i: Self,
    ) -> Self {
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }
}

/// A scalar that is ordered.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OrderedFloats<S: Scalar> {
    value: S,
}

impl<S: Scalar> OrderedFloats<S> {
    /// Create a new ordered float.
    pub fn new(value: S) -> Self {
        OrderedFloats { value }
    }
}

impl<S: Scalar> std::cmp::Eq for OrderedFloats<S> {}

impl<S: Scalar> std::cmp::Ord for OrderedFloats<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .partial_cmp(&other.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
