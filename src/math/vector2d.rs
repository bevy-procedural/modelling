use super::{Scalar, Vector};

/// Trait for coordinates in 2d space.
pub trait Vector2D<ScalarType: Scalar>: Vector<ScalarType> {
    /// Construct from scalar values.
    fn from_xy(x: ScalarType, y: ScalarType) -> Self;

    /// True iff the vertex curr is a convex corner.
    /// Assume counter-clockwise vertex order.
    fn convex(&self, prev: Self, next: Self) -> bool {
        (*self - prev).cross2d(&(next - *self)).is_positive()
    }

    /// Returns the barycentric sign of a point in a triangle.
    fn barycentric_sign(a: Self, b: Self, c: Self) -> ScalarType {
        (a - c).cross2d(&(b - c))
    }

    /// Returns the cross product of two 2d vectors.
    fn cross2d(&self, other: &Self) -> ScalarType {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Whether the point is inside the triangle.
    fn is_inside_triangle(&self, a: Self, b: Self, c: Self) -> bool {
        let bs1 = Self::barycentric_sign(*self, a, b);
        let bs2 = Self::barycentric_sign(*self, b, c);
        let bs3 = Self::barycentric_sign(*self, c, a);
        let inside_ccw = bs1.is_positive() && bs2.is_positive() && bs3.is_positive();
        let inside_cw = bs1.is_negative() && bs2.is_negative() && bs3.is_negative();
        inside_ccw || inside_cw
    }

    /// Whether the point is inside the circumcircle of the triangle.
    fn is_inside_circumcircle(&self, a: Self, b: Self, c: Self) -> bool {
        // https://en.wikipedia.org/wiki/Delaunay_triangulation#Algorithms

        let adx = a.x() - self.x();
        let ady = a.y() - self.y();
        let bdx = b.x() - self.x();
        let bdy = b.y() - self.y();
        let cdx = c.x() - self.x();
        let cdy = c.y() - self.y();
        ScalarType::det3(
            adx,
            ady,
            adx * adx + ady * ady,
            bdx,
            bdy,
            bdx * bdx + bdy * bdy,
            cdx,
            cdy,
            cdx * cdx + cdy * cdy,
        )
        .is_positive()
    }
}
