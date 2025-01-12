//! This module contains backend-independent nalgebra implementations

mod default_vertex_payload;
mod math;
mod mesh2d;
mod mesh_nd;
mod index;

pub use default_vertex_payload::*;
pub use math::*;
pub use mesh2d::*;
pub use mesh_nd::*;
pub use index::*;