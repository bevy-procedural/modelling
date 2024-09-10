use super::{Mesh, MeshType};
use crate::{
    math::Vector3D,
    representation::tesselate::{GenerateNormals, TesselationMeta, TriangulationAlgorithm},
};

impl<T: MeshType> Mesh<T> {
    /// convert the mesh to triangles and get all indices to do so.
    /// Also optionally duplicates vertices to insert normals etc. (otherwise, return empty vertices)
    pub fn tesselate(
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
            f.tesselate(
                self,
                &mut vertices,
                &mut indices,
                algorithm,
                generate_normals,
                meta,
            );
        }
        (indices, vertices)
    }
}
