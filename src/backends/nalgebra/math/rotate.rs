use super::{ScalarPlus, VecN};
use crate::math::{Rotator, Scalar};
use nalgebra::SMatrix;

/// Rotation in D-dimensional space.
#[derive(Clone, Debug, Copy)]
pub struct NdRotate<S: Scalar, const D: usize> {
    // TODO: this is clumsy
    /// rotation in 2D space
    rot2: Option<nalgebra::Rotation2<S>>,

    /// rotation in 3D space
    rot3: Option<nalgebra::Rotation3<S>>,

    /// rotation in D-dimensional space
    rot: Option<SMatrix<S, D, D>>,
}

impl<S: Scalar, const D: usize> NdRotate<S, D> {
    /// Creates a new rotation from a matrix.
    pub fn new(matrix: SMatrix<S, D, D>) -> Self {
        if D == 2 {
            Self {
                rot2: Some(nalgebra::Rotation2::from_matrix_unchecked(
                    matrix.fixed_view::<2, 2>(0, 0).into_owned(),
                )),
                rot3: None,
                rot: None,
            }
        } else if D == 3 {
            Self {
                rot2: None,
                rot3: Some(nalgebra::Rotation3::from_matrix_unchecked(
                    matrix.fixed_view::<3, 3>(0, 0).into_owned(),
                )),
                rot: None,
            }
        } else {
            Self {
                rot2: None,
                rot3: None,
                rot: Some(matrix),
            }
        }
    }

    /// Returns the matrix representation of the rotation.
    pub fn to_matrix(&self) -> SMatrix<S, D, D> {
        if let Some(rot) = &self.rot {
            rot.clone()
        } else if let Some(rot2) = &self.rot2 {
            rot2.matrix().fixed_resize::<D, D>(S::zero())
        } else if let Some(rot3) = &self.rot3 {
            rot3.matrix().fixed_resize::<D, D>(S::zero())
        } else {
            panic!("No rotation matrix found");
        }
    }

    pub fn from_rotation_arc(from: VecN<S, D>, to: VecN<S, D>) -> Self
    where
        S: ScalarPlus,
    {
        if D == 2 {
            Self {
                rot2: Some(nalgebra::Rotation2::rotation_between(
                    &from.fixed_rows::<2>(0).into_owned(),
                    &to.fixed_rows::<2>(0).into_owned(),
                )),
                rot3: None,
                rot: None,
            }
        } else if D == 3 {
            Self {
                rot2: None,
                rot3: Some(nalgebra::Rotation3::rotation_between(&from, &to)),
                rot: None,
            }
        } else {
            todo!();
        }
    }
}
/*

pub trait NdRotateTrait<S: Scalar, const D: usize> {
    /// Creates a new rotation from a matrix.
    fn new(matrix: SMatrix<S, D, D>) -> Self;

    /// Checks whether the rotation matrix is orthogonal and has a determinant of +1.
    fn is_valid(&self, _eps: S) -> bool {
        todo!();
        //nalgebra::base::Matrix::is_orthogonal(&self.matrix, eps)
        // && (self.matrix.determinant() - S::one()).abs() < eps
    }

    /// Returns the matrix representation of the rotation.
    fn as_matrix(&self) -> &SMatrix<S, D, D>;

    /// Creates a new rotation from a rotation arc.
    fn from_rotation_arc(from: VecN<S, D>, to: VecN<S, D>) -> Self;
}*/

impl<S: Scalar, const D: usize> Rotator<VecN<S, D>> for NdRotate<S, D> {}
