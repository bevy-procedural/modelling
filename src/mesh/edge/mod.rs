mod basics;
mod payload;
mod halfedge;

pub use basics::*;
pub use payload::*;
pub use halfedge::*;

use super::MeshType;

/// A directed or undirected edge or halfedge in a mesh.
pub trait Edge: EdgeBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Edge = Self>;
}
