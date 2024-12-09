use super::Vec2;
use crate::math::{Polygon, Scalar};
use nalgebra::SVector;

/// A polygon in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2d<S: Scalar> {
    vertices: Vec<SVector<S, 2>>,
}

impl<S: Scalar> Polygon<Vec2<S>> for Polygon2d<S> {
    fn from_points(points: &[Vec2<S>]) -> Self {
        Self {
            vertices: points.to_vec(),
        }
    }

    fn points(&self) -> &[Vec2<S>] {
        &self.vertices
    }

    fn num_points(&self) -> usize {
        self.vertices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Polygon;

    #[test]
    fn test_polygon2d() {
        for points in [
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ],
            vec![],
            vec![Vec2::new(0.0, 0.0)],
        ] {
            let polygon = Polygon2d::from_points(&points);
            assert_eq!(polygon.num_points(), points.len());
            assert_eq!(polygon.points(), points.as_slice());
        }
    }
}
