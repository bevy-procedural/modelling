//! Bevy specific implementations for the vertex payload and 3d rotation.

use super::VertexPayload;
use crate::math::Transform;
use bevy::math::{Quat, Vec2, Vec3, Vec4};

/// Vertex Payload for Bevy with 3d position, normal, and uv.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct BevyVertexPayload {
    /// The position of the vertex.
    position: Vec3,

    /// The normal of the vertex.
    normal: Vec3,

    /// The uv coordinates of the vertex.
    uv: Vec2,
}

impl VertexPayload for BevyVertexPayload {
    type S = f32;
    type Vec = Vec3;
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Vec4 = Vec4;
    type Trans = bevy::transform::components::Transform;
    type Quat = Quat;

    #[inline(always)]
    fn translate(&self, v: &Self::Vec) -> Self {
        Self {
            position: self.position + *v,
            normal: self.normal,
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn transform(&self, t: &Self::Trans) -> Self {
        Self {
            position: t.apply(self.position),
            normal: t.apply_vec(self.normal),
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn rotate(&self, r: &Self::Quat) -> Self {
        Self {
            position: r.mul_vec3(self.position),
            normal: r.mul_vec3(self.normal),
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn scale(&self, s: &Self::Vec) -> Self {
        Self {
            position: self.position * *s,
            normal: self.normal,
            uv: self.uv,
        }
    }

    #[inline(always)]
    fn pos(&self) -> &Self::Vec {
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
    fn has_normal(&self) -> bool {
        self.normal != Vec3::ZERO
    }

    #[inline(always)]
    fn from_pos(v: Self::Vec) -> Self {
        Self {
            position: v,
            normal: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }

    #[inline(always)]
    fn set_pos(&mut self, v: Self::Vec) {
        self.position = v;
    }
}

impl std::fmt::Display for BevyVertexPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:+05.3}, {:+05.3}, {:+05.3}",
            self.position.x, self.position.y, self.position.z,
        )
    }
}
