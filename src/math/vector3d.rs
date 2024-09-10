use super::{Scalar, Vector};

/// Trait for coordinates in 3d space.
pub trait Vector3D: Vector<Self::S> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// Construct from scalar values.
    fn new(x: Self::S, y: Self::S, z: Self::S) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [Self::S; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Returns the non-normalized normal of the vector.
    fn normal(&self, prev: Self, next: Self) -> Self {
        let a = *self - prev;
        let b = next - prev;
        a.cross(&b)
    }
}
