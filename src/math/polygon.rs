use super::{LineSegment2D, Scalar, Vector2D, VectorIteratorExt};

/// Trait for a polygon in n-dimensional space.
pub trait Polygon<Vec2: Vector2D>: Clone + PartialEq + std::fmt::Debug + 'static {
    /// The scalar type of the polygon.
    type S: Scalar;

    /// Returns a polygon from a list of points.
    fn from_points(points: &[Vec2]) -> Self;

    /// Returns a polygon from an iterator of points.
    fn from_iter(iter: impl IntoIterator<Item = Vec2>) -> Self {
        Self::from_points(&iter.into_iter().collect::<Vec<_>>())
    }

    /// Returns the average of the points of the polygon.
    fn centroid(&self) -> Vec2 {
        VectorIteratorExt::stable_mean(self.points().iter().copied())
    }

    /// Returns the points of the polygon.
    fn points(&self) -> &[Vec2];

    /// Returns the signed area of the polygon.
    fn signed_area(&self) -> Self::S;

    /// Returns the area of the polygon.
    fn area(&self) -> Self::S {
        self.signed_area().abs()
    }

    /// Returns whether the polygon is counter-clockwise oriented or zero.
    fn is_ccw(&self) -> bool {
        self.signed_area() >= Self::S::ZERO
    }

    /// Returns whether the polygon is clockwise oriented or zero.
    fn is_cw(&self) -> bool {
        self.signed_area() >= Self::S::ZERO
    }

    /// Whether a point is inside the polygon
    fn contains(&self, point: &Vec2) -> bool {
        let mut count = 0;
        let points = self.points();
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            if a.y() <= point.y() {
                if b.y() > point.y() && (b - a).perp_dot(&(*point - a)).is_positive() {
                    count += 1;
                }
            } else if b.y() <= point.y() && (b - a).perp_dot(&(*point - a)).is_negative() {
                count -= 1;
            }
        }
        count != 0
    }

    /// Whether an edge is a valid diagonal.
    /// If (i,j) is an edge on the boundary, consider it also a valid diagonal,
    /// but i==j is considered invalid.
    fn valid_diagonal(&self, i: usize, j: usize) -> bool {
        if i == j {
            return false;
        }

        // not a diagonal, but definitely valid
        if j + 1 == i || i + 1 == j {
            return true;
        }

        let ps = self.points();
        let n = ps.len();

        // It is also possible for the diagonal to be fully outside the polygon where it is concave.
        // We can test whether the midpoint of the diagonal is inside the polygon.
        let mid = (ps[i] + ps[j]) * Vec2::S::HALF;
        if !self.contains(&mid) {
            return false;
        }

        // Is there a boundary edge intersecting the diagonal?
        for start in 0..n {
            let end = (start + 1) % n;
            // ignore edges starting or ending at i or j
            if start == i || start == j || end == i || end == j {
                continue;
            }

            if LineSegment2D::new(ps[i], ps[j])
                .intersect_line(
                    &LineSegment2D::new(ps[start], ps[end]),
                    Vec2::S::EPS,
                    Vec2::S::EPS,
                )
                .is_some()
            {
                return false;
            }
        }

        true
    }
}
