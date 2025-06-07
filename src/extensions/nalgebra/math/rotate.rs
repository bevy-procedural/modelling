use super::{ScalarPlus, VecN};
use crate::math::{Rotator, Scalar, Vector};
use nalgebra::SMatrix;

/// Rotation in D-dimensional space.
#[derive(Clone, Debug, Copy)]
pub struct NdRotate<S: Scalar, const D: usize> {
    // TODO: this is clumsy. But nd-rotation is extremely hard to implement. We should probably update the traits to not include the hard to implement rotation methods!
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

    /// Creates a new rotation from an angle. Works only for 2D.
    pub fn from_angle(angle: S) -> Self
    where
        S: ScalarPlus,
    {
        assert!(D == 2);
        Self {
            rot2: Some(nalgebra::Rotation2::new(angle)),
            rot3: None,
            rot: None,
        }
    }

    /// Creates a new rotation from an axis and an angle. Works only for 3D.
    pub fn from_axis_angle(axis: nalgebra::Unit<VecN<S, 3>>, angle: S) -> Self
    where
        S: ScalarPlus,
    {
        assert!(D == 3);
        Self {
            rot2: None,
            rot3: Some(nalgebra::Rotation3::from_axis_angle(&axis, angle)),
            rot: None,
        }
    }

    /// Creates a new rotation from a rotation arc.
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
                rot3: if let Some(r) = nalgebra::Rotation3::rotation_between(
                    &from.fixed_rows::<3>(0).into_owned(),
                    &to.fixed_rows::<3>(0).into_owned(),
                ) {
                    Some(r)
                } else if Vector::<S, D>::is_about(&from, &(-to), S::EPS) {
                    // rotate 180 degrees around any axis
                    Some(nalgebra::Rotation3::from_axis_angle(
                        &nalgebra::Unit::new_normalize(VecN::<S, 3>::new(S::ONE, S::ZERO, S::ZERO)),
                        S::PI,
                    ))
                } else {
                    assert!(false, "Could not create rotation arc {} {}", from, to);
                    None
                },

                rot: None,
            }
        } else {
            todo!();
        }
    }
}

impl<S: Scalar, const D: usize> Rotator<VecN<S, D>> for NdRotate<S, D> {}

#[cfg(test)]
mod tests {
    use crate::extensions::nalgebra::*;
    use ::nalgebra::{SMatrix, Unit};

    #[test]
    fn test_nd_rotate() {
        let rot = NdRotate::<f64, 2>::from_angle(std::f64::consts::PI / 2.0);
        let expected = SMatrix::<f64, 2, 2>::new(0.0, -1.0, 1.0, 0.0);
        assert!((rot.to_matrix() - expected).abs().max() < 1e-10);

        let rot = NdRotate::<f64, 2>::from_rotation_arc(
            VecN::<f64, 2>::new(1.0, 0.0),
            VecN::<f64, 2>::new(0.0, 1.0),
        );
        let expected = SMatrix::<f64, 2, 2>::new(0.0, -1.0, 1.0, 0.0);
        assert!((rot.to_matrix() - expected).abs().max() < 1e-10);

        let rot: NdRotate<f64, 3> = NdRotate::<f64, 3>::from_axis_angle(
            Unit::new_normalize(VecN::<f64, 3>::new(1.0, 0.0, 0.0)),
            std::f64::consts::PI / 2.0,
        );
        let expected = SMatrix::<f64, 3, 3>::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
        assert!((rot.to_matrix() - expected).abs().max() < 1e-10);

        // The same operation can be done with a rotation arc
        let rot = NdRotate::<f64, 3>::from_rotation_arc(
            VecN::<f64, 3>::new(0.0, 1.0, 0.0),
            VecN::<f64, 3>::new(0.0, 0.0, 1.0),
        );
        let expected = SMatrix::<f64, 3, 3>::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
        assert!((rot.to_matrix() - expected).abs().max() < 1e-10);
    }
}
