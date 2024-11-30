use super::VecN;
use crate::math::{Rotator, Scalar};
use nalgebra::SMatrix;

/// Rotation in D-dimensional space.
/// Uses a orthonormal matrix to represent the rotation.
#[derive(Clone, Debug, Copy)]
pub struct NdRotate<S: Scalar, const D: usize> {
    matrix: SMatrix<S, D, D>,
}

impl<S: Scalar, const D: usize> NdRotate<S, D> {
    /// Creates a new rotation from a matrix.
    pub fn new(matrix: SMatrix<S, D, D>) -> Self {
        let s = Self { matrix };
        debug_assert!(s.is_valid(S::EPS.sqrt()));
        s
    }

    /// Checks whether the rotation matrix is orthogonal and has a determinant of +1.
    pub fn is_valid(&self, _eps: S) -> bool {
        todo!();
        //nalgebra::base::Matrix::is_orthogonal(&self.matrix, eps)
        // && (self.matrix.determinant() - S::one()).abs() < eps
    }
}

impl<S: Scalar, const D: usize> Rotator<VecN<S, D>> for NdRotate<S, D> {}
