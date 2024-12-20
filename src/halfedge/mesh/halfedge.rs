use super::HalfEdgeMeshImpl;
use crate::{halfedge::HalfEdgeImplMeshType, mesh::HalfEdgeMesh};

impl<T: HalfEdgeImplMeshType> HalfEdgeMesh<T> for HalfEdgeMeshImpl<T> {}
