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
    
    /// Returns the cross product of two vectors.
    fn cross(&self, other: &Self) -> Self;

    /// Returns the coordinate values as a tuple.
    fn tuple(&self) -> (Self::S, Self::S, Self::S) {
        (self.x(), self.y(), self.z())
    }

    /// Swizzle
    fn xyz(&self) -> Self {
        Self::new(self.x(), self.y(), self.z())
    }

    /// Swizzle
    fn xzy(&self) -> Self {
        Self::new(self.x(), self.z(), self.y())
    }

    /// Swizzle
    fn yxz(&self) -> Self {
        Self::new(self.y(), self.x(), self.z())
    }

    /// Swizzle
    fn yzx(&self) -> Self {
        Self::new(self.y(), self.z(), self.x())
    }

    /// Swizzle
    fn zxy(&self) -> Self {
        Self::new(self.z(), self.x(), self.y())
    }

    /// Swizzle
    fn zyx(&self) -> Self {
        Self::new(self.z(), self.y(), self.x())
    }
}
