use super::{BevyMesh3d, BevyVertexPayload2d, BevyVertexPayload3d, Polygon2dBevy};
use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    math::HasPosition,
    mesh::{
        CurvedEdge, CurvedEdgePayload, CurvedEdgeType, EmptyEdgePayload, EmptyFacePayload,
        EmptyMeshPayload, EuclideanMeshType, MeshBasics, MeshBasicsCurved, MeshType,
        MeshTypeHalfEdge,
    },
    prelude::HalfEdgeImplMeshTypePlus,
};
use bevy::math::{Affine2, Vec2, Vec3};

/// A mesh type for bevy with
/// - 2D vertices,
/// - 32 bit indices,
/// - no face payloads,
/// - f32 vertex positions and uv coordinates,
/// - but no vertex normals
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BevyMeshType2d32;

impl MeshType for BevyMeshType2d32 {
    type E = u32;
    type V = u32;
    type F = u32;
    type EP = CurvedEdgePayload<2, Self>;
    type VP = BevyVertexPayload2d;
    type FP = EmptyFacePayload<Self>;
    type MP = EmptyMeshPayload<Self>;
    type Mesh = BevyMesh2d;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}

impl EuclideanMeshType<2> for BevyMeshType2d32 {
    type S = f32;
    type Vec = Vec2;
    type Vec2 = Vec2;
    type Trans = Affine2;
    type Rot = f32;
    type Poly = Polygon2dBevy;
}

impl HalfEdgeImplMeshType for BevyMeshType2d32 {}
impl HalfEdgeImplMeshTypePlus for BevyMeshType2d32 {}
impl MeshTypeHalfEdge for BevyMeshType2d32 {}

impl CurvedEdge<2, BevyMeshType2d32> for HalfEdgeImpl<BevyMeshType2d32> {
    fn curve_type(&self, mesh: &BevyMesh2d) -> CurvedEdgeType<2, BevyMeshType2d32> {
        mesh.edge_payload(self).curve_type()
    }

    fn set_curve_type_in_mesh(
        &self,
        mesh: &mut BevyMesh2d,
        curve_type: CurvedEdgeType<2, BevyMeshType2d32>,
    ) {
        mesh.edge_payload_mut(self).set_curve_type(curve_type);
    }
}

/// A mesh with bevy 2D vertices. Edges may be curved.
pub type BevyMesh2d = HalfEdgeMeshImpl<BevyMeshType2d32>;

impl HalfEdgeMeshImpl<BevyMeshType2d32> {
    /// Convert a BevyMesh2d to a 3d mesh.
    /// If there are curved edges they will be converted with the given tolerance.
    pub fn to_3d(&self, tol: f32) -> BevyMesh3d {
        BevyMesh3d::import_mesh::<_, _, _, _, BevyMeshType2d32>(
            self.clone().flatten_curved_edges(tol),
            |vp: &BevyVertexPayload2d| {
                BevyVertexPayload3d::from_pos(Vec3::new(vp.pos().x, vp.pos().y, 0.0))
            },
            |_ep| {
                // TODO: flatten_curved_edges seems to miss some edges?
                //assert!(ep.is_empty()); // no curves or anything
                EmptyEdgePayload::default()
            },
            |_fp| EmptyFacePayload::default(),
            |_mp| EmptyMeshPayload::default(),
        )
    }
}
