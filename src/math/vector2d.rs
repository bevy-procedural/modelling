use super::{Scalar, Vector};

/// Trait for coordinates in 2d space.
pub trait Vector2D: Vector<Self::S, 2> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// Construct from scalar values.
    fn new(x: Self::S, y: Self::S) -> Self;

    /// True iff the vertex curr is a convex corner.
    /// Assume counter-clockwise vertex order.
    #[inline(always)]
    fn convex(&self, prev: Self, next: Self) -> bool {
        // TODO: Numerical robustness
        (*self - prev).perp_dot(&(next - *self)).is_positive()
    }

    /// True if the vertex is collinear.
    fn collinear(&self, a: Self, b: Self, eps: Self::S) -> bool {
        let ab = b - a;
        let ac = *self - a;
        ab.perp_dot(&ac).abs() <= eps
    }

    /// Angle between the segment from self to a and the segment from self to b.
    ///
    /// TODO: Remove this
    fn angle_tri(&self, a: Self, b: Self) -> Self::S;

    /// Returns the barycentric sign of a point in a triangle.
    #[inline(always)]
    fn barycentric_sign(a: Self, b: Self, c: Self) -> Self::S {
        (a - c).perp_dot(&(b - c))
    }

    /// Returns the cross product (perpendicular dot product) of two 2d vectors.
    #[inline(always)]
    fn perp_dot(&self, other: &Self) -> Self::S {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Whether the point is inside the triangle.
    #[inline(always)]
    fn is_inside_triangle(&self, a: Self, b: Self, c: Self) -> bool {
        // TODO: Numerical robustness
        // TODO: Possible remove this
        let bs1 = Self::barycentric_sign(*self, a, b);
        let bs2 = Self::barycentric_sign(*self, b, c);
        let bs3 = Self::barycentric_sign(*self, c, a);
        let inside_ccw = bs1.is_positive() && bs2.is_positive() && bs3.is_positive();
        let inside_cw = bs1.is_negative() && bs2.is_negative() && bs3.is_negative();
        inside_ccw || inside_cw
    }

    /// Swizzle
    fn xy(&self) -> Self {
        Self::new(self.x(), self.y())
    }

    /// Swizzle
    fn yx(&self) -> Self {
        Self::new(self.y(), self.x())
    }
}
