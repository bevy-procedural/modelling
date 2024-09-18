pub mod edge;
pub mod face;
pub mod mesh;
pub mod vertex;

pub use edge::*;
pub use face::*;
pub use mesh::*;
pub use vertex::*;

use crate::mesh::MeshType;

pub trait HalfEdgeMeshType:
    MeshType<
    Mesh = HalfEdgeMesh<Self>,
    Vertex = HalfEdgeVertex<Self>,
    Edge = HalfEdge<Self>,
    Face = HalfEdgeFace<Self>,
>
{
}
