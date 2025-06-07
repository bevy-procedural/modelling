//! This module contains backend-independent nalgebra implementations

mod default_vertex_payload;
mod math;
mod mesh2d;
mod mesh_nd;

pub use default_vertex_payload::*;
pub use math::*;
pub use mesh2d::*;
pub use mesh_nd::*;

/// reexport nalgebra types
pub mod nalgebra {
    // TODO: is this a good idea?
    // pub use nalgebra::*;
}
