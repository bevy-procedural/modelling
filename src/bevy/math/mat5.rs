use bevy::math::Vec4;

use crate::math::{HasZero, Rotator, Scalar, TransformTrait, Vector4D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat5<S: Scalar> {
    data: [S; 25],
}

impl<S: Scalar> HasZero for Mat5<S> {
    const ZERO: Self = Mat5 {
        data: [S::ZERO; 25],
    };
}

impl<S: Scalar> std::ops::Mul<Mat5<S>> for Mat5<S> {
    type Output = Mat5<S>;

    fn mul(self, rhs: Mat5<S>) -> Mat5<S> {
        let mut m = Mat5::ZERO;
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    m.data[i * 5 + j] += self.data[i * 5 + k] * rhs.data[k * 5 + j];
                }
            }
        }
        m
    }
}

impl<S: Scalar> std::ops::Add<Mat5<S>> for Mat5<S> {
    type Output = Mat5<S>;

    fn add(self, rhs: Mat5<S>) -> Mat5<S> {
        let mut m = Mat5::ZERO;
        for i in 0..25 {
            m.data[i] = self.data[i] + rhs.data[i];
        }
        m
    }
}

impl<S: Scalar, Vec4: Vector4D<S = S>> std::ops::Mul<Vec4> for Mat5<S> {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        let rhs = [v.x(), v.y(), v.z(), v.w(), S::ONE];
        let mut res = [S::ZERO; 5];
        for i in 0..5 {
            for j in 0..5 {
                res[i] += self.data[i * 5 + j] * rhs[j];
            }
        }
        Vec4::new(res[0], res[1], res[2], res[3])
    }
}

impl<S: Scalar> Mat5<S> {
    const IDENTITY: Self = Mat5 {
        data: [
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO, //
            S::ZERO,
            S::ONE,
            S::ZERO,
            S::ZERO,
            S::ZERO, //
            S::ZERO,
            S::ZERO,
            S::ONE,
            S::ZERO,
            S::ZERO, //
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE,
            S::ZERO, //
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ZERO,
            S::ONE, //
        ],
    };
}

impl<S: Scalar> Default for Mat5<S> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

pub struct Vec4Rotator {}

impl Rotator<Vec4> for Vec4Rotator {}

impl TransformTrait for Mat5<f32> {
    type Vec = Vec4;
    type S = f32;
    type Rot = Vec4Rotator;

    fn identity() -> Self {
        Mat5::IDENTITY
    }

    fn from_rotation(_: Self::Rot) -> Self {
        todo!("Not implemented");
    }

    fn from_rotation_arc(_from: Self::Vec, _to: Self::Vec) -> Self {
        todo!("Not implemented");
    }

    fn from_translation(v: Self::Vec) -> Self {
        let mut m = Mat5::IDENTITY;
        m.data[4] = v.x;
        m.data[9] = v.y;
        m.data[14] = v.z;
        m.data[19] = v.w;
        m
    }

    fn from_scale(v: Self::Vec) -> Self {
        let mut m = Mat5::IDENTITY;
        m.data[0] = v.x;
        m.data[6] = v.y;
        m.data[12] = v.z;
        m.data[18] = v.w;
        m
    }

    fn with_scale(&self, v: Self::Vec) -> Self {
        let mut m = *self;
        m.data[0] *= v.x;
        m.data[6] *= v.y;
        m.data[12] *= v.z;
        m.data[18] *= v.w;
        m
    }

    fn with_translation(&self, v: Self::Vec) -> Self {
        let mut m = *self;
        m.data[4] += v.x;
        m.data[9] += v.y;
        m.data[14] += v.z;
        m.data[19] += v.w;
        m
    }

    #[inline(always)]
    fn apply(&self, v: Self::Vec) -> Self::Vec {
        *self * v
    }

    #[inline(always)]
    fn apply_vec(&self, v: Self::Vec) -> Self::Vec {
        // don't apply translation
        let mut res = Vec4::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                res[i] += self.data[i * 5 + j] * v[j];
            }
        }
        res
    }
}
