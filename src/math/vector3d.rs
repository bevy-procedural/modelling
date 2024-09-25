use super::{HasZero, Scalar, Vector};

/// Trait for spherical coordinates in 3d space.
pub trait Spherical3d: Vector<Self::S> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// Construct from scalar values.
    fn new(r: Self::S, phi: Self::S, theta: Self::S) -> Self {
        Self::from_xyz(r, phi, theta)
    }

    /// Returns the radial distance.
    fn r(&self) -> Self::S {
        self.x()
    }

    /// Returns the azimuthal angle.
    fn phi(&self) -> Self::S {
        self.y()
    }

    /// Returns the polar angle.
    fn theta(&self) -> Self::S {
        self.z()
    }

    /// Converts to cartesian coordinates.
    fn cartesian(&self) -> Self::Vec3 {
        let r = self.r();
        let phi = self.phi();
        let theta = self.theta();

        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let z = r * theta.cos();

        Self::Vec3::new(x, y, z)
    }
}

/// Trait for coordinates in 3d space.
pub trait Vector3D: Vector<Self::S> {
    /// The scalar type of the coordinates used in the vector
    type S: Scalar;

    /// The associated spherical 3d vector type
    type Spherical: Spherical3d<S = Self::S, Vec3 = Self>;

    /// Construct from scalar values.
    fn new(x: Self::S, y: Self::S, z: Self::S) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [Self::S; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Returns the non-normalized normal of the vector.
    fn normal(&self, prev: Self, next: Self) -> Self {
        (*self - prev).cross(&(next - prev))
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

    /// Convert to spherical coordinates.
    fn spherical(&self) -> Self::Spherical {
        let r = self.length();
        let phi = self.y().atan2(self.x());
        let theta = if r == Self::S::ZERO {
            Self::S::ZERO
        } else {
            (self.z() / r).acos()
        };

        Self::Spherical::new(r, phi, theta)
    }

    /// Spherical Interpolation
    fn slerp(&self, other: &Self, t: Self::S) -> Self {
        debug_assert!(
            self.length() - Self::S::ONE < Self::S::EPS.sqrt(),
            "slerp requires normalized vectors"
        );
        debug_assert!(
            other.length() - Self::S::ONE < Self::S::EPS.sqrt(),
            "slerp requires normalized vectors"
        );

        let mut dot = self.dot(other);

        // Clamp the dot product to stay within the valid range of acos
        dot = dot.clamp(-Self::S::ONE, Self::S::ONE);

        // Calculate the angle between the vectors
        let theta = dot.acos();

        // If the angle is very small, return linear interpolation to avoid numerical issues
        if theta.abs() < Self::S::EPS.sqrt() {
            return *self;
        }

        let sin_theta = theta.sin();

        // Interpolate
        let st1 = ((Self::S::ONE - t) * theta).sin() / sin_theta;
        let st2 = (t * theta).sin() / sin_theta;

        // Return the interpolated vector
        *self * st1 + *other * st2
    }
}
