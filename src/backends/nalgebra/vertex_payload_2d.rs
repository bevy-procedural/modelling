use bevy::math::Vec2;

use crate::{
    math::{HasPosition, TransformTrait, Transformable},
    mesh::VertexPayload,
};

/// Vertex Payload for Bevy with 2d position, and uv.
#[derive(Clone, PartialEq, Default, Copy)]
pub struct VertexPayload2d {
    /// The position of the vertex.
    position: Vec2,

    /// The uv coordinates of the vertex.
    uv: Vec2,
}

impl VertexPayload for VertexPayload2d {
    fn allocate() -> Self {
        Self {
            position: Vec2::ZERO,
            // TODO: Zero doesn't indicate invalid uv coordinates.
            uv: Vec2::ZERO,
        }
    }
}

impl Transformable<2> for VertexPayload2d {
    type S = f32;
    type Vec = Vec2;
    type Trans = bevy::math::Affine2;
    type Rot = f32;

    #[inline(always)]
    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        self.position += *v;
        // TODO: should the uv be translated as well?
        self
    }

    #[inline(always)]
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        self.position = t.apply(self.position);
        // TODO: should the uv be transformed as well?
        self
    }

    #[inline(always)]
    fn rotate(&mut self, _r: &Self::Rot) -> &mut Self {
        todo!("rotate")
    }

    #[inline(always)]
    fn scale(&mut self, s: &Self::Vec) -> &mut Self {
        self.position *= *s;
        self
    }

    #[inline(always)]
    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        self.position = self.position.lerp(other.position, t);
        self.uv = self.uv.lerp(other.uv, t);
        self
    }
}

impl HasPosition<2, Vec2> for VertexPayload2d {
    type S = f32;

    #[inline(always)]
    fn from_pos(v: Vec2) -> Self {
        Self {
            position: v,
            uv: Vec2::ZERO,
        }
    }

    #[inline(always)]
    fn pos(&self) -> &Vec2 {
        &self.position
    }

    #[inline(always)]
    fn set_pos(&mut self, v: Vec2) {
        self.position = v;
    }
}

impl std::fmt::Debug for VertexPayload2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+05.3}, {:+05.3}", self.position.x, self.position.y,)
    }
}
