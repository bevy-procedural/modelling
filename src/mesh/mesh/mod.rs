//mod check;
mod basics;
mod builder;
mod check;
mod mesh_type;
mod normals;
mod payload;
mod topology;
mod transform;

pub use basics::*;
pub use builder::*;
pub use check::*;
pub use mesh_type::*;
pub use normals::*;
pub use payload::*;
pub use topology::*;
pub use transform::*;

use super::{Face3d, Triangulation, Vertex, VertexBasics};
use crate::{
    math::{HasPosition, Vector3D, VectorIteratorExt},
    tesselate::{triangulate_face, TesselationMeta, TriangulationAlgorithm},
};

/// The `MeshTrait` doesn't assume any specific data structure or topology,
/// i.e., could be a manifold half-edge mesh, a topological directed graph, etc.
pub trait MeshTrait<T: MeshType<Mesh = Self>>:
    basics::MeshBasics<T> + MeshNormals<T> + MeshTransforms<T> + MeshTopology<T>
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
