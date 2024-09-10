//! Traits to define the geometric primitives and operations used in the library.

pub mod impls;
mod index_type;
mod line_segment;
mod scalar;
mod transform;
mod vector;
mod vector2d;
mod vector3d;
mod vector4d;
mod polygon;
mod quaternion;

pub use index_type::*;
pub use line_segment::*;
pub use scalar::*;
pub use transform::*;
pub use vector::*;
pub use vector2d::*;
pub use vector3d::*;
pub use vector4d::*;
pub use polygon::*;
pub use quaternion::*;
