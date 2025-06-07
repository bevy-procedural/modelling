#![allow(dead_code)]
#![doc = include_str!("../README.md")]
#![doc = include_str!("../doc/start.md")]
#![doc = include_str!("../doc/cursors.md")]

pub mod extensions;
pub mod halfedge;
pub mod math;
pub mod mesh;
pub mod operations;
pub mod primitives;
pub mod tesselate;
pub mod util;

/// A prelude for easy importing of commonly used types and traits.
pub mod prelude {
    pub use crate::halfedge::*;
    pub use crate::math::*;
    pub use crate::mesh::{cursor::*, *};
    pub use crate::operations::*;
    pub use crate::primitives::*;
    pub use crate::tesselate::*;
    pub use crate::util::*;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    #[cfg(feature = "bevy")]
    fn test_library_bevy() {
        use crate::extensions::bevy::*;

        let mut mesh = BevyMesh3d::cube(1.0);
        mesh.generate_smooth_normals();
        let (_is, _vs) =
            mesh.triangulate_and_generate_flat_normals_post(TriangulationAlgorithm::Delaunay);
        // TODO: test something
    }

    #[test]
    #[cfg(feature = "nalgebra")]
    fn test_library_nalgebra() {
        use crate::extensions::nalgebra::*;

        let mut mesh = Mesh3d64::cube(1.0);
        mesh.generate_smooth_normals();
        let (_is, _vs) =
            mesh.triangulate_and_generate_flat_normals_post(TriangulationAlgorithm::Delaunay);
        // TODO: test something
    }
}