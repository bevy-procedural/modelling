use crate::math::Polygon;
use bevy::math::Vec2;

/// A polygon in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2dBevy {
    vertices: Vec<Vec2>,
}

impl Polygon<Vec2> for Polygon2dBevy {
    fn from_points(points: &[Vec2]) -> Self {
        Self {
            vertices: points.to_vec(),
        }
    }

    fn points(&self) -> &[Vec2] {
        &self.vertices
    }

    fn num_points(&self) -> usize {
        self.vertices.len()
    }

    fn append_point(&mut self, point: Vec2) {
        self.vertices.push(point);
    }
}
