mod basics;
mod builder;
mod check;
mod halfedge;
mod pseudo_winged;

use super::HalfEdgeMeshType;
use crate::{
    mesh::{MeshTopology, MeshTrait, TransformableMesh, Triangulateable, WithNormals},
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
pub struct HalfEdgeMeshImpl<T: HalfEdgeMeshType> {
    // TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
    // TODO: non-euclidean geometry
    vertices: DeletableVector<T::Vertex, T::V>,
    halfedges: DeletableVector<T::Edge, T::E>,
    faces: DeletableVector<T::Face, T::F>,
    payload: T::MP,
}

impl<T: HalfEdgeMeshType> HalfEdgeMeshImpl<T> {
    /// Creates a new empty halfedge mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
            payload: T::MP::default(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeMeshImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HalfEdgeMeshType> TransformableMesh<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeMeshType> WithNormals<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeMeshType> MeshTopology<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeMeshType> Triangulateable<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeMeshType> MeshTrait for HalfEdgeMeshImpl<T> {
    type T = T;
}
