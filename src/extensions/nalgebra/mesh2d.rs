use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    math::{HasPosition, Vector},
    mesh::{
        CurvedEdge, CurvedEdgePayload, CurvedEdgeType, EdgeBasics, EmptyEdgePayload,
        EmptyFacePayload, EmptyMeshPayload, EuclideanMeshType, MeshBasics, MeshType,
        MeshTypeHalfEdge,
    },
};

use super::{MeshNd64, NdAffine, NdRotate, Polygon2d, VecN, VertexPayloadPNU};

/// A mesh type for nalgebra with
/// - 2D vertices,
/// - usize indices,
/// - no face payloads,
/// - curved edge payload,
/// - f64 vertex positions and uv coordinates,
/// - no vertex normals,
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct MeshType2d64PNUCurved;

impl MeshType for MeshType2d64PNUCurved {
    type E = usize;
    type V = usize;
    type F = usize;
    type EP = CurvedEdgePayload<2, Self>;
    type VP = VertexPayloadPNU<f64, 2>;
    type FP = EmptyFacePayload<Self>;
    type MP = EmptyMeshPayload<Self>;
    type Mesh = Mesh2d64Curved;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}

impl EuclideanMeshType<2> for MeshType2d64PNUCurved {
    type S = f64;
    type Vec = VecN<f64, 2>;
    type Vec2 = VecN<f64, 2>;
    type Trans = NdAffine<f64, 2>;
    type Rot = NdRotate<f64, 2>;
    type Poly = Polygon2d<f64>;
}

impl HalfEdgeImplMeshType for MeshType2d64PNUCurved {}
impl MeshTypeHalfEdge for MeshType2d64PNUCurved {}

impl CurvedEdge<2, MeshType2d64PNUCurved> for HalfEdgeImpl<MeshType2d64PNUCurved> {
    fn curve_type(&self) -> CurvedEdgeType<2, MeshType2d64PNUCurved> {
        self.payload().curve_type()
    }

    fn set_curve_type(&mut self, curve_type: CurvedEdgeType<2, MeshType2d64PNUCurved>) {
        self.payload_mut().set_curve_type(curve_type);
    }
}

/// A mesh with 2D vertices, usize indices, f64 positions and uv coordinates, and curved edges.
pub type Mesh2d64Curved = HalfEdgeMeshImpl<MeshType2d64PNUCurved>;

impl HalfEdgeMeshImpl<MeshType2d64PNUCurved> {
    /// Convert a Mesh2d64Curved to a MeshNd64 mesh.
    /// If there are curved edges they will be converted with the given tolerance.
    pub fn to_nd<const D: usize>(&self, tol: f64) -> MeshNd64<D> {
        MeshNd64::<D>::import_mesh::<_, _, _, _, MeshType2d64PNUCurved>(
            self.clone().flatten_curved_edges(tol),
            |vp| VertexPayloadPNU::<f64, D>::from_pos(Vector::from_xy(vp.pos().x, vp.pos().y)),
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

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use super::*;
    use crate::{extensions::nalgebra::Vec3, prelude::*};

    #[test]
    fn test_mesh2d64curved_construction() {
        let n = 100;
        let radius = 1.0;

        let mut mesh = Mesh2d64Curved::new();
        mesh.insert_regular_star(radius, radius, n);
        assert_eq!(mesh.num_vertices(), n);
        assert_eq!(mesh.num_edges(), 2 * n);
        assert_eq!(mesh.num_faces(), 1);
        assert!(mesh.has_consecutive_vertex_ids());
        assert!(mesh.is_open());
        assert!(mesh.check().is_ok());

        let m3d = mesh.to_nd::<3>(1.0);
        assert_eq!(m3d.num_vertices(), n);
        assert_eq!(m3d.num_edges(), 2 * n);
        assert_eq!(m3d.num_faces(), 1);
        assert!(m3d.has_consecutive_vertex_ids());
        assert!(m3d.is_open());
        assert!(m3d.check().is_ok());

        let f = m3d.faces().next().expect("no face");
        assert!(f.is_convex(&m3d));
        assert!(f.is_planar2(&m3d));
        //assert!(f.is_simple(&m3d));
        assert_eq!(f.normal(&m3d).normalize(), Vec3::from_xyz(0.0, 0.0, 1.0));
        let p = f.as_polygon(&m3d);
        assert!(
            (p.signed_area().abs() - (regular_polygon_area(radius, n))).abs()
                <= f64::EPSILON.sqrt()
        );

        let m10d = mesh.to_nd::<10>(1.0);
        assert!(m10d.check().is_ok());
    }
}
