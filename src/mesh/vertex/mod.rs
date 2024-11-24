mod basics;
mod halfedge;
mod interpolator;
mod payload;

pub use basics::*;
pub use halfedge::*;
pub use interpolator::*;
pub use payload::*;

use super::MeshType;
use crate::math::HasPosition;

/// A vertex in a mesh.
pub trait Vertex: VertexBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Vertex = Self>;
}
