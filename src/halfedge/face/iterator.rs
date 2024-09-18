use super::super::{HalfEdgeFace, IncidentToFaceIterator};
use crate::{
    halfedge::HalfEdgeMeshType,
    mesh::{Edge, FaceBasics},
};

impl<T: HalfEdgeMeshType> HalfEdgeFace<T> {
    /// Iterates all half-edges incident to the face
    #[inline(always)]
    pub fn edges<'a>(&'a self, mesh: &'a T::Mesh) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::new(self.edge(mesh), mesh)
    }

    /// Iterates all half-edge ids incident to the face
    pub fn edge_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::E> + 'a {
        self.edges(mesh).map(|e| e.id())
    }
}
