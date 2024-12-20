use bevy::math::{Quat, Vec2, Vec3};

use crate::{
    math::{HasNormal, HasPosition, HasUV, TransformTrait, Transformable},
    mesh::VertexPayload,
};

/// Vertex Payload for Bevy with 3d position, normal, and uv.
#[derive(Clone, PartialEq, Default, Copy)]
pub struct BevyVertexPayload3d {
    /// The position of the vertex.
    position: Vec3,

    /// The normal of the vertex.
    normal: Vec3,

    /// The uv coordinates of the vertex.
    uv: Vec2,
}

impl VertexPayload for BevyVertexPayload3d {
    fn allocate() -> Self {
        Self {
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            // TODO: Zero doesn't indicate invalid uv coordinates.
            uv: Vec2::ZERO,
        }
    }
}

impl Transformable<3> for BevyVertexPayload3d {
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

impl HasPosition<3, Vec3> for BevyVertexPayload3d {
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

impl HasNormal<3, Vec3> for BevyVertexPayload3d {
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

impl HasUV<Vec2> for BevyVertexPayload3d {
    type S = f32;

    #[inline(always)]
    fn uv(&self) -> &Vec2 {
        &self.uv
    }

    #[inline(always)]
    fn set_uv(&mut self, uv: Vec2) {
        self.uv = uv;
    }
}

impl std::fmt::Debug for BevyVertexPayload3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:+05.3}, {:+05.3}, {:+05.3}",
            self.position.x, self.position.y, self.position.z,
        )
    }
}

#[cfg(feature = "nalgebra")]
impl<S: crate::math::Scalar> From<&crate::extensions::nalgebra::VertexPayloadPNU<S, 3>>
    for BevyVertexPayload3d
{
    fn from(value: &crate::extensions::nalgebra::VertexPayloadPNU<S, 3>) -> Self {
        Self {
            position: Vec3::new(
                value.pos().x.as_f64() as f32,
                value.pos().y.as_f64() as f32,
                value.pos().z.as_f64() as f32,
            ),
            normal: Vec3::new(
                value.normal().x.as_f64() as f32,
                value.normal().y.as_f64() as f32,
                value.normal().z.as_f64() as f32,
            ),
            uv: Vec2::new(value.uv().x.as_f64() as f32, value.uv().y.as_f64() as f32),
        }
    }
}
