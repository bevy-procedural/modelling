//! Plain f32 implementation of the mathematical traits.

use crate::math::Scalar;

impl Scalar for f32 {
    const PI: Self = std::f32::consts::PI;
    const EPS: Self = std::f32::EPSILON;

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
}
