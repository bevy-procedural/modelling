use super::{Scalar, Vector};

/// Trait for the data structure needed to rotate the value of type V.
pub trait Rotator<V>: Clone {}

/// Trait for tansformations in nd space. We call it `TransformTrait` to avoid
/// collisions with the `Transform` struct in Bevy.

pub trait TransformTrait: Clone + Copy + Default + std::fmt::Debug + 'static {
    /// The scalar type of the coordinates and angles used in the rotation.
    type S: Scalar;

    /// The vector type used in the transformatiom.
    type Vec: Vector<Self::S>;

    /// The rotation type used in the transformation.
    type Rot: Rotator<Self::Vec>;

    /// Returns the identity transformation.
    fn identity() -> Self;

    /// Constructs a transform from a rotation.
    fn from_rotation(r: Self::Rot) -> Self;

    /// Constructs a transform from a rotation arc.
    fn from_rotation_arc(from: Self::Vec, to: Self::Vec) -> Self;

    /// Constructs a transform from a translation.
    fn from_translation(v: Self::Vec) -> Self;

    /// Constructs a transform from a scale.
    fn from_scale(v: Self::Vec) -> Self;

    /// Adds scale.
    fn with_scale(&self, v: Self::Vec) -> Self;

    /// Adds translation.
    fn with_translation(&self, v: Self::Vec) -> Self;

    /// Applies the rotation/translation/scale/sheer to a vector.
    fn apply(&self, v: Self::Vec) -> Self::Vec;

    /// Applies the rotation/scale/sheer to a vector.
    fn apply_vec(&self, v: Self::Vec) -> Self::Vec;
}
