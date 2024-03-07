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

    /// Returns whether the scalar is strictly positive.
    fn is_positive(self) -> bool;

    /// Returns whether the scalar is strictly negative.
    fn is_negative(self) -> bool;

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
