//! Bevy specific implementations for the vertex payload and 3d rotation.

use super::{HasNormal, HasPosition, Transformable, VertexPayload};
use crate::math::Transform;
use bevy::math::{Quat, Vec2, Vec3};

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
    fn allocate() -> Self {
        Self {
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            // TODO: Zero doesn't indicate invalid uv coordinates.
            uv: Vec2::ZERO,
        }
    }
}

impl Transformable for BevyVertexPayload {
    type S = f32;
    type Vec = Vec3;
    type Trans = bevy::transform::components::Transform;
    type Rot = Quat;

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
    fn rotate(&self, r: &Self::Rot) -> Self {
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
    fn lerp(&self, other: &Self, t: Self::S) -> Self {
        Self {
            position: self.position.lerp(other.position, t),
            // TODO: or reset to zero?
            normal: self.normal.lerp(other.normal, t),
            uv: self.uv.lerp(other.uv, t),
        }
    }
}

impl HasPosition<Vec3> for BevyVertexPayload {
    type S = f32;

    #[inline(always)]
    fn from_pos(v: Vec3) -> Self {
        Self {
            position: v,
            normal: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }
    
    #[inline(always)]
    fn pos(&self) -> &Vec3 {
        &self.position
    }

    #[inline(always)]
    fn set_pos(&mut self, v: Vec3) {
        self.position = v;
    }
}

impl HasNormal<Vec3> for BevyVertexPayload {
    type S = f32;

    #[inline(always)]
    fn normal(&self) -> &Vec3 {
        &self.normal
    }

    #[inline(always)]
    fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
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
