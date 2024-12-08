use super::{NdRotate, ScalarPlus, VecN};
use crate::math::{Scalar, TransformTrait};
use nalgebra::{DMatrix, SMatrix, SVector};

/// Affine transformation in D-dimensional space.
/// Represented as a D x D matrix with an additional row for translation since const generics are unstable in rust.
#[derive(Clone, Debug, Copy)]
pub struct NdAffine<S: Scalar, const D: usize> {
    matrix: SMatrix<S, D, D>,
    translation: VecN<S, D>,
}

impl<S: Scalar, const D: usize> NdAffine<S, D> {
    /// Creates a new affine transformation from a matrix and a translation vector.
    pub fn new(matrix: SMatrix<S, D, D>, translation: VecN<S, D>) -> Self {
        Self {
            matrix,
            translation,
        }
    }

    fn as_matrix(&self) -> DMatrix<S> {
        let mut m = DMatrix::<S>::identity(D + 1, D + 1);
        m.view_mut((0, 0), (D, D)).copy_from(&self.matrix);
        m.view_mut((0, D), (D, 1)).copy_from(&self.translation);
        m
    }

    fn from_matrix(m: DMatrix<S>) -> Self {
        assert!(m.nrows() == D + 1 && m.ncols() == D + 1);
        assert!(m.fixed_view::<1, D>(D, 0).iter().all(|&x| x == S::ZERO));
        Self::new(
            m.fixed_view::<D, D>(0, 0).into_owned(),
            m.fixed_view::<D, 1>(0, D).into_owned(),
        )
    }
}

impl<S: Scalar, const D: usize> Default for NdAffine<S, D> {
    fn default() -> Self {
        Self::new(SMatrix::<S, D, D>::identity(), SVector::<S, D>::zeros())
    }
}

impl<S: ScalarPlus, const D: usize> TransformTrait<S, D> for NdAffine<S, D> {
    type Vec = VecN<S, D>;
    type Rot = NdRotate<S, D>;

    fn identity() -> Self {
        Self::default()
    }

    fn apply(&self, v: Self::Vec) -> Self::Vec {
        self.matrix * v + self.translation
    }

    fn apply_vec(&self, v: Self::Vec) -> Self::Vec {
        self.matrix * v
    }

    fn from_rotation(r: Self::Rot) -> Self {
        Self {
            matrix: r.to_matrix().clone(),
            translation: VecN::zeros(),
        }
    }

    fn from_rotation_arc(from: Self::Vec, to: Self::Vec) -> Self {
        Self::from_rotation(NdRotate::from_rotation_arc(from, to))
    }

    fn from_scale(v: Self::Vec) -> Self {
        Self::new(
            SMatrix::<S, D, D>::from_diagonal(&v),
            SVector::<S, D>::zeros(),
        )
    }

    fn from_translation(v: Self::Vec) -> Self {
        Self::new(SMatrix::<S, D, D>::identity(), v)
    }

    fn with_scale(&self, v: Self::Vec) -> Self {
        Self {
            matrix: self.matrix * SMatrix::<S, D, D>::from_diagonal(&v),
            translation: self.translation.component_mul(&v),
        }
    }

    fn with_translation(&self, v: Self::Vec) -> Self {
        Self {
            matrix: self.matrix.clone(),
            translation: self.translation.clone() + v,
        }
    }

    fn chain(&self, other: &Self) -> Self {
        // PERF: This can be optimized
        Self::from_matrix(self.as_matrix() * other.as_matrix())
    }
}
