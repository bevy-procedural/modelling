//! Generic nalgebra implementation of the mathematical traits.

mod affine;
mod polygon;
mod rotate;
mod transform;
mod vec2;
mod vec3;
mod vec4;
mod vec_n;

pub use affine::*;
pub use polygon::*;
pub use rotate::*;
pub use transform::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vec_n::*;

use crate::math::Scalar;
use nalgebra::{RealField, Scalar as ScalarNalgebra, SimdComplexField, SimdRealField};

// TODO: this is a bit restrictive... Can we somehow avoid using the Simd-traits?

/// A scalar that can be used with nalgebra.
pub trait ScalarPlus:
    Scalar + ScalarNalgebra + SimdComplexField + SimdRealField + RealField
{
}

impl ScalarPlus for f32 {}
impl ScalarPlus for f64 {}
