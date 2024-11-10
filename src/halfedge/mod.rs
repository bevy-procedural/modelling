//! This module implements a half-edge data structure for representing meshes.

mod edge;
mod face;
mod mesh;
mod primitives;
mod vertex;

pub use edge::*;
pub use face::*;
pub use mesh::*;
pub use vertex::*;

use crate::mesh::{HalfEdgeMeshType, MeshType};

/// This trait defines the associated types used in this half-edge mesh implementation and puts them into relation.
pub trait HalfEdgeImplMeshType:
    MeshType<
    Mesh = HalfEdgeMeshImpl<Self>,
    Vertex = HalfEdgeVertexImpl<Self>,
    Edge = HalfEdgeImpl<Self>,
    Face = HalfEdgeFaceImpl<Self>,
>
{
}
