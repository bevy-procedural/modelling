use super::{IndexType, Mesh};
use crate::{
    math::Vector3D,
    representation::{
        payload::Payload,
        tesselate::{GenerateNormals, TriangulationAlgorithm},
    },
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<P::S>,
{
    /// convert the mesh to triangles and get all indices to do so.
    /// Also optionally duplicates vertices to insert normals etc. (otherwise, return empty vertices)
    pub fn tesselate(
        &self,
        algorithm: TriangulationAlgorithm,
        generate_normals: GenerateNormals,
    ) -> (Vec<V>, Vec<P>) {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        for f in self.faces() {
            f.tesselate(
                self,
                &mut vertices,
                &mut indices,
                algorithm,
                generate_normals,
            );
        }
        (indices, vertices)
    }
}
