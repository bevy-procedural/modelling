use super::{NdHomography, NdRotate, ScalarPlus, VecN};
use crate::math::{Scalar, TransformTrait};
use nalgebra::{DMatrix, SMatrix, SVector};

/// Affine transformation in D-dimensional space.
/// Represented as a N x N matrix with an additional row for translation since const generics are unstable in rust.
#[derive(Clone, Debug, Copy)]
pub struct NdAffine<S: Scalar, const N: usize> {
    matrix: SMatrix<S, N, N>,
    translation: VecN<S, N>,
}

impl<S: Scalar, const D: usize> NdAffine<S, D> {
    /// Creates a new affine transformation from a matrix and a translation vector.
    pub fn new(matrix: SMatrix<S, D, D>, translation: VecN<S, D>) -> Self {
        Self {
            matrix,
            translation,
        }
    }

    // Returns the affine transformation as an augmented matrix.
    fn as_matrix(&self) -> DMatrix<S> {
        // PERF: avoid DMatrix
        let mut m = DMatrix::<S>::identity(D + 1, D + 1);
        m.view_mut((0, 0), (D, D)).copy_from(&self.matrix);
        m.view_mut((0, D), (D, 1)).copy_from(&self.translation);
        m
    }

    // Constructs an affine transformation from an augmented matrix.
    // Panics if the matrix is not in the expected format
    pub(super) fn from_matrix(m: DMatrix<S>) -> Self {
        assert!(m.nrows() == D + 1 && m.ncols() == D + 1);
        assert!(m.fixed_view::<1, D>(D, 0).iter().all(|&x| x == S::ZERO));
        Self::new(
            m.fixed_view::<D, D>(0, 0).into_owned(),
            m.fixed_view::<D, 1>(0, D).into_owned(),
        )
    }

    /// Returns true if the two affine transformations are approximately equal.
    pub fn is_about(&self, other: &Self, eps: S) -> bool
    where
        S: ScalarPlus,
    {
        (self.matrix - other.matrix).abs().max() < eps
            && (self.translation - other.translation).abs().max() < eps
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

    fn apply_point(&self, v: Self::Vec) -> Self::Vec {
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
        Self::from_matrix(other.as_matrix() * self.as_matrix())
    }
}

impl<S: Scalar, const N: usize> Into<NdHomography<S, N>> for NdAffine<S, N> {
    fn into(self) -> NdHomography<S, N> {
        NdHomography::from_homogenous(self.as_matrix())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector;

    use super::*;
    use nalgebra::{Matrix2, Matrix3};

    #[test]
    fn test_affine_transformations_2d() {
        type V2 = VecN<f64, 2>;

        // translate by (1, 2)
        let a = NdAffine::<f64, 2>::new(Matrix2::new(1.0, 0.0, 0.0, 1.0), V2::new(1.0, 2.0));
        // scale by 2 and translate by (3, 4)
        let b = NdAffine::<f64, 2>::new(Matrix2::new(2.0, 0.0, 0.0, 2.0), V2::new(3.0, 4.0));
        let c = b.chain(&a);

        assert_eq!(a.apply_point(V2::new(0.0, 0.0)), V2::new(1.0, 2.0));
        assert_eq!(a.apply_point(V2::new(1.0, 1.0)), V2::new(2.0, 3.0));
        assert_eq!(b.apply_point(V2::new(0.0, 0.0)), V2::new(3.0, 4.0));
        assert_eq!(b.apply_point(V2::new(1.0, 1.0)), V2::new(5.0, 6.0));
        assert_eq!(c.apply_point(V2::new(0.0, 0.0)), V2::new(4.0, 6.0));
        assert_eq!(c.apply_point(V2::new(1.0, 1.0)), V2::new(6.0, 8.0));

        assert!(NdAffine::<f64, 2>::from_translation(V2::new(1.0, 2.0)).is_about(&a, 1e-10));
        assert!(NdAffine::<f64, 2>::from_scale(V2::new(2.0, 2.0))
            .with_translation(V2::new(3.0, 4.0))
            .is_about(&b, 1e-10));
        assert!(
            NdAffine::<f64, 2>::from_translation(V2::new(3.0 / 2.0, 4.0 / 2.0))
                .with_scale(V2::new(2.0, 2.0))
                .is_about(&b, 1e-10)
        );
        assert!(a.is_about(&NdAffine::<f64, 2>::from_matrix(a.as_matrix()), 1e-10));
        assert!(b.is_about(&NdAffine::<f64, 2>::from_matrix(b.as_matrix()), 1e-10));
        assert!(c.is_about(&NdAffine::<f64, 2>::from_matrix(c.as_matrix()), 1e-10));

        assert!(
            (NdAffine::<f64, 2>::default().as_matrix() - Matrix3::identity())
                .abs()
                .max()
                < 1e-10
        );
        assert!(
            (NdAffine::<f64, 2>::identity().as_matrix() - Matrix3::identity())
                .abs()
                .max()
                < 1e-10
        );

        let rot = NdRotate::<f64, 2>::from_angle(std::f64::consts::PI / 2.0);
        let rot_affine = NdAffine::<f64, 2>::from_rotation(rot);
        assert!(rot_affine
            .apply_point(V2::new(1.0, 0.0))
            .is_about(&V2::new(0.0, 1.0), 1e-10));

        let move1 = NdAffine::<f64, 2>::from_translation(V2::new(1.0, 0.0));

        assert!(move1
            .chain(&rot_affine)
            .apply_point(V2::new(2.0, 0.0))
            .is_about(
                &rot_affine.apply_point(move1.apply_point(V2::new(2.0, 0.0))),
                1e-10
            ));
        assert!(rot_affine
            .chain(&move1)
            .apply_point(V2::new(2.0, 0.0))
            .is_about(
                &move1.apply_point(rot_affine.apply_point(V2::new(2.0, 0.0))),
                1e-10
            ));
        assert!(rot_affine
            .chain(&move1)
            .apply_point(V2::new(0.0, 2.0))
            .is_about(
                &move1.apply_point(rot_affine.apply_point(V2::new(0.0, 2.0))),
                1e-10
            ));

        assert!(move1
            .chain(&rot_affine)
            .apply_point(V2::new(2.0, 0.0))
            .is_about(&V2::new(0.0, 3.0), 1e-10));
        assert!(rot_affine
            .chain(&move1)
            .apply_point(V2::new(2.0, 0.0))
            .is_about(&V2::new(1.0, 2.0), 1e-10));
        assert!(move1
            .chain(&rot_affine)
            .apply_point(V2::new(0.0, 2.0))
            .is_about(&V2::new(-2.0, 1.0), 1e-10));
        assert!(rot_affine
            .chain(&move1)
            .apply_point(V2::new(0.0, 2.0))
            .is_about(&V2::new(-1.0, 0.0), 1e-10));
    }
}
