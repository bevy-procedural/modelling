//! Bevy specific implementations for the vertex payload and 3d rotation.

use super::Payload;
use crate::math::{Transform, Vector};
use bevy::math::{Vec2, Vec3};

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
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Trans = bevy::transform::components::Transform;

    #[inline(always)]
    fn translate(&self, v: &Self::Vec) -> Self {
        Self {
            position: self.position + *v,
            normal: self.normal,
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn transform(&self, t: &<Self::Vec as Vector<Self::S>>::Transform) -> Self {
        Self {
            position: t.apply(self.position),
            normal: t.apply_vec(self.normal),
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn vertex(&self) -> &Self::Vec {
        &self.position
    }

    #[inline(always)]
    fn normal(&self) -> &Self::Vec {
        &self.normal
    }

    #[inline(always)]
    fn set_normal(&mut self, normal: Self::Vec) {
        self.normal = normal;
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
