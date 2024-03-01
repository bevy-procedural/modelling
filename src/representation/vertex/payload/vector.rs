/// To be used as a scalar in n-dimensional space.
pub trait Scalar:
    Copy
    + Default
    + PartialEq
    + PartialOrd
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + From<f32>
    + 'static
{
    
}

impl Scalar for f32 {}
impl Scalar for f64 {}

/// Trait for coordinates in n-dimensional space.
pub trait Vector<ScalarType: Scalar>:
    Copy
    + Default
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + 'static
{
    /// Returns the origin vector.
    fn zero() -> Self;

    /// Returns the number of dimensions.
    fn dimensions() -> usize;

    /// Returns the distance between two points.
    fn distance(&self, other: &Self) -> ScalarType;
}

/// Trait for coordinates in 2d space.
pub trait Vector2D<ScalarType: Scalar>: Vector<ScalarType> {
    /// Returns the x-coordinate.
    fn x(&self) -> ScalarType;

    /// Returns the y-coordinate.
    fn y(&self) -> ScalarType;

    /// Construct from scalar values.
    fn from_xy(x: ScalarType, y: ScalarType) -> Self;
}

/// Trait for coordinates in 3d space.
pub trait Vector3D<ScalarType: Scalar>: Vector<ScalarType> {
    /// Returns the x-coordinate.
    fn x(&self) -> ScalarType;

    /// Returns the y-coordinate.
    fn y(&self) -> ScalarType;

    /// Returns the z-coordinate.
    fn z(&self) -> ScalarType;

    /// Construct from scalar values.
    fn from_xyz(x: ScalarType, y: ScalarType, z: ScalarType) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [ScalarType; 3] {
        [self.x(), self.y(), self.z()]
    }
}
