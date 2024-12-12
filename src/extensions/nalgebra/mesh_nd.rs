use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    mesh::{
        EmptyEdgePayload, EmptyFacePayload, EmptyMeshPayload, EuclideanMeshType, MeshType,
        MeshType3D, MeshTypeHalfEdge,
    },
};

use super::{NdAffine, NdRotate, Polygon2d, VecN, VertexPayloadPNU};

/// A mesh type for nalgebra with
/// - nd vertices,
/// - usize indices,
/// - no face or edge payload,
/// - f64 vertex positions, normals, and uv coordinates
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MeshTypeNd64PNU<const D: usize>;

/// 2d variant of MeshTypeNd64PNU
pub type MeshType2d64PNU = MeshTypeNd64PNU<2>;
/// 3d variant of MeshTypeNd64PNU
pub type MeshType3d64PNU = MeshTypeNd64PNU<3>;
/// 4d variant of MeshTypeNd64PNU
pub type MeshType4d64PNU = MeshTypeNd64PNU<4>;

impl<const D: usize> MeshType for MeshTypeNd64PNU<D> {
    type E = usize;
    type V = usize;
    type F = usize;
    type EP = EmptyEdgePayload<Self>;
    type VP = VertexPayloadPNU<f64, D>;
    type FP = EmptyFacePayload<Self>;
    type MP = EmptyMeshPayload<Self>;
    type Mesh = MeshNd64<D>;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}
impl<const D: usize> EuclideanMeshType<D> for MeshTypeNd64PNU<D> {
    type S = f64;
    type Vec = VecN<f64, D>;
    type Vec2 = VecN<f64, 2>;
    type Trans = NdAffine<f64, D>;
    type Rot = NdRotate<f64, D>;
    type Poly = Polygon2d<f64>;
}
impl<const D: usize> HalfEdgeImplMeshType for MeshTypeNd64PNU<D> {}
impl<const D: usize> MeshTypeHalfEdge for MeshTypeNd64PNU<D> {}
impl MeshType3D for MeshTypeNd64PNU<3> {}

/// A mesh with
/// - nalgebra nd vertices,
/// - usize indices,
/// - f64 positions, normals, and uv coordinates
pub type MeshNd64<const D: usize> = HalfEdgeMeshImpl<MeshTypeNd64PNU<D>>;
/// 2d variant of MeshNd64
pub type Mesh2d64 = MeshNd64<2>;
/// 3d variant of MeshNd64
pub type Mesh3d64 = MeshNd64<3>;
/// 4d variant of MeshNd64
pub type Mesh4d64 = MeshNd64<4>;

/// 64-bit 3d variant of the half-edge vertex
pub type Mesh3d64Vertex = HalfEdgeVertexImpl<MeshTypeNd64PNU<3>>;

/// 64-bit 3d variant of the half-edge edge
pub type Mesh3d64Edge = HalfEdgeImpl<MeshTypeNd64PNU<3>>;

/// 64-bit 3d variant of the half-edge face
pub type Mesh3d64Face = HalfEdgeFaceImpl<MeshTypeNd64PNU<3>>;
