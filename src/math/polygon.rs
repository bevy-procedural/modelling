use super::{Scalar, Vector};

/// Trait for a polygon in n-dimensional space.
pub trait Polygon<V: Vector<Self::S>>: Clone + PartialEq + std::fmt::Debug + 'static {
    /// The scalar type of the polygon.
    type S: Scalar;

    /// Returns a polygon from a list of points.
    fn from_points(points: &[V]) -> Self;

    /// Returns a polygon from an iterator of points.
    fn from_iter(iter: impl IntoIterator<Item = V>) -> Self {
        Self::from_points(&iter.into_iter().collect::<Vec<_>>())
    }

    /// Returns the points of the polygon.
    fn points(&self) -> &[V];

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
}
