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
//pub mod operations;
//pub mod primitives;
pub mod tesselate;
pub mod util;
#[cfg(feature = "bevy")]
pub mod bevy;
