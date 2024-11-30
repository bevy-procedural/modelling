use crate::math::{HasZero, Scalar, Vector};

use super::transform_n::TransformN;

/// A generic vector with N elements.
///
/// Operations are applied element-wise, e.g.,
/// `Vec2(a,b) * Vec2(c,d) = Vec2(a*c, b*d)`.
#[derive(Clone, Copy)]
pub struct VecN<const N: usize, T: Scalar> {
    data: [T; N],
}


impl<const N: usize, S: Scalar> VecN<N, S> {
    pub fn new(data: [S; N]) -> Self {
        Self { data }
    }

    pub fn as_ref(&self) -> &[S; N] {
        &self.data
    }

    pub fn as_mut(&mut self) -> &mut [S; N] {
        &mut self.data
    }

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
    }
}

impl<const N: usize, S: Scalar> std::ops::Index<usize> for VecN<N, S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, S: Scalar> std::ops::IndexMut<usize> for VecN<N, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, S: Scalar> std::ops::Add for VecN<N, S>
where
    S: std::ops::Add<Output = S>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] + rhs.data[i];
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Sub for VecN<N, S>
where
    S: std::ops::Sub<Output = S>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] - rhs.data[i];
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Mul<S> for VecN<N, S>
where
    S: std::ops::Mul<Output = S> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] * rhs;
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Mul<VecN<N, S>> for VecN<N, S>
where
    S: std::ops::Mul<Output = S> + std::ops::Add<Output = S> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: VecN<N, S>) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] * rhs.data[i];
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Div<S> for VecN<N, S>
where
    S: std::ops::Div<Output = S> + Copy,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] / rhs;
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Div<VecN<N, S>> for VecN<N, S>
where
    S: std::ops::Div<Output = S> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = self.data[i] / rhs.data[i];
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::Neg for VecN<N, S>
where
    S: std::ops::Neg<Output = S>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut data = [S::ZERO; N];
        for i in 0..N {
            data[i] = -self.data[i];
        }
        Self { data }
    }
}

impl<const N: usize, S: Scalar> std::ops::AddAssign for VecN<N, S>
where
    S: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<const N: usize, S: Scalar> std::ops::SubAssign for VecN<N, S>
where
    S: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<const N: usize, S: Scalar> std::ops::MulAssign<S> for VecN<N, S>
where
    S: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: S) {
        for i in 0..N {
            self.data[i] *= rhs;
        }
    }
}

impl<const N: usize, S: Scalar> std::ops::MulAssign<VecN<N, S>> for VecN<N, S>
where
    S: std::ops::MulAssign + std::ops::AddAssign + Copy,
{
    fn mul_assign(&mut self, rhs: VecN<N, S>) {
        for i in 0..N {
            self.data[i] *= rhs.data[i];
        }
    }
}

impl<const N: usize, S: Scalar> std::ops::DivAssign<S> for VecN<N, S>
where
    S: std::ops::DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: S) {
        for i in 0..N {
            self.data[i] /= rhs;
        }
    }
}

impl<const N: usize, S: Scalar> std::ops::DivAssign<VecN<N, S>> for VecN<N, S>
where
    S: std::ops::DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: VecN<N, S>) {
        for i in 0..N {
            self.data[i] /= rhs.data[i];
        }
    }
}

impl<const N: usize, T: Scalar> HasZero for VecN<N, T> {
    const ZERO: Self = Self { data: [T::ZERO; N] };
}

impl<const N: usize, T: Scalar> std::fmt::Debug for VecN<N, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}(", N)?;
        for i in 0..N {
            write!(f, "{:?}", self.data[i])?;
            if i < N - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

impl<const N: usize, T: Scalar> std::fmt::Display for VecN<N, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}(", N)?;
        for i in 0..N {
            write!(f, "{}", self.data[i])?;
            if i < N - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

impl<const N: usize, T: Scalar> PartialEq for VecN<N, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, T: Scalar> Eq for VecN<N, T> where T: Eq {}

impl<const N: usize, T: Scalar> Default for VecN<N, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<const N: usize, T: Scalar> Vector<T> for VecN<N, T> {
    type Vec2 = VecN<2, T>;
    type Vec3 = VecN<3, T>;
    type Vec4 = VecN<4, T>;
    type Trans = TransformN<N, T>;

    #[inline(always)]
    fn dimensions() -> usize {
        N
    }

    #[inline(always)]
    fn distance(&self, other: &Self) -> T {
        self.distance_squared(other).sqrt()
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> T {
        Scalar::stable_sum(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| (*a - *b) * (*a - *b)),
        )
    }

    #[inline(always)]
    fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    fn length_squared(&self) -> T {
        Scalar::stable_sum(self.data.iter().map(|a| *a * *a))
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> T {
        Scalar::stable_sum(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a * *b),
        )
    }

    #[inline(always)]
    fn x(&self) -> T {
        self.data[0]
    }

    #[inline(always)]
    fn y(&self) -> T {
        if N >= 2 {
            self.data[1]
        } else {
            T::ZERO
        }
    }

    #[inline(always)]
    fn z(&self) -> T {
        if N >= 3 {
            self.data[2]
        } else {
            T::ZERO
        }
    }

    #[inline(always)]
    fn w(&self) -> T {
        if N >= 4 {
            self.data[3]
        } else {
            T::ZERO
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
    fn splat(value: T) -> Self {
        Self::new([value; N])
    }

    #[inline(always)]
    fn from_x(x: T) -> Self {
        let mut data = [T::ZERO; N];
        data[0] = x;
        Self { data }
    }

    #[inline(always)]
    fn from_xy(x: T, y: T) -> Self {
        let mut data = [T::ZERO; N];
        data[0] = x;
        if N >= 2 {
            data[1] = y;
        }
        Self { data }
    }
    #[inline(always)]
    fn from_xyz(x: T, y: T, z: T) -> Self {
        let mut data = [T::ZERO; N];
        data[0] = x;
        if N >= 2 {
            data[1] = y;
        }
        if N >= 3 {
            data[2] = z;
        }
        Self { data }
    }

    #[inline(always)]
    fn is_about(&self, other: &Self, epsilon: T) -> bool {
        // TODO: robust comparison
        for i in 0..N {
            if (self.data[i] - other.data[i]).abs() > epsilon {
                return false;
            }
        }
        true
    }
}
