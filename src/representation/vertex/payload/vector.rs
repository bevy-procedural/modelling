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
    /// The value of Ludolph's number.
    const PI: Self;

    /// The value of the machine epsilon.
    const EPS: Self;

    /// Returns whether the scalar is strictly positive.
    fn is_positive(self) -> bool;

    /// Returns whether the scalar is strictly negative.
    fn is_negative(self) -> bool;

    /// Returns the absolute value of the scalar.
    fn abs(self) -> Self {
        if self.is_positive() {
            self
        } else {
            -self
        }
    }

    /// Returns the arcus cosine of the scalar.
    fn acos(self) -> Self;

    fn det3(
        a: Self,
        b: Self,
        c: Self,
        d: Self,
        e: Self,
        f: Self,
        g: Self,
        h: Self,
        i: Self,
    ) -> Self {
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }
}

impl Scalar for f32 {
    const PI: Self = std::f32::consts::PI;
    const EPS: Self = std::f32::EPSILON;

    #[inline(always)]
    fn is_positive(self) -> bool {
        self.is_sign_positive()
    }

    #[inline(always)]
    fn is_negative(self) -> bool {
        self.is_sign_negative()
    }

    #[inline(always)]
    fn acos(self) -> Self {
        f32::acos(self)
    }
}

impl Scalar for f64 {
    const PI: Self = std::f64::consts::PI;
    const EPS: Self = std::f64::EPSILON;

    #[inline(always)]
    fn is_positive(self) -> bool {
        self.is_sign_positive()
    }

    #[inline(always)]
    fn is_negative(self) -> bool {
        self.is_sign_negative()
    }

    #[inline(always)]
    fn acos(self) -> Self {
        f64::acos(self)
    }
}

/// Trait for tansformations in 3d space.

pub trait Transform: Clone + Copy + Default + std::fmt::Debug + 'static {
    /// The scalar type of the coordinates and angles used in the rotation.
    type S: Scalar;

    /// The vector type used in the rotation.
    type Vec: Vector<Self::S>;

    /// Returns the identity rotation.
    fn identity() -> Self;

    /// Returns a rotation from an axis and an angle.
    fn from_axis_angle(axis: Self::Vec, angle: Self::S) -> Self;

    /// Returns a rotation from a rotation arc.
    fn from_rotation_arc(from: Self::Vec, to: Self::Vec) -> Self;

    /// Constructs a transform from a translation.
    fn from_translation(v: Self::Vec) -> Self;

    /// Constructs a transform from a scale.
    fn from_scale(v: Self::Vec) -> Self;

    /// Applies the rotation/translation/scale/sheer to a vector.
    fn apply(&self, v: Self::Vec) -> Self::Vec;

    /// Applies the rotation/scale/sheer to a vector.
    fn apply_vec(&self, v: Self::Vec) -> Self::Vec ;

}

/// Trait for coordinates in n-dimensional space.
pub trait Vector<ScalarType: Scalar>:
    Copy
    + Default
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::MulAssign
    + std::ops::Div<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + 'static
{
    /// The 2d vector type used in the coordinates.
    type Vec2D: Vector2D<ScalarType>;

    /// The 3d vector type used in the coordinates.
    type Vec3D: Vector3D<ScalarType>;

    /// The rotation type used in the vector.
    type Transform: Transform<S = ScalarType, Vec = Self>;
    
    /// Returns the origin vector.
    fn zero() -> Self;

    /// Returns the number of dimensions.
    fn dimensions() -> usize;

    /// Returns the distance between two points.
    fn distance(&self, other: &Self) -> ScalarType;

    /// Returns the squared distance between two points.
    fn distance_squared(&self, other: &Self) -> ScalarType;

    /// Returns the dot product of two vectors.
    fn dot(&self, other: &Self) -> ScalarType;

    /// Returns the cross product of two vectors.
    fn cross(&self, other: &Self) -> Self;

    /// Returns the x-coordinate.
    fn x(&self) -> ScalarType;

    /// Returns the y-coordinate. (or 0 if not present)
    fn y(&self) -> ScalarType;

    /// Returns the z-coordinate. (or 0 if not present)
    fn z(&self) -> ScalarType;

    /// Returns the w-coordinate. (or 0 if not present)
    fn w(&self) -> ScalarType;

    /// Returns the coordinates as a tuple.
    fn xy(&self) -> Self::Vec2D {
        Self::Vec2D::from_xy(self.x(), self.y())
    }

    /// Returns the coordinates as a tuple.
    fn xyz(&self) -> Self::Vec3D {
        Self::Vec3D::from_xyz(self.x(), self.y(), self.z())
    }

    /// Normalizes the vector.
    fn normalize(&self) -> Self;
}

/// Trait for coordinates in 2d space.
pub trait Vector2D<ScalarType: Scalar>: Vector<ScalarType> {
    /// Construct from scalar values.
    fn from_xy(x: ScalarType, y: ScalarType) -> Self;

    /// True iff the vertex curr is a convex corner.
    /// Assume counter-clockwise vertex order.
    fn convex(&self, prev: Self, next: Self) -> bool {
        (*self - prev).cross2d(&(next - *self)).is_positive()
    }

    /// Returns the barycentric sign of a point in a triangle.
    fn barycentric_sign(a: Self, b: Self, c: Self) -> ScalarType {
        (a - c).cross2d(&(b - c))
    }

    /// Returns the cross product of two 2d vectors.
    fn cross2d(&self, other: &Self) -> ScalarType {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Whether the point is inside the triangle.
    fn is_inside_triangle(&self, a: Self, b: Self, c: Self) -> bool {
        let bs1 = Self::barycentric_sign(*self, a, b);
        let bs2 = Self::barycentric_sign(*self, b, c);
        let bs3 = Self::barycentric_sign(*self, c, a);
        let inside_ccw = bs1.is_positive() && bs2.is_positive() && bs3.is_positive();
        let inside_cw = bs1.is_negative() && bs2.is_negative() && bs3.is_negative();
        inside_ccw || inside_cw
    }

    /// Whether the point is inside the circumcircle of the triangle.
    fn is_inside_circumcircle(&self, a: Self, b: Self, c: Self) -> bool {
        // https://en.wikipedia.org/wiki/Delaunay_triangulation#Algorithms

        let adx = a.x() - self.x();
        let ady = a.y() - self.y();
        let bdx = b.x() - self.x();
        let bdy = b.y() - self.y();
        let cdx = c.x() - self.x();
        let cdy = c.y() - self.y();
        ScalarType::det3(
            adx,
            ady,
            adx * adx + ady * ady,
            bdx,
            bdy,
            bdx * bdx + bdy * bdy,
            cdx,
            cdy,
            cdx * cdx + cdy * cdy,
        )
        .is_positive()
    }
}

/// Trait for coordinates in 3d space.
pub trait Vector3D<ScalarType: Scalar>: Vector<ScalarType> {

    /// Construct from scalar values.
    fn from_xyz(x: ScalarType, y: ScalarType, z: ScalarType) -> Self;

    /// Convert to an array.
    fn to_array(&self) -> [ScalarType; 3] {
        [self.x(), self.y(), self.z()]
    }
}
