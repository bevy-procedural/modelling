use super::{ScalarPlus, Vec2};
use crate::math::{Polygon, Scalar};
use nalgebra::SVector;

/// A polygon in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon2d<S: Scalar> {
    vertices: Vec<SVector<S, 2>>,
}

impl<S: ScalarPlus> Polygon<Vec2<S>> for Polygon2d<S> {
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
