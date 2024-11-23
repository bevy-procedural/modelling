use super::{Bevy2DPolygon, BevyMesh3d, BevyVertexPayload2d, BevyVertexPayload3d};
use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    math::HasPosition,
    mesh::{
        CurvedEdge, CurvedEdgePayload, CurvedEdgeType, EdgeBasics, EmptyEdgePayload,
        EmptyFacePayload, EmptyMeshPayload, MeshBasics, MeshBuilder, MeshType, MeshTypeHalfEdge,
    },
};
use bevy::math::{Affine2, Vec2, Vec3, Vec4};
use itertools::Itertools;

/// A mesh type for bevy with 2D vertices, 32 bit indices, 32 bit floats, and no face or edge payload (no normals etc.)
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BevyMeshType2d32;

impl MeshType for BevyMeshType2d32 {
    type E = u32;
    type V = u32;
    type F = u32;
    type EP = CurvedEdgePayload<Self>;
    type VP = BevyVertexPayload2d;
    type FP = EmptyFacePayload;
    type MP = EmptyMeshPayload;
    type S = f32;
    type Vec = Vec2;
    type Vec2 = Vec2;
    type Vec3 = Vec3;
    type Vec4 = Vec4;
    type Trans = Affine2;
    type Rot = f32;
    type Poly = Bevy2DPolygon;
    type Mesh = BevyMesh2d;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}
impl HalfEdgeImplMeshType for BevyMeshType2d32 {}
impl MeshTypeHalfEdge for BevyMeshType2d32 {}

impl CurvedEdge<BevyMeshType2d32> for HalfEdgeImpl<BevyMeshType2d32> {
    fn curve_type(&self) -> CurvedEdgeType<BevyMeshType2d32> {
        self.payload().curve_type()
    }

    fn set_curve_type(&mut self, curve_type: CurvedEdgeType<BevyMeshType2d32>) {
        self.payload_mut().set_curve_type(curve_type);
    }
}

/// A mesh with bevy 2D vertices. Edges may be curved.
pub type BevyMesh2d = HalfEdgeMeshImpl<BevyMeshType2d32>;

impl HalfEdgeMeshImpl<BevyMeshType2d32> {
    /// Convert a BevyMesh2d to a 3d mesh.
    /// If there are curved edges they will be converted with the given tolerance.
    pub fn to_3d(&self, tol: f32) -> BevyMesh3d {
        // TODO: optimize this
        let mut mesh = self.clone();

        // Convert curved edges
        for e in mesh.edge_ids().collect::<Vec<_>>().iter() {
            let edge = mesh.edge(*e);
            if edge.curve_type() != CurvedEdgeType::Linear {
                let vs = edge.flatten_casteljau(tol, &mesh);
                if vs.len() == 0 {
                    continue;
                }
                mesh.insert_vertices_into_edge(
                    *e,
                    vs.iter().map(|v| {
                        (
                            CurvedEdgePayload::default(),
                            CurvedEdgePayload::default(),
                            BevyVertexPayload2d::from_pos(*v),
                        )
                    }),
                );
                mesh.edge_mut(*e).set_curve_type(CurvedEdgeType::Linear);
            }
        }

        BevyMesh3d::import_mesh::<_, _, _, BevyMeshType2d32>(
            &mesh,
            |vp: &BevyVertexPayload2d| {
                BevyVertexPayload3d::from_pos(Vec3::new(vp.pos().x, vp.pos().y, 0.0))
            },
            |_ep| EmptyEdgePayload::default(),
            |_fp| EmptyFacePayload::default(),
        )
    }
}
