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
    Mesh = HalfEdgeMeshImpl<Self>,
    Vertex = HalfEdgeVertexImpl<Self>,
    Edge = HalfEdgeImpl<Self>,
    Face = HalfEdgeFaceImpl<Self>,
>
{
}
