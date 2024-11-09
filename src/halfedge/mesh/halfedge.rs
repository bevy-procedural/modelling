use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::{HalfEdgeMeshType, IncidentToFaceBackIterator, IncidentToFaceIterator},
    mesh::{HalfEdgeMesh, MeshBasics},
};

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> for HalfEdgeMeshImpl<T> {
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_from<'a>(&'a self, e: T::E) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::<'a, T>::new(*self.edge(e), self)
    }

    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_back_from<'a>(&'a self, e: T::E) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::<'a, T>::new(*self.edge(e), self)
    }
}
