use crate::math::{HasZero, Scalar, TransformTrait, Transformable, Vector, Vector2D};

type Vec2<T: Scalar> = super::vec_n::VecN<2, T>;

impl<T: Scalar> Vector2D for Vec2<T> {
    type S = T;

    #[inline(always)]
    fn new(x: T, y: T) -> Self {
        Self::new([x, y])
    }

    /// Magnitude of the vector.
    fn magnitude(&self) -> T {
        self.length()
    }

    /// Angle between two vectors.
    fn angle(&self, a: Self, b: Self) -> T {
        (a - *self).angle_between(b - *self)
    }

    fn perp_dot(&self, other: &Self) -> T {
        self.x() * other.y() - self.y() * other.x()
    }
}

/*
impl TransformTrait for Affine2 {
    type S = f32;
    type Vec = Vec2;
    type Rot = f32;

    #[inline(always)]
    fn identity() -> Self {
        Affine2::IDENTITY
    }

    fn from_rotation(angle: f32) -> Self {
        bevy::math::Affine2::from_angle(angle)
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec2, to: Vec2) -> Self {
        bevy::math::Affine2::from_angle(from.angle_between(to))
    }

    #[inline(always)]
    fn from_translation(v: Vec2) -> Self {
        bevy::math::Affine2::from_translation(v)
    }

    #[inline(always)]
    fn from_scale(v: Vec2) -> Self {
        bevy::math::Affine2::from_scale(v)
    }

    #[inline(always)]
    fn with_scale(&self, scale: Self::Vec) -> Self {
        bevy::math::Affine2::from_scale(scale) * *self
    }

    #[inline(always)]
    fn with_translation(&self, v: Self::Vec) -> Self {
        bevy::math::Affine2::from_translation(v) * *self
    }

    #[inline(always)]
    fn apply(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_point2(self, v)
    }

    #[inline(always)]
    fn apply_vec(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_vector2(self, v)
    }
}

impl Transformable for Vec2 {
    type S = f32;
    type Rot = f32;
    type Trans = Affine2;
    type Vec = Vec2;
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        *self = t.apply(*self);
        self
    }

    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        *self = bevy::math::Vec2::lerp(*self, *other, t);
        self
    }
}
*/
