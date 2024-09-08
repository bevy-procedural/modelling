use crate::math::Polygon;
use crate::math::Scalar;
use bevy::math::f32;
use bevy::math::Vec2;

/// A polygon in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Bevy2DPolygon {
    vertices: Vec<Vec2>,
}

impl Polygon<Vec2> for Bevy2DPolygon {
    type S = f32;

    fn from_points(points: &[Vec2]) -> Self {
        Self {
            vertices: points.to_vec(),
        }
    }

    fn points(&self) -> &[Vec2] {
        &self.vertices
    }

    fn signed_area(&self) -> f32 {
        0.5 * f32::sum((0..self.vertices.len()).into_iter().map(|i| {
            let j = (i + 1) % self.vertices.len();
            self.vertices[i].x * self.vertices[j].y - self.vertices[j].x * self.vertices[i].y
        }))
    }
}
