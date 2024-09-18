mod basics;
mod iterator;
mod topology;
mod check;

use super::{HalfEdge, HalfEdgeMeshType};
use crate::{
    mesh::{Edge, MeshTrait, MeshBasics, MeshNormals, MeshTransforms},
    util::DeletableVector,
};

/// A halfedge-inspired mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are currently not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
///
/// Currently only euclidean geometry is supported.
#[derive(Clone)]
pub struct HalfEdgeMesh<T: HalfEdgeMeshType> {
    // TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
    // TODO: non-euclidean geometry
    vertices: DeletableVector<T::Vertex, T::V>,
    halfedges: DeletableVector<T::Edge, T::E>,
    faces: DeletableVector<T::Face, T::F>,
    payload: T::MP,
}

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Creates a new empty halfedge mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
            payload: T::MP::default(),
        }
    }

    /// Flips the edge, i.e., swaps the origin and target vertices.
    pub fn flip_edge(&mut self, e: T::E) -> &mut Self {
        HalfEdge::flip(e, self);
        self
    }

    /// Flip all edges (and faces) turning the mesh inside out.
    pub fn flip(&mut self) -> &mut Self {
        // TODO: this is an unnecessary clone
        let ids: Vec<T::E> = self.edges().map(|e| e.id()).collect();
        ids.iter().for_each(|&e| {
            self.flip_edge(e);
        });
        self
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeMesh<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HalfEdgeMeshType> MeshTransforms<T> for HalfEdgeMesh<T> {}
impl<T: HalfEdgeMeshType> MeshNormals<T> for HalfEdgeMesh<T> {}
impl<T: HalfEdgeMeshType> MeshTrait<T> for HalfEdgeMesh<T> {}
