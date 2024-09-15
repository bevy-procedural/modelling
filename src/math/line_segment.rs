//! Line segments in 2d space.

use super::{Scalar, Vector2D};

/// Trait for line segments in 2d space.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment2D<Vec2: Vector2D> {
    start: Vec2,
    end: Vec2,
}

impl<Vec2: Vector2D> LineSegment2D<Vec2> {
    /// Creates a new line segment from two points.
    #[inline(always)]
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    /// Returns the start point of the line segment.
    #[inline(always)]
    pub fn start(&self) -> Vec2 {
        self.start
    }

    /// Returns the end point of the line segment.
    #[inline(always)]
    pub fn end(&self) -> Vec2 {
        self.end
    }

    /// Returns the length of the line segment.
    #[inline(always)]
    pub fn length(&self) -> Vec2::S {
        self.start().distance(&self.end())
    }

    /// Returns the squared length of the line segment.
    #[inline(always)]
    pub fn length_squared(&self) -> Vec2::S {
        self.start().distance_squared(&self.end())
    }

    /// Returns the midpoint of the line segment.
    #[inline(always)]
    pub fn midpoint(&self) -> Vec2 {
        self.start() + (self.end() - self.start()) * Vec2::S::HALF
    }

    /// Returns the direction of the line segment.
    #[inline(always)]
    pub fn direction(&self) -> Vec2 {
        (self.end() - self.start()).normalize()
    }

    /// Returns the intersection point of two line segments.
    /// `eps` is the epsilon for the cross product, i.e., for whether the lines are considered parallel.
    /// `eps2` is the epsilon for the t and u values, i.e., for the line length.
    pub fn intersect_line(&self, other: &Self, eps: Vec2::S, eps2: Vec2::S) -> Option<Vec2> {
        let r = self.end() - self.start();
        let s = other.end() - other.start();
        let rxs = r.cross2d(&s);
        if rxs.abs() <= eps {
            // Lines are parallel or coincident
            return None;
        }
        let q_p = other.start() - self.start();
        let t = q_p.cross2d(&s) / rxs;
        let u = q_p.cross2d(&r) / rxs;
        if t >= -eps2 && t <= eps2 + 1.0.into() && u >= -eps2 && u <= eps2 + 1.0.into() {
            Some(self.start() + r * t)
        } else {
            None
        }
    }
}
