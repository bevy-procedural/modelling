use super::mat5::{Mat5, Vec4Rotator};
use crate::math::{HasZero, Scalar, TransformTrait, Transformable, Vector, Vector4D};
use bevy::math::Vec4;

impl HasZero for Vec4 {
    #[inline]
    fn zero() -> Self {
        Vec4::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Vec4::ZERO
    }
}

impl Vector<f32, 4> for Vec4 {
    #[inline]
    fn distance(&self, other: &Self) -> f32 {
        Vec4::distance(*self, *other)
    }

    #[inline]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec4::distance_squared(*self, *other)
    }

    #[inline]
    fn length(&self) -> f32 {
        Vec4::length(*self)
    }

    #[inline]
    fn length_squared(&self) -> f32 {
        Vec4::length_squared(*self)
    }

    #[inline]
    fn dot(&self, other: &Self) -> f32 {
        Vec4::dot(*self, *other)
    }

    #[inline]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    fn z(&self) -> f32 {
        self.z
    }

    #[inline]
    fn w(&self) -> f32 {
        self.w
    }

    #[inline]
    fn normalize(&self) -> Self {
        Vec4::normalize(*self)
    }

    #[inline]
    fn splat(value: f32) -> Self {
        Vec4::splat(value)
    }

    #[inline]
    fn from_x(x: f32) -> Self {
        Vec4::new(x, 0.0, 0.0, 0.0)
    }

    #[inline]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec4::new(x, y, 0.0, 0.0)
    }

    #[inline]
    fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec4::new(x, y, z, 0.0)
    }

    #[inline]
    fn is_about(&self, other: &Self, epsilon: f32) -> bool {
        self.x.is_about(other.x, epsilon)
            && self.y.is_about(other.y, epsilon)
            && self.z.is_about(other.z, epsilon)
            && self.w.is_about(other.w, epsilon)
    }
}

impl Vector4D for Vec4 {
    type S = f32;

    #[inline]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4::new(x, y, z, w)
    }
}

// TODO: implement more methods to improve performance
impl Transformable<4> for Vec4 {
    type Rot = Vec4Rotator;
    type S = f32;
    type Trans = Mat5<f32>;
    type Vec = Vec4;

    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        *self = t.apply_point(*self);
        self
    }

    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        *self = Vec4::lerp(*self, *other, t);
        self
    }
}
