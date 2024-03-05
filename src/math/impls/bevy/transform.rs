use bevy::math::{Affine2, Quat, Vec2, Vec3};

use crate::math::Transform;


// TODO: Switch to Affine3
impl Transform for bevy::transform::components::Transform {
    type S = f32;
    type Vec = Vec3;

    #[inline(always)]
    fn identity() -> Self {
        bevy::transform::components::Transform::default()
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        assert!((from.length() - 1.0).abs() < 0.01);
        assert!((to.length() - 1.0).abs() < 0.01);
        bevy::transform::components::Transform::from_rotation(Quat::from_rotation_arc(from, to))
    }

    #[inline(always)]
    fn from_translation(v: Vec3) -> Self {
        bevy::transform::components::Transform::from_translation(v)
    }

    #[inline(always)]
    fn from_scale(v: Vec3) -> Self {
        bevy::transform::components::Transform::from_scale(v)
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

impl Transform for Affine2 {
    type S = f32;
    type Vec = Vec2;

    #[inline(always)]
    fn identity() -> Self {
        Affine2::IDENTITY
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec2, to: Vec2) -> Self {
        bevy::math::Affine2::from_angle(from.angle_between(to))
    }

    #[inline(always)]
    fn from_translation(v: Vec2) -> Self {
        bevy::math::Affine2::from_translation(v)
    }

    #[inline(always)]
    fn from_scale(v: Vec2) -> Self {
        bevy::math::Affine2::from_scale(v)
    }

    #[inline(always)]
    fn apply(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_point2(self, v)
    }

    #[inline(always)]
    fn apply_vec(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_vector2(self, v)
    }
}
