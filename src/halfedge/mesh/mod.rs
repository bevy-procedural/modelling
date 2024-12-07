mod basics;
mod builder;
mod check;
mod halfedge;
mod pseudo_winged;

use super::HalfEdgeImplMeshType;
use crate::{
    math::HasNormal,
    mesh::{
        EuclideanMeshType, MeshTopology, MeshTrait, TransformableMesh, Triangulateable, WithNormals,
    },
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
pub struct HalfEdgeMeshImpl<T: HalfEdgeImplMeshType> {
    // TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
    // TODO: non-euclidean geometry
    vertices: DeletableVector<T::Vertex, T::V>,
    halfedges: DeletableVector<T::Edge, T::E>,
    faces: DeletableVector<T::Face, T::F>,
    payload: T::MP,
}

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
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

impl<T: HalfEdgeImplMeshType> Default for HalfEdgeMeshImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const D: usize, T: HalfEdgeImplMeshType + EuclideanMeshType<D>> TransformableMesh<D, T>
    for HalfEdgeMeshImpl<T>
{
}
impl<
        const D: usize,
        T: HalfEdgeImplMeshType + EuclideanMeshType<D, VP: HasNormal<D, T::Vec, S = T::S>>,
    > WithNormals<D, T> for HalfEdgeMeshImpl<T>
{
}
impl<T: HalfEdgeImplMeshType> MeshTopology<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeImplMeshType> Triangulateable<T> for HalfEdgeMeshImpl<T> {}
impl<T: HalfEdgeImplMeshType> MeshTrait for HalfEdgeMeshImpl<T> {
    type T = T;
}

#[cfg(feature = "netsci")]
impl<T: HalfEdgeImplMeshType> crate::mesh::NetworkScience<T> for HalfEdgeMeshImpl<T> {}
