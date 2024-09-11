//! Plain f64 implementation of the mathematical traits.

use crate::math::{HasZero, Scalar};

impl HasZero for f64 {
    const ZERO: Self = 0.0;
}

impl Scalar for f64 {
    const PI: Self = std::f64::consts::PI;
    const EPS: Self = std::f64::EPSILON;
    const ONE: Self = 1.0;

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
        f64::acos(self)
    }

    #[inline(always)]
    fn sin(&self) -> Self {
        f64::sin(*self)
    }

    #[inline(always)]
    fn cos(&self) -> Self {
        f64::cos(*self)
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self
    }

    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as f64
    }

    #[inline(always)]
    fn max(&self, b: Self) -> Self {
        f64::max(*self, b)
    }

    #[inline(always)]
    fn min(&self, b: Self) -> Self {
        f64::min(*self, b)
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }

    #[inline(always)]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }
}
