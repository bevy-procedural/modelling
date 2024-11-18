#![allow(dead_code)]

#![doc = include_str!("../README.md")]
#![doc = include_str!("../doc/start.md")]

pub mod halfedge;
pub mod math;
pub mod mesh;
pub mod tesselate;
pub mod util;
pub mod operations;
pub mod primitives;

#[cfg(feature = "bevy")]
pub mod bevy;

/*
/// A prelude for easy importing of commonly used types and traits.
pub mod prelude {
    pub use crate::halfedge::*;
    pub use crate::math::*;
    pub use crate::mesh::*;
    pub use crate::tesselate::*;
    pub use crate::util::*;
    pub use crate::operations::*;

    #[cfg(feature = "bevy")]
    pub use crate::bevy::*;
}
*/