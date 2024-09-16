use crate::math::{HasZero, Spherical3d, Transform, Vector, Vector3D};
use bevy::{
    math::{Quat, Vec2, Vec3, Vec4},
    transform::components::Transform as TransformBevy,
};

impl HasZero for Vec3 {
    const ZERO: Self = Vec3::ZERO;
}

impl Vector<f32> for Vec3 {
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Vec4 = Vec4;
    type Trans = TransformBevy;

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
    fn length(&self) -> f32 {
        Vec3::length(*self)
    }

    #[inline(always)]
    fn length_squared(&self) -> f32 {
        Vec3::length_squared(*self)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec3::dot(*self, *other)
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
    type Spherical = Vec3;

    #[inline(always)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }

    #[inline(always)]
    fn cross(&self, other: &Self) -> Self {
        Vec3::cross(*self, *other)
    }
}

impl Spherical3d for Vec3 {
    type S = f32;
}

// TODO: Switch to Affine3
impl Transform for TransformBevy {
    type S = f32;
    type Vec = Vec3;
    type Rot = Quat;

    #[inline(always)]
    fn identity() -> Self {
        TransformBevy::default()
    }

    #[inline(always)]
    fn from_rotation(q: Quat) -> Self {
        TransformBevy::from_rotation(q)
    }

    #[inline(always)]
    fn from_rotation_arc(from: Self::Vec, to: Self::Vec) -> Self {
        TransformBevy::from_rotation(Quat::from_rotation_arc(from, to))
    }

    #[inline(always)]
    fn from_translation(v: Vec3) -> Self {
        TransformBevy::from_translation(v)
    }

    #[inline(always)]
    fn from_scale(v: Vec3) -> Self {
        TransformBevy::from_scale(v)
    }

    #[inline(always)]
    fn with_scale(&self, scale: Self::Vec) -> Self {
        TransformBevy::with_scale(*self, scale)
    }

    #[inline(always)]
    fn with_translation(&self, v: Self::Vec) -> Self {
        TransformBevy::with_translation(*self, v)
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

    #[inline(always)]
    fn apply_vec(&self, v: Vec3) -> Vec3 {
        self.apply(v)
    }
}
