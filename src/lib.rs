#![allow(dead_code)]

//!
//! [![crates.io](https://img.shields.io/crates/v/procedural_modelling)](https://crates.io/crates/procedural_modelling)
//! [![GitHub Repo stars](https://img.shields.io/github/stars/bevy-procedural/modelling)](https://github.com/bevy-procedural/modelling)
//!
//! A framework-agnostic Procedural Modelling crate.
//!

pub mod halfedge;
pub mod math;
pub mod mesh;
pub mod tesselate;
pub mod util;
pub mod operations;
//pub mod primitives;

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

    #[cfg(feature = "bevy")]
    pub use crate::bevy::*;
}
*/