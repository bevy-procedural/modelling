//mod check;
mod basics;
mod builder;
mod check;
mod mesh_type;
mod normals;
mod payload;
mod position;
mod topology;
mod transform;
mod triangulate;

pub use basics::*;
pub use builder::*;
pub use check::*;
pub use mesh_type::*;
pub use normals::*;
pub use payload::*;
pub use position::*;
pub use topology::*;
pub use transform::*;
pub use triangulate::*;

/// The `MeshTrait` doesn't assume any specific data structure or topology,
/// i.e., could be a manifold half-edge mesh, a topological directed graph, etc.
pub trait MeshTrait: MeshBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Mesh = Self>;
}
