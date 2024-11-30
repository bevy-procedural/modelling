//! Bevy specific implementations for the vertex payload and 3d rotation.

use bevy::math::{Quat, Vec2, Vec3};

use crate::{
    math::{HasNormal, HasPosition, TransformTrait, Transformable},
    mesh::VertexPayload,
};

/// Vertex Payload for Bevy with 3d position, normal, and uv.
#[derive(Clone, PartialEq, Default, Copy)]
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
    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        self.position += *v;
        // TODO: should the uv be translated as well?
        self
    }

    #[inline(always)]
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        self.position = t.apply(self.position);
        self.normal = t.apply_vec(self.normal);
        // TODO: should the uv be transformed as well?
        self
    }

    #[inline(always)]
    fn rotate(&mut self, r: &Self::Rot) -> &mut Self {
        self.position = r.mul_vec3(self.position);
        self.normal = r.mul_vec3(self.normal);
        // TODO: should the uv be transformed as well?
        self
    }

    #[inline(always)]
    fn scale(&mut self, s: &Self::Vec) -> &mut Self {
        self.position *= *s;
        self
    }

    #[inline(always)]
    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        self.position = self.position.lerp(other.position, t);
        // TODO: or reset to zero?
        self.normal = self.normal.lerp(other.normal, t);
        self.uv = self.uv.lerp(other.uv, t);
        self
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

impl std::fmt::Debug for BevyVertexPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:+05.3}, {:+05.3}, {:+05.3}",
            self.position.x, self.position.y, self.position.z,
        )
    }
}
