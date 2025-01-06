use crate::math::{HasZero, Scalar, TransformTrait, Transformable, Vector, Vector2D};
use bevy::math::{Affine2, Vec2};

impl Vector<f32, 2> for Vec2 {
    // Don't use the bevy implementation since it is approximate!
    /*#[inline]
    fn angle_between(&self, other: Self) -> f32 {
        Vec2::angle_to(*self, other)
    }*/

    #[inline]
    fn distance(&self, other: &Self) -> f32 {
        Vec2::distance(*self, *other)
    }

    #[inline]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec2::distance_squared(*self, *other)
    }

    #[inline]
    fn length(&self) -> f32 {
        Vec2::length(*self)
    }

    #[inline]
    fn length_squared(&self) -> f32 {
        Vec2::length_squared(*self)
    }

    #[inline]
    fn dot(&self, other: &Self) -> f32 {
        Vec2::dot(*self, *other)
    }

    #[inline]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    fn z(&self) -> f32 {
        0.0
    }

    #[inline]
    fn w(&self) -> f32 {
        0.0
    }

    #[inline]
    fn normalize(&self) -> Self {
        Vec2::normalize(*self)
    }

    #[inline]
    fn splat(value: f32) -> Self {
        Vec2::splat(value)
    }

    #[inline]
    fn from_x(x: f32) -> Self {
        Vec2::new(x, 0.0)
    }

    #[inline]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }

    /// drop the z coordinate
    #[inline]
    fn from_xyz(x: f32, y: f32, _: f32) -> Self {
        Vec2::new(x, y)
    }

    #[inline]
    fn is_about(&self, other: &Self, epsilon: f32) -> bool {
        self.x.is_about(other.x, epsilon) && self.y.is_about(other.y, epsilon)
    }
}

impl HasZero for Vec2 {
    #[inline]
    fn zero() -> Self {
        Vec2::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Vec2::ZERO
    }
}

impl Vector2D for Vec2 {
    type S = f32;

    #[inline]
    fn new(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }

    fn perp_dot(&self, other: &Self) -> Self::S {
        Vec2::perp_dot(*self, *other)
    }
}

impl TransformTrait<f32, 2> for Affine2 {
    type Vec = Vec2;
    type Rot = f32;

    #[inline]
    fn identity() -> Self {
        Affine2::IDENTITY
    }

    fn from_rotation(angle: f32) -> Self {
        bevy::math::Affine2::from_angle(angle)
    }

    #[inline]
    fn from_rotation_arc(from: Vec2, to: Vec2) -> Self {
        bevy::math::Affine2::from_angle(from.angle_to(to))
    }

    #[inline]
    fn from_translation(v: Vec2) -> Self {
        bevy::math::Affine2::from_translation(v)
    }

    #[inline]
    fn from_scale(v: Vec2) -> Self {
        bevy::math::Affine2::from_scale(v)
    }

    #[inline]
    fn with_scale(&self, scale: Self::Vec) -> Self {
        bevy::math::Affine2::from_scale(scale) * *self
    }

    #[inline]
    fn with_translation(&self, v: Self::Vec) -> Self {
        bevy::math::Affine2::from_translation(v) * *self
    }

    #[inline]
    fn apply(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_point2(self, v)
    }

    #[inline]
    fn apply_vec(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_vector2(self, v)
    }

    #[inline]
    fn chain(&self, other: &Self) -> Self {
        *self * *other
    }
}

impl Transformable<2> for Vec2 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Vector, Vector2D};

    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_vec2_bevy_nalgebra() {
        use crate::extensions::nalgebra as na;

        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        let to_na = |v: Vec2| na::Vec2::<f32>::new(v.x(), v.y());

        assert!(Vector2D::angle_tri(&Vec2::ZERO, a, b).is_about(std::f32::consts::FRAC_PI_2, 1e-6));
        assert!(Vector2D::angle_tri(&Vector::zero(), to_na(a), to_na(b))
            .is_about(std::f32::consts::FRAC_PI_2, 1e-6));

        assert!(Vector2D::perp_dot(&a, &b).is_about(1.0, 1e-6));
        assert!(Vector2D::perp_dot(&to_na(a), &to_na(b)).is_about(1.0, 1e-6));

        let c = Vec2::new(1.0, 1.0);
        let d = Vec2::new(-1.0, 1.0);

        assert!(Vector2D::angle_tri(&a, c, d).is_about(1.1071486, 1e-6));
        assert!(Vector2D::angle_tri(&to_na(a), to_na(c), to_na(d)).is_about(1.1071486, 1e-6));

        assert!(Vector2D::perp_dot(&c, &d).is_about(2.0, 1e-6));
        assert!(Vector2D::perp_dot(&to_na(c), &to_na(d)).is_about(2.0, 1e-6));

        // TODO: more
    }

    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_vec2_bevy_nalgebra_fuzzer() {
        use crate::extensions::nalgebra as na;

        for _ in 1..10 {
            let a = Vec2::new(rand::random(), rand::random());
            let b = Vec2::new(rand::random(), rand::random());

            // some minimum length
            if a.length() <= 1e-04 || b.length() <= 1e-04 {
                continue;
            }

            let to_na = |v: Vec2| na::Vec2::<f32>::new(v.x(), v.y());

            println!(
                "a: {:?}, b: {:?} {} {}",
                a,
                b,
                Vector2D::angle_tri(&Vector::zero(), a, b),
                Vector2D::angle_tri(&Vector::zero(), to_na(a), to_na(b))
            );

            assert!(Vector2D::angle_tri(&Vector::zero(), a, b).is_about(
                Vector2D::angle_tri(&Vector::zero(), to_na(a), to_na(b)),
                1e-6
            ));

            assert!(Vector2D::perp_dot(&a, &b).is_about(Vector2D::perp_dot(&a, &b), 1e-6));
        }
    }
}
