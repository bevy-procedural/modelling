use crate::representation::MeshType;

use super::{
    super::{Face, IncidentToFaceIterator, IndexType, Mesh, Vertex},
    FacePayload,
};

impl<E: IndexType, F: IndexType, FP: FacePayload> Face<E, F, FP> {
    /// Iterates all half-edges incident to the face
    #[inline(always)]
    pub fn edges<'a, T: MeshType<E = E, F = F, FP = FP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::new(self.edge(mesh), mesh)
    }

    /// Iterates all vertices adjacent to the face
    #[inline(always)]
    pub fn vertices<'a, T: MeshType<E = E, F = F, FP = FP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = Vertex<E, T::V, T::VP>> + 'a + Clone + ExactSizeIterator {
        self.edges(mesh).map(|e| e.target(mesh))
    }
}
