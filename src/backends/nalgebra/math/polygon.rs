use super::Vec2;
use crate::math::{Polygon, Scalar, ScalarIteratorExt};
use nalgebra::SVector;

/// A polygon in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2d<S: Scalar> {
    vertices: Vec<SVector<S, 2>>,
}

impl<S: Scalar> Polygon<Vec2<S>> for Polygon2d<S> {
    type S = S;

    fn from_points(points: &[Vec2<S>]) -> Self {
        Self {
            vertices: points.to_vec(),
        }
    }

    fn points(&self) -> &[Vec2<S>] {
        &self.vertices
    }

    fn signed_area(&self) -> S {
        S::HALF
            * (0..self.vertices.len())
                .into_iter()
                .map(|i| {
                    let j = (i + 1) % self.vertices.len();
                    self.vertices[i].x * self.vertices[j].y
                        - self.vertices[j].x * self.vertices[i].y
                })
                .stable_sum()
    }
}
