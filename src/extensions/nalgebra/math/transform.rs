use super::{NdAffine, NdRotate, ScalarPlus, VecN};
use crate::math::{Scalar, TransformTrait, Vector3D};
use nalgebra::{DMatrix, Matrix4, Perspective3, RealField, SMatrix, SVector};

/// Linear Transformation in N1-dimensional space.
#[derive(Clone, Debug)]
pub struct NdHomography<S: Scalar, const N: usize> {
    //matrix: SMatrix<S, N+1, N+1>,
    // TODO: Avoid DMatrix. How to express this in const generics?
    matrix: DMatrix<S>,
}

impl<S: Scalar, const N: usize> NdHomography<S, N> {
    /// Creates a new homography from a matrix.
    pub fn from_homogenous(matrix: DMatrix<S>) -> Self {
        assert_eq!(
            matrix.nrows(),
            N + 1,
            "N1 must be equal to N + 1 for a homography, but got N1 = {} and N = {}",
            matrix.nrows(),
            N
        );
        assert_eq!(
            matrix.ncols(),
            N + 1,
            "N1 must be equal to N + 1 for a homography, but got N1 = {} and N = {}",
            matrix.ncols(),
            N
        );
        Self { matrix }
    }

    /// Creates a new homography from a SMatrix.
    pub fn from_matrix<const N1: usize>(matrix: &SMatrix<S, N1, N1>) -> Self {
        let mut d = DMatrix::<S>::identity(N1, N1);
        d.view_mut((0, 0), (N1, N1)).copy_from(matrix);
        Self::from_homogenous(d)
    }

    // Returns the homography as an augmented matrix.
    fn as_matrix(&self) -> &DMatrix<S> {
        &self.matrix
    }
}

impl<S: Scalar + RealField> NdHomography<S, 3> {
    /// Creates a new homography from a perspective matrix.
    pub fn from_perspective(aspect: S, fov_y: S, z_near: S, z_far: S) -> Self {
        Self::from_matrix(Perspective3::<S>::new(aspect, fov_y, z_near, z_far).as_matrix())
    }

    /// Builds a left-handed look-at view matrix.
    pub fn look_at_lh<Vec: Vector3D<S = S>>(eye: &Vec, target: &Vec, up: &Vec) -> Self {
        Self::from_matrix(&Matrix4::<S>::look_at_lh(
            &eye.to_array().into(),
            &target.to_array().into(),
            &up.to_array().into(),
        ))
    }
}

impl<S: Scalar, const N: usize> Default for NdHomography<S, N> {
    fn default() -> Self {
        Self::from_homogenous(DMatrix::<S>::identity(N + 1, N + 1))
    }
}

impl<S: Scalar, const N: usize> Into<NdAffine<S, N>> for NdHomography<S, N> {
    fn into(self) -> NdAffine<S, N> {
        let mut d = DMatrix::<S>::identity(N + 1, N + 1);
        d.copy_from(&self.matrix);
        NdAffine::from_matrix(d)
    }
}

// Converts an n dimensional point to an n+1 dimensional point by appending 1 at the end.
fn to_homogenous_point<S: Scalar, const N: usize>(v: &VecN<S, N>) -> nalgebra::DVector<S> {
    let mut v1 = nalgebra::DVector::<S>::zeros(N + 1);
    v1.rows_mut(0, N).copy_from(&v);
    v1[N] = S::ONE;
    v1
}

// Converts an n dimensional vector to an n+1 dimensional vector by appending 0 at the end.
fn to_homogenous_vector<S: Scalar, const N: usize>(v: &VecN<S, N>) -> nalgebra::DVector<S> {
    let mut v0 = nalgebra::DVector::<S>::zeros(N + 1);
    v0.rows_mut(0, N).copy_from(&v);
    v0
}

impl<S: ScalarPlus, const N: usize> TransformTrait<S, N> for NdHomography<S, N> {
    type Vec = VecN<S, N>;
    type Rot = NdRotate<S, N>;

    fn identity() -> Self {
        Self::default()
    }

    fn apply_point(&self, mut v: Self::Vec) -> Self::Vec {
        // TODO: Don't do homogenous conversions internally.
        // PERF: avoid cloning https://nalgebra.org/docs/user_guide/projections/
        let res = self.matrix.clone() * to_homogenous_point(&v);
        v.copy_from(&res.rows(0, N));
        v / res[N]
    }

    fn apply_vec(&self, mut v: Self::Vec) -> Self::Vec {
        // TODO: Don't do homogenous conversions internally.
        // PERF: avoid cloning https://nalgebra.org/docs/user_guide/projections/
        let res = self.matrix.clone() * to_homogenous_vector(&v);
        v.copy_from(&res.rows(0, N));
        v / res[N]
    }

    fn chain(&self, other: &Self) -> Self {
        // PERF: This can be optimized
        Self::from_homogenous(self.as_matrix() * other.as_matrix())
    }

    fn from_rotation(r: Self::Rot) -> Self {
        let mut matrix = DMatrix::<S>::identity(N + 1, N + 1);
        matrix.view_mut((0, 0), (N, N)).copy_from(&r.to_matrix());
        Self::from_homogenous(matrix)
    }

    fn from_rotation_arc(from: Self::Vec, to: Self::Vec) -> Self {
        Self::from_rotation(NdRotate::from_rotation_arc(from, to))
    }

    fn from_scale(v: Self::Vec) -> Self {
        let mut matrix = DMatrix::<S>::identity(N + 1, N + 1);
        matrix
            .view_mut((0, 0), (N, N))
            .copy_from(&SMatrix::<S, N, N>::from_diagonal(&v));
        Self::from_homogenous(matrix)
    }

    fn from_translation(v: Self::Vec) -> Self {
        let mut matrix = DMatrix::<S>::identity(N + 1, N + 1);
        matrix
            .view_mut((0, N), (N, 1))
            .copy_from(&SVector::<S, N>::from(v));
        Self::from_homogenous(matrix)
    }

    fn with_scale(&self, v: Self::Vec) -> Self {
        Self::from_scale(v).chain(self)
    }

    fn with_translation(&self, v: Self::Vec) -> Self {
        Self::from_translation(v).chain(self)
    }
}
