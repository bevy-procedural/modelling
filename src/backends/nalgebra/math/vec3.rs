use super::VecN;
use crate::math::{Scalar, Spherical3d, Vector3D};

/// A 3D vector.
pub type Vec3<S> = VecN<S, 3>;

impl<T: Scalar> Vector3D for Vec3<T> {
    type S = T;
    type Spherical = Vec3<T>;

    fn new(x: Self::S, y: Self::S, z: Self::S) -> Self {
        Self::from([x, y, z])
    }

    fn cross(&self, other: &Self) -> Self {
        nalgebra::Matrix::cross(self, other)
    }
}

impl<T: Scalar> Spherical3d for Vec3<T> {
    type S = T;
    type Vec3 = Vec3<T>;
}
