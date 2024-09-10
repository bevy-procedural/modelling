use crate::math::{HasZero, Vector, Vector3D};
use bevy::math::{Vec2, Vec3};

impl HasZero for Vec3 {
    const ZERO: Self = Vec3::ZERO;
}

impl Vector<f32> for Vec3 {
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Trans = bevy::transform::components::Transform;

    #[inline(always)]
    fn dimensions() -> usize {
        3
    }

    #[inline(always)]
    fn distance(&self, other: &Self) -> f32 {
        Vec3::distance(*self, *other)
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec3::distance_squared(*self, *other)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec3::dot(*self, *other)
    }

    #[inline(always)]
    fn cross(&self, other: &Self) -> Self {
        Vec3::cross(*self, *other)
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
        self.z
    }

    #[inline(always)]
    fn w(&self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn normalize(&self) -> Self {
        Vec3::normalize(*self)
    }

    #[inline(always)]
    fn splat(value: f32) -> Self {
        Vec3::splat(value)
    }

    #[inline(always)]
    fn from_x(x: f32) -> Self {
        Vec3::new(x, 0.0, 0.0)
    }

    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec3::new(x, y, 0.0)
    }

    #[inline(always)]
    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }

}

impl Vector3D for Vec3 {
    type S = f32;

    #[inline(always)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }
}
