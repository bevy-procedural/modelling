use super::{LineSegment2D, Scalar, ScalarIteratorExt, Vector2D, VectorIteratorExt};

/// Trait for a polygon in n-dimensional space.
///
/// It should be able to handle degenerate and self-overlapping polygons and also
/// empty polygons or those with only 1 or 2 vertices.
pub trait Polygon<Vec2: Vector2D>: Clone + PartialEq + std::fmt::Debug + 'static {
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

    /// Returns the number of vertices of the polygon.
    fn num_points(&self) -> usize;

    /// Returns the signed area of the polygon.    
    fn signed_area(&self) -> Vec2::S {
        // PERF: This should directly run on a point iterator.
        let points = self.points();
        Vec2::S::HALF
            * (0..points.len())
                .into_iter()
                .map(|i| {
                    let j = (i + 1) % points.len();
                    points[i].x() * points[j].y() - points[j].x() * points[i].y()
                })
                .stable_sum()
    }

    /// Returns the area of the polygon.
    fn area(&self) -> Vec2::S {
        self.signed_area().abs()
    }

    /// Returns whether the polygon is counter-clockwise oriented or zero.
    fn is_ccw(&self) -> bool {
        self.signed_area() >= Vec2::S::ZERO
    }

    /// Returns whether the polygon is clockwise oriented or zero.
    fn is_cw(&self) -> bool {
        self.signed_area() <= Vec2::S::ZERO
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
        if j + 1 == i
            || i + 1 == j
            || (i == 0 && j == self.num_points() - 1)
            || (j == 0 && i == self.num_points() - 1)
        {
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

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use super::*;
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_polygon2d() {
        for (points, area) in [
            (vec![], 0.0),
            (vec![Vec2::new(0.0, 0.0)], 0.0),
            (vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)], 0.0),
            (
                vec![
                    Vec2::new(0.0, 0.0),
                    Vec2::new(1.0, 0.0),
                    Vec2::new(0.0, 0.0),
                ],
                0.0,
            ),
            (
                vec![
                    Vec2::new(0.0, 0.0),
                    Vec2::new(1.0, 1.0),
                    Vec2::new(0.0, 1.0),
                ],
                0.5,
            ),
            (
                vec![
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 1.0),
                    Vec2::new(1.0, 1.0),
                ],
                -0.5,
            ),
            (
                vec![
                    Vec2::new(0.0, 0.0),
                    Vec2::new(1.0, 0.0),
                    Vec2::new(1.0, 1.0),
                    Vec2::new(0.0, 1.0),
                ],
                1.0,
            ),
            // TODO: Shouldn't be negative
            /*(
                Mesh2d64::regular_polygon(1.0, 100)
                    .vertices()
                    .map(|v| v.pos())
                    .collect(),
                -3.1395259784676552,
            ),*/
        ] {
            let polygon = Polygon2d::from_points(&points);
            assert_eq!(polygon.num_points(), points.len());
            assert_eq!(polygon.points(), points.as_slice());
            assert!(polygon.signed_area().is_about(area, 1e-10));
            assert!(polygon.area().is_about(area.abs(), 1e-10));

            if area != 0.0 {
                for i in 0..points.len() {
                    for j in 0..points.len() {
                        assert_eq!(polygon.valid_diagonal(i, j), i != j);
                    }
                }

                assert_eq!(polygon.is_ccw(), area > 0.0);
                assert_eq!(polygon.is_cw(), area < 0.0);
                assert!(polygon.contains(&Vec2::new(0.5, 0.51)));

                let centroid = polygon.centroid();
                assert!(polygon.contains(&centroid));

                // undefined on the boundary, but moving an epsilon inside makes the test pass
                let eps = 1e-10;
                for p in points {
                    let inside = p.lerp(&centroid, eps);
                    assert!(polygon.contains(&inside));

                    let outside = p.lerp(&centroid, -eps);
                    assert!(!polygon.contains(&outside));
                }
            } else {
                // TODO: test the degenerate points
            }
        }
    }

    #[test]
    fn test_concave_polygon2d() {
        // TODO: also test self intersecting polygons and nasty concave ones
    }
}
