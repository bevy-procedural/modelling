use crate::{
    extensions::nalgebra::{NdRotate, ScalarPlus, VecN},
    math::{HasNormal, HasPosition, HasUV, Scalar, TransformTrait, Transformable},
    mesh::VertexPayload,
};

use super::NdHomography;

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
    type Trans = NdHomography<S, D>;
    type Rot = NdRotate<S, D>;

    #[inline]
    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        self.position += *v;
        // TODO: should the uv be translated as well?
        self
    }

    #[inline]
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        self.position = t.apply_point(self.position);
        self.normal = t.apply_vec(self.normal);
        // TODO: should the uv be transformed as well?
        self
    }

    #[inline]
    fn lerp(&mut self, _other: &Self, t: Self::S) -> &mut Self {
        self.position = self.position.lerp(&_other.position, t);
        self.normal = self.normal.lerp(&_other.normal, t);
        self.uv = self.uv.lerp(&_other.uv, t);
        self
    }
}

impl<S: Scalar, const D: usize> HasPosition<D, VecN<S, D>> for VertexPayloadPNU<S, D> {
    type S = S;

    #[inline]
    fn from_pos(v: VecN<S, D>) -> Self {
        Self {
            position: v,
            normal: VecN::zeros(),
            uv: VecN::zeros(), // TODO: how to indicate that the uv is not defined?
        }
    }

    #[inline]
    fn pos(&self) -> &VecN<S, D> {
        &self.position
    }

    #[inline]
    fn set_pos(&mut self, v: VecN<S, D>) {
        self.position = v;
    }
}

impl<S: Scalar, const D: usize> std::fmt::Debug for VertexPayloadPNU<S, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexPayloadPNU")
            .field("p", &self.position)
            .field("n", &self.normal)
            .field("uv", &self.uv)
            .finish()
    }
}

impl<S: Scalar, const D: usize> HasNormal<D, VecN<S, D>> for VertexPayloadPNU<S, D> {
    type S = S;

    #[inline]
    fn normal(&self) -> &VecN<S, D> {
        &self.normal
    }

    #[inline]
    fn set_normal(&mut self, normal: VecN<S, D>) {
        self.normal = normal;
    }
}

impl<S: Scalar, const D: usize> HasUV<VecN<S, 2>> for VertexPayloadPNU<S, D> {
    type S = S;

    #[inline]
    fn uv(&self) -> &VecN<S, 2> {
        &self.uv
    }

    #[inline]
    fn set_uv(&mut self, uv: VecN<S, 2>) {
        self.uv = uv;
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_vertex_payload_pnu() {
        let mut vp = VertexPayloadPNU::<f64, 3>::from_pos(Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(
            format!("{:?}", vp),
            "VertexPayloadPNU { p: [[1.0, 2.0, 3.0]], n: [[0.0, 0.0, 0.0]], uv: [[0.0, 0.0]] }"
        );

        assert_eq!(vp.pos(), &Vec3::new(1.0, 2.0, 3.0));
        vp.set_pos(Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(vp.pos(), &Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(vp.normal(), &Vec3::zeros());
        vp.set_normal(Vec3::new(7.0, 8.0, 9.0));
        assert_eq!(vp.normal(), &Vec3::new(7.0, 8.0, 9.0));

        vp.translate(&Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(vp.pos(), &Vec3::new(5.0, 7.0, 9.0));
        vp.set_pos(Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(vp.pos(), &Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(vp.normal(), &Vec3::new(7.0, 8.0, 9.0));

        vp.rotate(&NdRotate::from_axis_angle(Vec3::x_axis(), f64::PI));
        assert!(vp.pos().is_about(&Vec3::new(4.0, -5.0, -6.0), 1e-6));
        // TODO: assert!(vp.normal().is_about(&Vec3::new(7.0, -8.0, -9.0), 1e-6));

        let vp2 = VertexPayloadPNU::<f64, 3>::from_pos(Vec3::new(0.0, 0.0, 0.0));
        let vp3 = vp2.lerped(&vp, 0.1);
        assert!(vp3.pos().is_about(&Vec3::new(0.4, -0.5, -0.6), 1e-6));
        // TODO: assert!(vp3.normal().is_about(&Vec3::new(0.7, -0.8, -0.9), 1e-6));
    }
}
