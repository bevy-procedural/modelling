use super::{rotate::NdRotate, transform_n::NdAffine, ScalarPlus};
use crate::math::{HasZero, Scalar, TransformTrait, Transformable, Vector};
use nalgebra::SVector;

/// A N-dimensional vector.
pub type VecN<S, const D: usize> = SVector<S, D>;

impl<S: Scalar, const D: usize> HasZero for VecN<S, D> {
    #[inline]
    fn zero() -> Self {
        Self::zeros()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.iter().all(|&x| x.is_zero())
    }
}

impl<S: Scalar, const D: usize> Vector<S, D> for VecN<S, D> {
    #[inline]
    fn distance(&self, other: &Self) -> S {
        self.distance_squared(other).sqrt()
    }

    #[inline]
    fn distance_squared(&self, other: &Self) -> S {
        Scalar::stable_sum(
            self.data
                .as_slice()
                .iter()
                .zip(other.data.as_slice().iter())
                .map(|(a, b)| (*a - *b) * (*a - *b)),
        )
    }

    #[inline]
    fn length(&self) -> S {
        self.length_squared().sqrt()
    }

    #[inline]
    fn length_squared(&self) -> S {
        Scalar::stable_sum(self.data.as_slice().iter().map(|a| *a * *a))
    }

    #[inline]
    fn dot(&self, other: &Self) -> S {
        Scalar::stable_sum(
            self.data
                .as_slice()
                .iter()
                .zip(other.data.as_slice().iter())
                .map(|(a, b)| *a * *b),
        )
    }

    #[inline]
    fn x(&self) -> S {
        self[0]
    }

    #[inline]
    fn y(&self) -> S {
        if D >= 2 {
            self[1]
        } else {
            S::ZERO
        }
    }

    #[inline]
    fn z(&self) -> S {
        if D >= 3 {
            self[2]
        } else {
            S::ZERO
        }
    }

    #[inline]
    fn w(&self) -> S {
        if D >= 4 {
            self[3]
        } else {
            S::ZERO
        }
    }

    #[inline]
    fn normalize(&self) -> Self {
        let length = self.length();
        if length.is_zero() {
            *self
        } else {
            *self / length
        }
    }

    #[inline]
    fn splat(value: S) -> Self {
        Self::from([value; D])
    }

    #[inline]
    fn from_x(x: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        Self::from(data)
    }

    #[inline]
    fn from_xy(x: S, y: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        if D >= 2 {
            data[1] = y;
        }
        Self::from(data)
    }
    #[inline]
    fn from_xyz(x: S, y: S, z: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        if D >= 2 {
            data[1] = y;
        }
        if D >= 3 {
            data[2] = z;
        }
        Self::from(data)
    }

    #[inline]
    fn is_about(&self, other: &Self, eps: S) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.is_about(*b, eps))
    }
}

impl<S: ScalarPlus, const D: usize> Transformable<D> for VecN<S, D> {
    type S = S;
    type Rot = NdRotate<S, D>;
    type Trans = NdAffine<S, D>;
    type Vec = VecN<S, D>;

    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        *self = t.apply(*self);
        self
    }

    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        *self += *v;
        self
    }

    fn scale(&mut self, s: &Self::Vec) -> &mut Self {
        for i in 0..D {
            self[i] *= s[i];
        }
        self
    }

    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        for i in 0..D {
            self[i] = self[i].lerp(other[i], t);
        }
        self
    }
}
