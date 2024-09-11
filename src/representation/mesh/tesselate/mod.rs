use std::collections::HashMap;

use super::{Mesh, MeshType};
use crate::{
    math::{IndexType, Vector, Vector3D},
    representation::{
        payload::VertexPayload,
        tesselate::{GenerateNormals, TesselationMeta, Triangulation, TriangulationAlgorithm},
    },
};

impl<T: MeshType> Mesh<T> {
    /// convert the mesh to triangles and get all indices to do so.
    /// Also optionally duplicates vertices to insert normals etc. (otherwise, return empty vertices)
    pub fn triangulate(
        &self,
        algorithm: TriangulationAlgorithm,
        generate_normals: GenerateNormals,
        meta: &mut TesselationMeta<T::V>,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T::Vec: Vector3D<S = T::S>,
    {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        for f in self.faces() {
            f.triangulate(
                self,
                &mut vertices,
                &mut indices,
                algorithm,
                generate_normals,
                meta,
            );
        }

        match generate_normals {
            GenerateNormals::Smooth => {
                // Smooth normals are calculated without vertex duplication.
                // Hence, we have to set the normals of the whole mesh.
                // we copy the vertices still to both compact the indices and set the normals without mutating the mesh
                let face_normals: HashMap<T::F, T::Vec> =
                    self.faces().map(|f| (f.id(), f.normal(self))).collect();

                let mut id_map = HashMap::new();

                self.vertices().for_each(|v| {
                    id_map.insert(v.id(), <T::V as IndexType>::new(vertices.len()));

                    // set the average of face normals for each vertex
                    let vertex_normal =
                        T::Vec::mean(v.faces(self).map(|f| face_normals[&f.id()])).normalize();
                    let mut p = v.payload().clone();
                    p.set_normal(vertex_normal);
                    vertices.push(p);
                });

                Triangulation::new(&mut indices).map_indices(&id_map);
            }
            _ => {
                // other methods don't need any central calculation
            }
        }
        (indices, vertices)
    }
}
