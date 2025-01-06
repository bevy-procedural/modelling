use super::Scalar;

/// Trait for types that have a zero value.
pub trait HasZero {
    /// Returns the zero value for this type.
    fn zero() -> Self;

    /// Returns whether this value is zero.
    fn is_zero(&self) -> bool;
}

impl<T: num_traits::Zero + Scalar> HasZero for T {
    #[inline]
    fn zero() -> Self {
        T::zero()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}
