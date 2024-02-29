use super::{IndexType, Mesh};
use bevy::{
    math::Vec3,
    render::{
        mesh::{PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
};

impl<E, V, F> Mesh<E, V, F, Vec3>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
{
    fn raw_vertices(&self) -> Vec<[f32; 3]> {
        self.vertices().map(|v| v.payload().to_array()).collect()
    }

    fn bevy_indices(&self) -> bevy::render::mesh::Indices {
        let indices = self.tesselate();
        if std::mem::size_of::<V>() == std::mem::size_of::<u32>() {
            bevy::render::mesh::Indices::U32(
                indices.into_iter().map(|x| x.index() as u32).collect(),
            )
        } else if std::mem::size_of::<V>() == std::mem::size_of::<u16>()
            || std::mem::size_of::<V>() == std::mem::size_of::<u8>()
        {
            bevy::render::mesh::Indices::U16(
                indices.into_iter().map(|x| x.index() as u16).collect(),
            )
        } else {
            panic!("Unsupported index type {}", std::mem::size_of::<V>());
        }
    }

    /// Convert the mesh to a bevy mesh
    pub fn to_bevy(&self) -> bevy::render::mesh::Mesh {
        let mut mesh = bevy::render::mesh::Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::all(),
        );
        let vertices = self.raw_vertices();
        mesh.insert_indices(self.bevy_indices());
        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vertices),
        );
        mesh
    }
}
