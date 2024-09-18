use super::mat5::Mat5;
use crate::math::{HasZero, Scalar, Vector, Vector4D};
use bevy::math::{Vec2, Vec3, Vec4};

impl HasZero for Vec4 {
    const ZERO: Self = Vec4::ZERO;
}

impl Vector<f32> for Vec4 {
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Vec4 = Vec4;
    type Trans = Mat5<f32>;

    #[inline(always)]
    fn dimensions() -> usize {
        3
    }

    #[inline(always)]
    fn distance(&self, other: &Self) -> f32 {
        Vec4::distance(*self, *other)
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec4::distance_squared(*self, *other)
    }

    #[inline(always)]
    fn length(&self) -> f32 {
        Vec4::length(*self)
    }

    #[inline(always)]
    fn length_squared(&self) -> f32 {
        Vec4::length_squared(*self)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec4::dot(*self, *other)
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
        self.w
    }

    #[inline(always)]
    fn normalize(&self) -> Self {
        Vec4::normalize(*self)
    }

    #[inline(always)]
    fn splat(value: f32) -> Self {
        Vec4::splat(value)
    }

    #[inline(always)]
    fn from_x(x: f32) -> Self {
        Vec4::new(x, 0.0, 0.0, 0.0)
    }

    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec4::new(x, y, 0.0, 0.0)
    }

    #[inline(always)]
    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec4::new(x, y, z, 0.0)
    }

    #[inline(always)]
    fn is_about(&self, other: &Self, epsilon: f32) -> bool {
        self.x.is_about(other.x, epsilon)
            && self.y.is_about(other.y, epsilon)
            && self.z.is_about(other.z, epsilon)
            && self.w.is_about(other.w, epsilon)
    }
}

impl Vector4D for Vec4 {
    type S = f32;

    #[inline(always)]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4::new(x, y, z, w)
    }
}
