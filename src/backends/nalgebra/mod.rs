//! This module contains backend-independent nalgebra implementations

mod math;
mod vertex_payload_2d;
/*mod mesh2d;
mod mesh3d;
mod vertex_payload_3d;
*/

pub use math::*;
pub use vertex_payload_2d::*;
/*pub use mesh2d::*;
pub use mesh3d::*;
pub use vertex_payload_3d::*;
*/