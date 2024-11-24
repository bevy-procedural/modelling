#![allow(dead_code)]
#![doc = include_str!("../README.md")]
#![doc = include_str!("../doc/start.md")]

pub mod halfedge;
pub mod math;
pub mod mesh;
pub mod operations;
pub mod primitives;
pub mod tesselate;
pub mod util;

#[cfg(feature = "bevy")]
pub mod bevy;

/// A prelude for easy importing of commonly used types and traits.
pub mod prelude {
    pub use crate::halfedge::*;
    pub use crate::math::*;
    pub use crate::mesh::*;
    pub use crate::operations::*;
    pub use crate::primitives::*;
    pub use crate::tesselate::*;
    pub use crate::util::*;

    #[cfg(feature = "bevy")]
    pub use crate::bevy::*;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    #[cfg(feature = "bevy")]
    fn test_library() {
        let mut mesh = BevyMesh3d::geodesic_octahedron(3.0, 128);
        let mut meta = TesselationMeta::default();
        mesh.generate_smooth_normals();
        let (_is, _vs) = mesh.triangulate_and_generate_flat_normals_post(
            TriangulationAlgorithm::Delaunay,
            &mut meta,
        );
        // TODO: test something
    }
}
