use super::{Rotator, Scalar, TransformTrait, Vector};

/// A trait that defines how a vertex payload can be linearly transformed.
pub trait Transformable<const D: usize>: Sized + Clone {
    /// The transformation type used in the payload.
    type Trans: TransformTrait<Self::S, D, Vec = Self::Vec, Rot = Self::Rot>;

    /// The rotation type used in the payload.
    type Rot: Rotator<Self::Vec>;

    /// The vector type used in the payload.
    type Vec: Vector<Self::S, D, Trans = Self::Trans>;

    /// The scalar type of the coordinates used in the payload. Mainly to choose between f32 and f64. But could also work with integers etc...
    type S: Scalar;

    /// Returns the coordinates of the payload as a reference.
    fn transformed(&self, t: &Self::Trans) -> Self {
        let mut c = self.clone();
        c.transform(t);
        c
    }

    /// Returns a translated clone of the payload.
    fn translated(&self, v: &Self::Vec) -> Self {
        let mut c = self.clone();
        c.translate(v);
        c
    }

    /// Returns the scaled clone of the payload.
    fn scaled(&self, s: &Self::Vec) -> Self {
        let mut c = self.clone();
        c.scale(s);
        c
    }

    /// Returns the rotated clone of the payload.
    fn rotated(&self, r: &Self::Rot) -> Self {
        let mut c = self.clone();
        c.rotate(r);
        c
    }

    /// Interpolates between two payloads.
    fn lerped(&self, other: &Self, t: Self::S) -> Self {
        let mut c = self.clone();
        c.lerp(other, t);
        c
    }

    /// Returns the coordinates of the payload as a reference.
    fn transform(&mut self, t: &Self::Trans) -> &mut Self;

    /// Returns a translated clone of the payload.
    fn translate(&mut self, v: &Self::Vec) -> &mut Self {
        self.transform(&Self::Trans::from_translation(*v))
    }

    /// Returns the scaled clone of the payload.
    fn scale(&mut self, s: &Self::Vec) -> &mut Self {
        self.transform(&Self::Trans::from_scale(*s))
    }

    /// Returns the rotated clone of the payload.
    fn rotate(&mut self, r: &Self::Rot) -> &mut Self {
        self.transform(&Self::Trans::from_rotation(r.clone()))
    }

    /// Interpolates between two payloads.
    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self;
}
