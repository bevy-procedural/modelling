//mod check;
mod basics;
mod builder;
mod check;
mod halfedge;
mod halfedge_builder;
mod iso;
mod mesh_type;
mod normals;
mod path_builder;
mod payload;
mod position;
mod topology;
mod transform;
mod triangulate;

pub use basics::*;
pub use builder::*;
pub use check::*;
pub use halfedge::*;
pub use halfedge_builder::*;
pub use iso::*;
pub use mesh_type::*;
pub use normals::*;
pub use path_builder::*;
pub use payload::*;
pub use position::*;
pub use topology::*;
pub use transform::*;
pub use triangulate::*;

#[cfg(feature = "netsci")]
mod netsci;

#[cfg(feature = "netsci")]
pub use netsci::*;

#[cfg(feature = "fonts")]
mod fonts;

#[cfg(feature = "fonts")]
pub use fonts::*;

/// The `MeshTrait` doesn't assume any specific data structure or topology,
/// i.e., could be a manifold half-edge mesh, a topological directed graph, etc.
pub trait MeshTrait: MeshBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Mesh = Self>;
}
