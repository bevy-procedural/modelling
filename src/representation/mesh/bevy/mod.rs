//! This module implements bevy specific mesh handling

use super::{Mesh, MeshType};
use crate::{
    math::{IndexType, Vector3D},
    representation::{
        payload::{bevy::BevyVertexPayload, VertexPayload},
        tesselate::{GenerateNormals, TesselationMeta, TriangulationAlgorithm},
        EmptyEdgePayload, EmptyFacePayload,
    },
};
use bevy::render::{
    mesh::{PrimitiveTopology, VertexAttributeValues},
    render_asset::RenderAssetUsages,
};
use std::time::Instant;

/// A mesh type for bevy with 3D vertices, 32 bit indices, 32 bit floats, and no face or edge payload (no normals etc.)
#[derive(Clone, Copy)]
pub struct BevyMeshType3d32;

impl MeshType for BevyMeshType3d32 {
    type E = u32;
    type V = u32;
    type F = u32;
    type EP = EmptyEdgePayload;
    type VP = BevyVertexPayload;
    type FP = EmptyFacePayload;
    type S = <BevyVertexPayload as VertexPayload>::S;
    type Vec = <BevyVertexPayload as VertexPayload>::Vec;
    type Vec2 = <BevyVertexPayload as VertexPayload>::Vec2;
    type Vec3 = <BevyVertexPayload as VertexPayload>::Vec3;
    type Trans = <BevyVertexPayload as VertexPayload>::Trans;
}

/// A mesh with bevy 3D vertices
pub type BevyMesh3d = Mesh<BevyMeshType3d32>;

impl<T: MeshType<VP = BevyVertexPayload, Vec: Vector3D<S = T::S>>> Mesh<T> {
    fn raw_vertices(&self) -> Vec<[f32; 3]> {
        self.vertices()
            .map(|v| v.payload().vertex().to_array())
            .collect()
    }

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
            GenerateNormals::Flat,
            &mut TesselationMeta::default(),
        );
    }

    /// Like bevy_set, but with additional meta information
    pub fn bevy_set_ex(
        &self,
        mesh: &mut bevy::render::mesh::Mesh,
        algo: TriangulationAlgorithm,
        normals: GenerateNormals,
        meta: &mut TesselationMeta<T::V>,
    ) {
        assert!(mesh.primitive_topology() == PrimitiveTopology::TriangleList);
        assert!(mesh.asset_usage.contains(RenderAssetUsages::MAIN_WORLD));
        Self::bevy_remove_attributes(mesh);

        // use https://crates.io/crates/stats_alloc to measure memory usage
        let now = Instant::now();
        let (is, mut vs) = self.tesselate(algo, normals, meta);
        let elapsed = now.elapsed();
        println!("///////////////////\nTriangulation took {:.2?}", elapsed);

        if vs.len() == 0 {
            vs = self.vertices().map(|v| v.payload()).cloned().collect();
        }

        mesh.insert_indices(self.bevy_indices(&is));

        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vs.iter().map(|vp| vp.vertex().to_array()).collect()),
        );
        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(vs.iter().map(|vp| vp.normal().to_array()).collect()),
        );

        // mesh.duplicate_vertices();
        // mesh.compute_flat_normals();
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
        normals: GenerateNormals,
    ) -> bevy::render::mesh::Mesh {
        let mut mesh = bevy::render::mesh::Mesh::new(PrimitiveTopology::TriangleList, usage);
        self.bevy_set_ex(&mut mesh, algo, normals, &mut TesselationMeta::default());
        mesh
    }
}
