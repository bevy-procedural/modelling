use std::collections::HashMap;

use super::{Mesh, MeshType};
use crate::{
    math::{IndexType, Vector3D},
    representation::{
        payload::HasPosition,
        tesselate::{TesselationMeta, Triangulation, TriangulationAlgorithm},
    },
};

impl<T: MeshType> Mesh<T> {
    /// Since the indices are not necessarily in order,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new dense vertex buffer.
    fn get_compact_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP> {
        let mut id_map = HashMap::new();
        let mut vertices = Vec::with_capacity(self.num_vertices());
        for v in self.vertices() {
            id_map.insert(v.id(), T::V::new(vertices.len()));
            vertices.push(v.payload().clone());
        }
        Triangulation::new(indices).map_indices(&id_map);
        vertices
    }

    /// convert the mesh to triangles and get all indices to do so.
    /// Compact the vertices and return the indices
    pub fn triangulate(
        &self,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<T::V>,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        let mut indices = Vec::new();
        for f in self.faces() {
            let mut tri = Triangulation::new(&mut indices);
            f.triangulate(self, &mut tri, algorithm, meta);
            // TODO debug_assert!(tri.verify_full());
        }

        let vs = self.get_compact_vertices(&mut indices);
        (indices, vs)
    }
}
