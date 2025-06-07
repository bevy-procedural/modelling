//! This module contains a mini renderer that produces animated svg output for the mesh.
//! It is useful for debugging and visualizing small meshes in a browser.
//! Since it doesn't have any external dependencies, it compiles very quickly and is easy to use.

mod lighting;
mod settings;
mod svg;

pub use settings::*;
pub use svg::*;
