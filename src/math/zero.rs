/// Trait for types that have a zero value.
pub trait HasZero {
    /// Returns the zero value for this type.
    fn zero() -> Self;

    /// Returns whether this value is zero.
    fn is_zero(&self) -> bool;
}
