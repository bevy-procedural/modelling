use super::{Scalar, Vector};

/// Trait for coordinates in 2d space.
pub trait Vector2D: Vector<Self::S> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// Construct from scalar values.
    fn new(x: Self::S, y: Self::S) -> Self;

    /// True iff the vertex curr is a convex corner.
    /// Assume counter-clockwise vertex order.
    #[inline(always)]
    fn convex(&self, prev: Self, next: Self) -> bool {
        // TODO: Numerical robustness
        (*self - prev).cross2d(&(next - *self)).is_positive()
    }

    /// True if the vertex is collinear.
    fn collinear(&self, a: Self, b: Self, eps: Self::S) -> bool {
        let ab = b - a;
        let ac = *self - a;
        ab.cross2d(&ac).abs() <= eps
    }

    /// Magnitude of the vector.
    fn magnitude(&self) -> Self::S;

    /// Angle between two vectors.
    fn angle(&self, a: Self, b: Self) -> Self::S;

    /// Returns the barycentric sign of a point in a triangle.
    #[inline(always)]
    fn barycentric_sign(a: Self, b: Self, c: Self) -> Self::S {
        (a - c).cross2d(&(b - c))
    }

    /// Returns the cross product of two 2d vectors.
    #[inline(always)]
    fn cross2d(&self, other: &Self) -> Self::S {
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
    #[deprecated(since = "0.1.0", note = "use something more robust")]
    fn is_inside_circumcircle(&self, a: Self, b: Self, c: Self, eps: Self::S) -> bool {
        // https://en.wikipedia.org/wiki/Delaunay_triangulation#Algorithms

        let adx = a.x() - self.x();
        let ady = a.y() - self.y();
        let bdx = b.x() - self.x();
        let bdy = b.y() - self.y();
        let cdx = c.x() - self.x();
        let cdy = c.y() - self.y();
        let d = Self::S::det3(
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

    /// Returns the coordinate values as a tuple.
    fn tuple(&self) -> (Self::S, Self::S) {
        (self.x(), self.y())
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
