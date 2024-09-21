use super::HalfEdgeMesh;
use crate::halfedge::HalfEdgeMeshType;

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /*
    /// Whether the mesh has non-manifold vertices
    pub fn has_nonmanifold_vertices(&self) -> bool {
        self.vertices.iter().any(|v| !v.is_manifold())
    }

    /// Whether the mesh is manifold, i.e., has no boundary edges and no non-manifold vertices
    pub fn is_manifold(&self) -> bool {
        !self.is_open() && !self.has_nonmanifold_vertices()
    }*/
}
