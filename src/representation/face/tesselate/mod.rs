use super::{Face, Mesh, Payload};
use crate::representation::IndexType;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle fan
    pub fn tesselate<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) {
        // TODO: Only works for convex faces

        let center = self.vertices(mesh).next().unwrap();
        self.vertices(mesh)
            .skip(1)
            .map(|v| v.id())
            .collect::<Vec<V>>()
            .windows(2)
            .for_each(|w| {
                indices.push(center.id());
                indices.push(w[0]);
                indices.push(w[1]);
            });
    }
}
