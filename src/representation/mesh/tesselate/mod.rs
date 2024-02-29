use super::{IndexType, Mesh};
use crate::representation::payload::Payload;

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
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
