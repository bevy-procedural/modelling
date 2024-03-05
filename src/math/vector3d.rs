use super::{Scalar, Vector};

/// Trait for coordinates in 3d space.
pub trait Vector3D<ScalarType: Scalar>: Vector<ScalarType> {
    /// Construct from scalar values.
    fn from_xyz(x: ScalarType, y: ScalarType, z: ScalarType) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [ScalarType; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Returns the non-normalized normal of the vector.
    fn normal(&self, prev: Self, next: Self) -> Self {
        let a = *self - prev;
        let b = next - prev;
        a.cross(&b)
    }
}
