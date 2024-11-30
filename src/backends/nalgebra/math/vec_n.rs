use nalgebra::SVector;

use crate::math::{HasZero, Scalar, Vector};

use super::transform_n::TransformN;

/*
/// Returns the angle between two vectors.
pub fn angle_between(&self, other: Self) -> S {
    let len_self = self.length();
    let len_other = other.length();

    if len_self.is_zero() || len_other.is_zero() {
        // Angle is undefined for zero-length vectors; handle as needed
        return S::ZERO;
    }

    let cos_theta = self.dot(&other) / (len_self * len_other);

    // Clamp cos_theta to [-1, 1] to handle numerical inaccuracies
    cos_theta.clamp(-S::ONE, S::ONE).acos()
}*/

impl<S: Scalar, const D: usize> HasZero for SVector<S, D> {
    const ZERO: Self = Self::from([S::ZERO; D]);
}

impl<S: Scalar, const D: usize> Vector<S, D> for SVector<S, D> {
    type Vec2 = SVector<S, 2>;
    type Vec3 = SVector<S, 3>;
    type Vec4 = SVector<S, 4>;
    type Trans = TransformN<S, D>;

    #[inline(always)]
    fn distance(&self, other: &Self) -> S {
        self.distance_squared(other).sqrt()
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> S {
        Scalar::stable_sum(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| (*a - *b) * (*a - *b)),
        )
    }

    #[inline(always)]
    fn length(&self) -> S {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    fn length_squared(&self) -> S {
        Scalar::stable_sum(self.data.iter().map(|a| *a * *a))
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> S {
        Scalar::stable_sum(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a * *b),
        )
    }

    #[inline(always)]
    fn x(&self) -> S {
        self.data[0]
    }

    #[inline(always)]
    fn y(&self) -> S {
        if D >= 2 {
            self.data[1]
        } else {
            S::ZERO
        }
    }

    #[inline(always)]
    fn z(&self) -> S {
        if D >= 3 {
            self.data[2]
        } else {
            S::ZERO
        }
    }

    #[inline(always)]
    fn w(&self) -> S {
        if D >= 4 {
            self.data[3]
        } else {
            S::ZERO
        }
    }

    #[inline(always)]
    fn normalize(&self) -> Self {
        let length = self.length();
        if length.is_zero() {
            *self
        } else {
            *self / length
        }
    }

    #[inline(always)]
    fn splat(value: S) -> Self {
        Self::new([value; D])
    }

    #[inline(always)]
    fn from_x(x: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        Self { data }
    }

    #[inline(always)]
    fn from_xy(x: S, y: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        if D >= 2 {
            data[1] = y;
        }
        Self { data }
    }
    #[inline(always)]
    fn from_xyz(x: S, y: S, z: S) -> Self {
        let mut data = [S::ZERO; D];
        data[0] = x;
        if D >= 2 {
            data[1] = y;
        }
        if D >= 3 {
            data[2] = z;
        }
        Self { data }
    }

    #[inline(always)]
    fn is_about(&self, other: &Self, epsilon: S) -> bool {
        // TODO: robust comparison
        for i in 0..D {
            if (self.data[i] - other.data[i]).abs() > epsilon {
                return false;
            }
        }
        true
    }
}
