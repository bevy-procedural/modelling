//mod check;
mod basics;
mod mesh_type;
mod normals;
mod payload;
mod transform;

pub use basics::*;
pub use mesh_type::*;
pub use normals::*;
pub use payload::*;
pub use transform::*;

use super::{Face, Face3d, Vertex};
use crate::{
    math::{HasPosition, Vector3D, VectorIteratorExt},
    tesselate::{triangulate_face, TesselationMeta, Triangulation, TriangulationAlgorithm},
};

/// The `Mesh` trait doesn't assume any specific data structure or topology.
pub trait Mesh<T: MeshType<Mesh = Self>>:
    basics::MeshBasics<T> + MeshNormals<T> + MeshTransforms<T>
{
    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices().map(|v| v.pos()).stable_mean()
    }

    /// convert the mesh to triangles and get all indices to do so.
    /// Compact the vertices and return the indices
    fn triangulate(
        &self,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<T::V>,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Face: Face3d<T>,
    {
        let mut indices = Vec::new();
        for f in self.faces() {
            let mut tri = Triangulation::new(&mut indices);
            triangulate_face::<T>(f, self, &mut tri, algorithm, meta)

            // TODO debug_assert!(tri.verify_full());
        }

        let vs = self.get_compact_vertices(&mut indices);
        (indices, vs)
    }
}
