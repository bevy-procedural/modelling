//! This module contains the backend-specific implementations

#[cfg(feature = "bevy")]
pub mod bevy;

#[cfg(feature = "wgpu")]
pub mod wgpu;

#[cfg(feature = "svg")]
pub mod svg;

pub mod native;
