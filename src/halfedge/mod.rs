//! This module implements a half-edge data structure for representing meshes.

mod edge;
mod face;
mod mesh;
mod vertex;
mod primitives;

pub use edge::*;
pub use face::*;
pub use mesh::*;
pub use vertex::*;

use crate::mesh::MeshType;

/// This trait defines the associated types used in a half-edge mesh and puts them into relation.
pub trait HalfEdgeMeshType:
    MeshType<
    Mesh = HalfEdgeMesh<Self>,
    Vertex = HalfEdgeVertex<Self>,
    Edge = HalfEdge<Self>,
    Face = HalfEdgeFace<Self>,
>
{
}
