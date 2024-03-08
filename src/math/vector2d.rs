use super::{Scalar, Vector};

/// Trait for coordinates in 2d space.
pub trait Vector2D<S: Scalar>: Vector<S> {
    /// Construct from scalar values.
    fn from_xy(x: S, y: S) -> Self;

    /// True iff the vertex curr is a convex corner.
    /// Assume counter-clockwise vertex order.
    #[inline(always)]
    fn convex(&self, prev: Self, next: Self) -> bool {
        // TODO: Numerical robustness
        (*self - prev).cross2d(&(next - *self)).is_positive()
    }

    /// True if the vertex is collinear.
    fn collinear(&self, a: Self, b: Self, eps: S) -> bool {
        let ab = b - a;
        let ac = *self - a;
        ab.cross2d(&ac).abs() <= eps
    }

    /// Magnitude of the vector.
    fn magnitude(&self) -> S;

    /// Angle between two vectors.
    fn angle(&self, a: Self, b: Self) -> S;

    /// Returns the barycentric sign of a point in a triangle.
    #[inline(always)]
    fn barycentric_sign(a: Self, b: Self, c: Self) -> S {
        (a - c).cross2d(&(b - c))
    }

    /// Returns the cross product of two 2d vectors.
    #[inline(always)]
    fn cross2d(&self, other: &Self) -> S {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Whether the point is inside the triangle.
    #[inline(always)]
    fn is_inside_triangle(&self, a: Self, b: Self, c: Self) -> bool {
        // TODO: Numerical robustness
        let bs1 = Self::barycentric_sign(*self, a, b);
        let bs2 = Self::barycentric_sign(*self, b, c);
        let bs3 = Self::barycentric_sign(*self, c, a);
        let inside_ccw = bs1.is_positive() && bs2.is_positive() && bs3.is_positive();
        let inside_cw = bs1.is_negative() && bs2.is_negative() && bs3.is_negative();
        inside_ccw || inside_cw
    }

    /// Whether the point is inside the circumcircle of the triangle.
    #[deprecated(since="0.1.0", note="use something more robust")]
    fn is_inside_circumcircle(&self, a: Self, b: Self, c: Self, eps: S) -> bool {
        // https://en.wikipedia.org/wiki/Delaunay_triangulation#Algorithms

        let adx = a.x() - self.x();
        let ady = a.y() - self.y();
        let bdx = b.x() - self.x();
        let bdy = b.y() - self.y();
        let cdx = c.x() - self.x();
        let cdy = c.y() - self.y();
        let d = S::det3(
            adx,
            ady,
            adx * adx + ady * ady,
            bdx,
            bdy,
            bdx * bdx + bdy * bdy,
            cdx,
            cdy,
            cdx * cdx + cdy * cdy,
        );
        d >= eps
    }
}
