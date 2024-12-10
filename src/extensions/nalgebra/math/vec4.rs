use super::VecN;
use crate::math::{Scalar, Vector4D};

/// A 4D vector.
pub type Vec4<S> = VecN<S, 4>;

impl<T: Scalar> Vector4D for Vec4<T> {
    type S = T;

    fn new(x: Self::S, y: Self::S, z: Self::S, w: Self::S) -> Self {
        Self::from([x, y, z, w])
    }
}
