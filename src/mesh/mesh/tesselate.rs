use std::collections::HashMap;

use super::{Mesh, MeshType};
use crate::{
    math::{IndexType, Vector3D},
    mesh::{
        payload::{HasPosition, VertexPayload},
        tesselate::{TesselationMeta, Triangulation, TriangulationAlgorithm},
    },
};

impl<T: MeshType> T::Mesh {
    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn get_compact_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP> {
        let mut vertices = Vec::with_capacity(self.num_vertices());

        if self.vertices.len() == self.vertices.capacity() {
            // Vertex buffer is already compact.
            // Since the index map creation is time consuming, we avoid this if possible.
            for _ in 0..self.vertices.capacity() {
                vertices.push(T::VP::allocate());
            }
            for v in self.vertices() {
                vertices[v.id().index()] = v.payload().clone();
            }
        } else {
            // Vertex buffer is sparse.
            // We need to create a map from the old indices to the new compact indices.
            let mut id_map = HashMap::new();
            for v in self.vertices() {
                id_map.insert(v.id(), T::V::new(vertices.len()));
                vertices.push(v.payload().clone());
            }
            Triangulation::new(indices).map_indices(&id_map);
        }

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
