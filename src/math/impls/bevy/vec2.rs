use crate::math::{Vector, Vector2D};
use bevy::math::{Affine2, Vec2, Vec3};

impl Vector<f32> for Vec2 {
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Trans = Affine2;

    #[inline(always)]
    fn zero() -> Self {
        Vec2::ZERO
    }

    #[inline(always)]
    fn dimensions() -> usize {
        2
    }

    #[inline(always)]
    fn distance(&self, other: &Self) -> f32 {
        Vec2::distance(*self, *other)
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec2::distance_squared(*self, *other)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec2::dot(*self, *other)
    }

    #[inline(always)]
    fn cross(&self, other: &Self) -> Self {
        Vec2::new(self.x() * other.y() - self.y() * other.x(), 0.0)
    }

    #[inline(always)]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    fn z(&self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn w(&self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn normalize(&self) -> Self {
        Vec2::normalize(*self)
    }

    #[inline(always)]
    fn splat(value: f32) -> Self {
        Vec2::splat(value)
    }
}

impl Vector2D for Vec2 {
    type S = f32;
    
    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }

    /// Magnitude of the vector.
    fn magnitude(&self) -> f32 {
        Vec2::length(*self)
    }

    /// Angle between two vectors.
    fn angle(&self, a: Self, b: Self) -> f32 {
        Vec2::angle_between(a - *self, b - *self)
    }
}
