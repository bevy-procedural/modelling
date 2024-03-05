//! Bevy specific implementations for the vertex payload and 3d rotation.

use super::{Payload, Transform3D, Vector, Vector2D, Vector3D};
use bevy::{
    math::{Quat, Vec2, Vec3},
    transform::components::Transform,
};

impl Vector<f32> for Vec3 {
    type Vec2D = Vec2;
    type Vec3D = Vec3;

    #[inline(always)]
    fn zero() -> Self {
        Vec3::ZERO
    }

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
}

impl Vector3D<f32> for Vec3 {
    type Transform = Transform;

    #[inline(always)]
    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }
}

impl Vector<f32> for Vec2 {
    type Vec2D = Vec2;
    type Vec3D = Vec3;

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
}

impl Vector2D<f32> for Vec2 {
    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }
}

/// Vertex Payload for Bevy with 3d position, normal, and uv.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct BevyPayload {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl Payload for BevyPayload {
    type S = f32;
    type Vec = Vec3;

    #[inline(always)]
    fn translate(&self, v: &Self::Vec) -> Self {
        Self {
            position: self.position + *v,
            normal: self.normal,
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn vertex(&self) -> &Self::Vec {
        &self.position
    }

    #[inline(always)]
    fn from_vec(v: Self::Vec) -> Self {
        Self {
            position: v,
            normal: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }
}

impl Transform3D for Transform {
    type S = f32;
    type Vec = Vec3;

    #[inline(always)]
    fn identity() -> Self {
        Transform::default()
    }

    #[inline(always)]
    fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        Transform::from_rotation(Quat::from_axis_angle(axis, angle))
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        assert!((from.length() - 1.0).abs() < 0.01);
        assert!((to.length() - 1.0).abs() < 0.01);
        Transform::from_rotation(Quat::from_rotation_arc(from, to))
    }

    #[inline(always)]
    fn apply(&self, v: Vec3) -> Vec3 {
        if v.x.is_nan() || v.y.is_nan() || v.z.is_nan() {
            panic!("NAN in vertex: {:?}", v);
        }
        let v2 = self.transform_point(v);
        if v2.x.is_nan() {
            panic!("NAN in transformed vertex: {:?} {:?} {:?}", v, self, v2);
        }
        v2
    }
}
