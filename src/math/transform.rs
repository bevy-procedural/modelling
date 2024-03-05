use super::{Scalar, Vector};

/// Trait for tansformations in 3d space.

pub trait Transform: Clone + Copy + Default + std::fmt::Debug + 'static {
    /// The scalar type of the coordinates and angles used in the rotation.
    type S: Scalar;

    /// The vector type used in the rotation.
    type Vec: Vector<Self::S>;

    /// Returns the identity rotation.
    fn identity() -> Self;

    /// Returns a rotation from a rotation arc.
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
