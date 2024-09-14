//! Plain f32 implementation of the mathematical traits.

use crate::math::{HasZero, Rotator, Scalar, Vector2D};

impl HasZero for f32 {
    const ZERO: Self = 0.0;
}

impl Scalar for f32 {
    const PI: Self = std::f32::consts::PI;
    const EPS: Self = std::f32::EPSILON;
    const ONE: Self = 1.0;
    const PHI: Self = 1.61803398874989484820;

    #[inline(always)]
    fn is_positive(self) -> bool {
        self.is_sign_positive()
    }

    #[inline(always)]
    fn is_negative(self) -> bool {
        self.is_sign_negative()
    }

    #[inline(always)]
    fn acos(self) -> Self {
        f32::acos(self)
    }

    #[inline(always)]
    fn sin(&self) -> Self {
        f32::sin(*self)
    }

    #[inline(always)]
    fn cos(&self) -> Self {
        f32::cos(*self)
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as f32
    }

    #[inline(always)]
    fn max(&self, b: Self) -> Self {
        f32::max(*self, b)
    }

    #[inline(always)]
    fn min(&self, b: Self) -> Self {
        f32::min(*self, b)
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }

    #[inline(always)]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }
}

impl<V: Vector2D<S = f32>> Rotator<V> for f32 {}
