/// A trait for types that have a zero value.
pub trait HasZero {
    /// A value of zero.
    const ZERO: Self;

    // TODO: remove this
}

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
    + std::ops::DivAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Neg<Output = Self>
    + From<f32>
    + HasZero
    + 'static
{
    /// The value of Ludolph's number.
    const PI: Self;

    /// The value of the machine epsilon.
    const EPS: Self;

    /// A value of one.
    const ONE: Self;

    /// A value of two.
    const TWO: Self;

    /// A value of three.
    const THREE: Self;

    /// A value of four.
    const FOUR: Self;

    /// A value of five.
    const FIVE: Self;

    /// A value of ten.
    const TEN: Self;

    /// A value of one half.
    const HALF: Self;

    /// The golden ratio.
    const PHI: Self;

    /// Positive infinity.
    const INFINITY: Self;

    /// Negative infinity.
    const NEG_INFINITY: Self;

    /// Returns whether the scalar is zero.
    fn is_zero(self) -> bool;

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

    /// Returns the sine of the scalar.
    fn sin(&self) -> Self;

    /// Returns the cosine of the scalar.
    fn cos(&self) -> Self;

    /// Returns the tangent of the scalar.
    fn tan(&self) -> Self;

    /// Returns the atan2 of the scalar.
    fn atan2(&self, x: Self) -> Self;

    /// Returns the maximum of two scalars.
    fn max(&self, b: Self) -> Self;

    /// Returns the minimum of two scalars.
    fn min(&self, b: Self) -> Self;

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

    /// Calculate the sum of an iterator of scalars using some numerically stable algorithm.
    fn stable_sum<I: Iterator<Item = Self>>(values: I) -> Self {
        neumaier_summation(values).0
    }

    /// Calculate the mean of an iterator of scalars using some numerically stable algorithm.
    fn stable_mean<I: Iterator<Item = Self>>(values: I) -> Self {
        let (sum, n) = neumaier_summation(values);
        sum / Self::from_usize(n)
    }

    /// Clamp the scalar to a given range.
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    /// Returns whether the scalar is about another scalar within a given epsilon.
    fn is_about(&self, other: Self, epsilon: Self) -> bool {
        (*self - other).abs() < epsilon
    }
}

/// Additional methods for scalar iterators.
pub trait ScalarIteratorExt<S: Scalar>: Iterator<Item = S> {
    /// Calculate the sum of an iterator of scalars using some numerically stable algorithm.
    fn stable_sum(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: std::ops::Add<Output = Self::Item> + HasZero,
    {
        Scalar::stable_sum(self)
    }

    /// Calculate the mean of an iterator of scalars using some numerically stable algorithm.
    fn stable_mean(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: std::ops::Add<Output = Self::Item> + HasZero,
    {
        Scalar::stable_mean(self)
    }
}

impl<I: Iterator<Item = S>, S: Scalar> ScalarIteratorExt<S> for I {}

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

/// Neumaier summation algorithm.
/// This is a more numerically stable way to sum up a list of scalars.
/// It is especially useful when the scalars are of different magnitudes.
/// It's slightly more precise than Kahan summation.
pub fn neumaier_summation<S: Scalar, I: Iterator<Item = S>>(iter: I) -> (S, usize) {
    let mut sum = S::ZERO;
    let mut c = S::ZERO;
    let mut count = 0;
    for value in iter {
        count += 1;
        let t = sum + value;
        if sum.abs() >= value.abs() {
            c += (sum - t) + value;
        } else {
            c += (value - t) + sum;
        }
        sum = t;
    }
    (sum + c, count)
}

/// Kahan summation algorithm.
/// This is a more numerically stable way to sum up a list of scalars.
/// It can be overloaded with a very broad range of floating point types including most vectors.
pub fn kahan_summation<
    X: std::ops::Add<Output = X> + HasZero + std::ops::Sub<Output = X> + Copy,
    I: Iterator<Item = X>,
>(
    iter: I,
) -> (X, usize) {
    let mut sum = X::ZERO;
    let mut c = X::ZERO;
    let mut count = 0;
    for value in iter {
        count += 1;
        let y = value - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    (sum, count)
}
