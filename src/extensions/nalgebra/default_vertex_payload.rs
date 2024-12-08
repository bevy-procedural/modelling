use crate::{
    extensions::nalgebra::{NdAffine, NdRotate, ScalarPlus, VecN},
    math::{HasNormal, HasPosition, Scalar, TransformTrait, Transformable},
    mesh::VertexPayload,
};

/// d-dimensional Vertex Payload with position, normal, and uv coordinates.
#[derive(Clone, PartialEq, Copy)]
pub struct VertexPayloadPNU<S: Scalar, const D: usize> {
    /// The position of the vertex.
    position: VecN<S, D>,

    /// The normal of the vertex.
    normal: VecN<S, D>,

    /// The uv coordinates of the vertex.
    uv: VecN<S, 2>,
}

impl<S: Scalar, const D: usize> VertexPayload for VertexPayloadPNU<S, D> {
    fn allocate() -> Self {
        Self {
            position: VecN::zeros(),
            normal: VecN::zeros(),
            uv: VecN::zeros(), // TODO: how to indicate that the uv is not defined?
        }
    }
}

impl<S: ScalarPlus, const D: usize> Transformable<D> for VertexPayloadPNU<S, D> {
    type S = S;
    type Vec = VecN<S, D>;
    type Trans = NdAffine<S, D>;
    type Rot = NdRotate<S, D>;

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
    fn lerp(&mut self, _other: &Self, _t: Self::S) -> &mut Self {
        todo!("lerp")
    }
}

impl<S: Scalar, const D: usize> HasPosition<D, VecN<S, D>> for VertexPayloadPNU<S, D> {
    type S = S;

    #[inline(always)]
    fn from_pos(v: VecN<S, D>) -> Self {
        Self {
            position: v,
            normal: VecN::zeros(),
            uv: VecN::zeros(), // TODO: how to indicate that the uv is not defined?
        }
    }

    #[inline(always)]
    fn pos(&self) -> &VecN<S, D> {
        &self.position
    }

    #[inline(always)]
    fn set_pos(&mut self, v: VecN<S, D>) {
        self.position = v;
    }
}

impl<S: Scalar, const D: usize> std::fmt::Debug for VertexPayloadPNU<S, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexPayloadPosNorUV")
            .field("position", &self.position)
            .field("normal", &self.normal)
            .field("uv", &self.uv)
            .finish()
    }
}

impl<S: Scalar, const D: usize> HasNormal<D, VecN<S, D>> for VertexPayloadPNU<S, D> {
    type S = S;

    #[inline(always)]
    fn normal(&self) -> &VecN<S, D> {
        &self.normal
    }

    #[inline(always)]
    fn set_normal(&mut self, normal: VecN<S, D>) {
        self.normal = normal;
    }
}
