use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::{HalfEdgeImplMeshType, IncidentToFaceBackIterator, IncidentToFaceIterator},
    mesh::{HalfEdgeMesh, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMesh<T> for HalfEdgeMeshImpl<T> {
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_from<'a>(&'a self, e: T::E) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::<'a, T>::new(self.edge(e).clone(), self)
    }

    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_back_from<'a>(&'a self, e: T::E) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::<'a, T>::new(self.edge(e).clone(), self)
    }
}
