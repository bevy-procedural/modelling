use super::{BevyVertexPayload3d, Polygon2dBevy};
use crate::{
    halfedge::{
        HalfEdgeFaceImpl, HalfEdgeImpl, HalfEdgeImplMeshType, HalfEdgeMeshImpl, HalfEdgeVertexImpl,
    },
    math::{HasNormal, HasPosition, IndexType},
    mesh::{
        EmptyEdgePayload, EmptyFacePayload, EmptyMeshPayload, EuclideanMeshType, MeshType,
        MeshType3D, MeshTypeHalfEdge, Triangulateable,
    },
    tesselate::{TesselationMeta, TriangulationAlgorithm},
};
use bevy::{
    asset::RenderAssetUsages, math::{Quat, Vec2, Vec3}, mesh::{Indices as BevyIndices, PrimitiveTopology, VertexAttributeValues}
};

/// A mesh type for bevy with
/// - 3D vertices,
/// - 32 bit indices,
/// - no face or edge payload,
/// - f32 vertex positions, normals, and uv coordinates
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BevyMeshType3d32;

impl MeshType for BevyMeshType3d32 {
    type E = u32;
    type V = u32;
    type F = u32;
    type EP = EmptyEdgePayload<Self>;
    type VP = BevyVertexPayload3d;
    type FP = EmptyFacePayload<Self>;
    type MP = EmptyMeshPayload<Self>;
    type Mesh = BevyMesh3d;
    type Face = HalfEdgeFaceImpl<Self>;
    type Edge = HalfEdgeImpl<Self>;
    type Vertex = HalfEdgeVertexImpl<Self>;
}
impl EuclideanMeshType<3> for BevyMeshType3d32 {
    type S = f32;
    type Vec = Vec3;
    type Vec2 = Vec2;
    type Trans = bevy::transform::components::Transform;
    type Rot = Quat;
    type Poly = Polygon2dBevy;
}
impl HalfEdgeImplMeshType for BevyMeshType3d32 {}
impl MeshTypeHalfEdge for BevyMeshType3d32 {}
impl MeshType3D for BevyMeshType3d32 {}

/// A mesh with bevy 3D vertices
pub type BevyMesh3d = HalfEdgeMeshImpl<BevyMeshType3d32>;

impl<T: HalfEdgeImplMeshType<VP = BevyVertexPayload3d> + MeshType3D<Vec = Vec3, S = f32>>
    HalfEdgeMeshImpl<T>
{
    fn bevy_indices(&self, indices: &Vec<T::V>) -> BevyIndices {
        if std::mem::size_of::<T::V>() == std::mem::size_of::<u32>() {
            BevyIndices::U32(indices.into_iter().map(|x| x.index() as u32).collect())
        } else if std::mem::size_of::<T::V>() == std::mem::size_of::<u16>()
            || std::mem::size_of::<T::V>() == std::mem::size_of::<u8>()
        {
            BevyIndices::U16(indices.into_iter().map(|x| x.index() as u16).collect())
        } else {
            panic!("Unsupported index type {}", std::mem::size_of::<T::V>());
        }
    }

    fn bevy_remove_attributes(mesh: &mut bevy::prelude::Mesh) {
        mesh.remove_indices();
        let mut attributes_to_remove = Vec::new();
        for (attr, _) in mesh.attributes() {
            attributes_to_remove.push(attr.clone());
        }
        for attr in attributes_to_remove {
            mesh.remove_attribute(attr);
        }
    }

    /// Replace the mesh's attributes with the current mesh.
    /// Requires the mesh to be a triangle list and have the MAIN_WORLD usage.
    pub fn bevy_set(&self, mesh: &mut bevy::prelude::Mesh) {
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
        mesh: &mut bevy::prelude::Mesh,
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
            bevy::prelude::Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(
                vs.iter()
                    .map(|vp: &<BevyMeshType3d32 as MeshType>::VP| vp.pos().to_array())
                    .collect(),
            ),
        );
        mesh.insert_attribute(
            bevy::prelude::Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(
                vs.iter()
                    .map(|vp| (vp as &BevyVertexPayload3d).normal().to_array())
                    .collect(),
            ),
        );
    }

    /// Convert the mesh to a bevy mesh
    pub fn to_bevy(&self, usage: RenderAssetUsages) -> bevy::prelude::Mesh {
        let mut mesh = bevy::prelude::Mesh::new(PrimitiveTopology::TriangleList, usage);
        self.bevy_set(&mut mesh);
        mesh
    }

    /// Convert the mesh to a bevy mesh with additional meta information
    pub fn to_bevy_ex(
        &self,
        usage: RenderAssetUsages,
        algo: TriangulationAlgorithm,
        generate_flat_normals: bool,
    ) -> bevy::prelude::Mesh {
        let mut mesh = bevy::prelude::Mesh::new(PrimitiveTopology::TriangleList, usage);
        self.bevy_set_ex(
            &mut mesh,
            algo,
            generate_flat_normals,
            &mut TesselationMeta::default(),
        );
        mesh
    }
}

#[cfg(feature = "nalgebra")]
impl From<&crate::extensions::nalgebra::Mesh3d64> for HalfEdgeMeshImpl<BevyMeshType3d32> {
    fn from(value: &crate::extensions::nalgebra::Mesh3d64) -> Self {
        BevyMesh3d::import_mesh::<_, _, _, _, crate::extensions::nalgebra::MeshType3d64PNU>(
            value,
            |vp| vp.into(),
            |_ep| EmptyEdgePayload::default(),
            |_fp| EmptyFacePayload::default(),
            |_mp| EmptyMeshPayload::default(),
        )
    }
}
