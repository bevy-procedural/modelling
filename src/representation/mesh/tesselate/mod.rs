use super::{IndexType, Mesh};
use crate::{math::Vector3D, representation::payload::Payload};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<P::S>,
{
    /// convert the mesh to triangles and get all indices to do so
    pub fn tesselate(&self) -> Vec<V> {
        let mut indices = Vec::new();
        for f in self.faces() {
            f.tesselate(self, &mut indices);
        }
        indices
    }

    /// convert the mesh to triangles and get all indices to do so.
    /// Also duplicates vertices to insert normals etc.
    pub fn tesselate_complete(&self) -> (Vec<V>, Vec<P>) {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        for f in self.faces() {
            f.tesselate_smooth_normal(self, &mut vertices, &mut indices);
        }
        (indices, vertices)
    }
}
