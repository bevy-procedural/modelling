use crate::math::Scalar;

/// A generic transformation matrix with N elements.
#[derive(Clone, Copy)]
pub struct TransformN<S: Scalar, const N: usize> {
    data: [[S; N]; N],
}
