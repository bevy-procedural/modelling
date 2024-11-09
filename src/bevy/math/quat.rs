use crate::math::{Quarternion, Rotator};
use bevy::math::{Quat, Vec3, Vec4};

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

impl Rotator<Vec3> for Quat {}
