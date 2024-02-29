use crate::representation::payload::Payload;

use super::super::{Face, IncidentToFaceIterator, IndexType, Mesh, Vertex};

impl<E: IndexType, F: IndexType> Face<E, F> {
    /// Iterates all half-edges incident to the face
    #[inline(always)]
    pub fn edges<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> IncidentToFaceIterator<'a, E, V, F, P> {
        IncidentToFaceIterator::new(self.edge(mesh), mesh)
    }

    /// Iterates all vertices adjacent to the face
    #[inline(always)]
    pub fn vertices<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = Vertex<E, V, P>> + 'a {
        self.edges(mesh).map(|e| e.target(mesh))
    }
}
