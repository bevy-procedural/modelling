use super::VecN;
use crate::math::{Scalar, Vector, Vector2D};

/// A 2D vector.
pub type Vec2<S> = VecN<S, 2>;

impl<S: Scalar> Vector2D for Vec2<S> {
    type S = S;

    #[inline(always)]
    fn new(x: S, y: S) -> Self {
        Self::from([x, y])
    }

    fn perp_dot(&self, other: &Self) -> S {
        (self.x() * other.y()) - (self.y() * other.x())
    }
}
