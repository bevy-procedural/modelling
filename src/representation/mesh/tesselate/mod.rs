use super::{IndexType, Mesh};
use crate::representation::payload::{Payload, Vector3D};

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
}
