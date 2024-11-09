use super::{Scalar, Vector};

/// Trait for coordinates in 3d space.
pub trait Vector4D: Vector<Self::S> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// Construct from scalar values.
    fn new(x: Self::S, y: Self::S, z: Self::S, w: Self::S) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [Self::S; 4] {
        [self.x(), self.y(), self.z(), self.w()]
    }

    /// Returns the coordinate values as a tuple.
    fn tuple(&self) -> (Self::S, Self::S, Self::S, Self::S) {
        (self.x(), self.y(), self.z(), self.w())
    }

    /// Swizzle
    fn xyzw(&self) -> Self {
        Self::new(self.x(), self.y(), self.z(), self.w())
    }
}
