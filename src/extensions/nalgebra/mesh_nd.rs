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

/*
impl<T: HalfEdgeImplMeshType<VP = BevyVertexPayload3d> + MeshType3D<Vec = Vec3, S = f32>>
    HalfEdgeMeshImpl<T>
{
    fn bevy_indices(&self, indices: &Vec<T::V>) -> bevy::render::mesh::Indices {
        if std::mem::size_of::<T::V>() == std::mem::size_of::<u32>() {
            bevy::render::mesh::Indices::U32(
                indices.into_iter().map(|x| x.index() as u32).collect(),
            )
        } else if std::mem::size_of::<T::V>() == std::mem::size_of::<u16>()
            || std::mem::size_of::<T::V>() == std::mem::size_of::<u8>()
        {
            bevy::render::mesh::Indices::U16(
                indices.into_iter().map(|x| x.index() as u16).collect(),
            )
        } else {
            panic!("Unsupported index type {}", std::mem::size_of::<T::V>());
        }
    }

    fn bevy_remove_attributes(mesh: &mut bevy::render::mesh::Mesh) {
        mesh.remove_indices();
        let mut attributes_to_remove = Vec::new();
        for (attr, _) in mesh.attributes() {
            attributes_to_remove.push(attr);
        }
        for attr in attributes_to_remove {
            mesh.remove_attribute(attr);
        }
    }

    /// Replace the mesh's attributes with the current mesh.
    /// Requires the mesh to be a triangle list and have the MAIN_WORLD usage.
    pub fn bevy_set(&self, mesh: &mut bevy::render::mesh::Mesh) {
        self.bevy_set_ex(
            mesh,
            TriangulationAlgorithm::Auto,
            false,
            &mut TesselationMeta::default(),
        );
    }

    /// Like bevy_set, but with additional meta information
    pub fn bevy_set_ex(
        &self,
        mesh: &mut bevy::render::mesh::Mesh,
        algo: TriangulationAlgorithm,
        generate_flat_normals: bool,
        meta: &mut TesselationMeta<T::V>,
    ) {
        assert!(mesh.primitive_topology() == PrimitiveTopology::TriangleList);
        assert!(mesh.asset_usage.contains(RenderAssetUsages::MAIN_WORLD));
        Self::bevy_remove_attributes(mesh);

        // use https://crates.io/crates/stats_alloc to measure memory usage
        //let now = std::time::Instant::now();
        let (is, vs) = if generate_flat_normals {
            self.triangulate_and_generate_flat_normals_post(algo, meta)
        } else {
            self.triangulate(algo, meta)
        };
        //let elapsed = now.elapsed();
        //println!("///////////////////\nTriangulation took {:.2?}", elapsed);

        mesh.insert_indices(self.bevy_indices(&is));
        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(
                vs.iter()
                    .map(|vp: &<MeshTypeNd64PNU as MeshType>::VP| vp.pos().to_array())
                    .collect(),
            ),
        );
        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(
                vs.iter()
                    .map(|vp| (vp as &BevyVertexPayload3d).normal().to_array())
                    .collect(),
            ),
        );
    }

    /// Convert the mesh to a bevy mesh
    pub fn to_bevy(&self, usage: RenderAssetUsages) -> bevy::render::mesh::Mesh {
        let mut mesh = bevy::render::mesh::Mesh::new(PrimitiveTopology::TriangleList, usage);
        self.bevy_set(&mut mesh);
        mesh
    }

    /// Convert the mesh to a bevy mesh with additional meta information
    pub fn to_bevy_ex(
        &self,
        usage: RenderAssetUsages,
        algo: TriangulationAlgorithm,
        generate_flat_normals: bool,
    ) -> bevy::render::mesh::Mesh {
        let mut mesh = bevy::render::mesh::Mesh::new(PrimitiveTopology::TriangleList, usage);
        self.bevy_set_ex(
            &mut mesh,
            algo,
            generate_flat_normals,
            &mut TesselationMeta::default(),
        );
        mesh
    }
}
*/
