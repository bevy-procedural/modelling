use super::mat5::Mat5;
use crate::math::{HasZero, Quarternion, Vector, Vector4D};
use bevy::math::{Quat, Vec2, Vec3, Vec4};

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
}

impl Vector4D for Vec4 {
    type S = f32;

    #[inline(always)]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4::new(x, y, z, w)
    }
}

impl Quarternion for Quat {
    type S = f32;
    type Vec3 = Vec3;
    type Vec4 = Vec4;

    #[inline(always)]
    fn identity() -> Self {
        Quat::IDENTITY
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        //assert!((from.length() - 1.0).abs() < 0.01);
        //assert!((to.length() - 1.0).abs() < 0.01);
        Quat::from_rotation_arc(from, to)
    }

    #[inline(always)]
    fn from_axis_angle(axis: Self::Vec3, angle: Self::S) -> Self {
        Quat::from_axis_angle(axis, angle)
    }

    #[inline(always)]
    fn axis_angle(&self) -> (Self::Vec3, Self::S) {
        self.to_axis_angle()
    }

    #[inline(always)]
    fn vec4(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}
