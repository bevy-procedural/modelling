use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::{HalfEdgeImplMeshType, BackwardEdgeIterator, ForwardEdgeIterator},
    mesh::{HalfEdgeMesh, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMesh<T> for HalfEdgeMeshImpl<T> {
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_from<'a>(&'a self, e: T::E) -> ForwardEdgeIterator<'a, T> {
        ForwardEdgeIterator::<'a, T>::new(self.edge(e).clone(), self)
    }

    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_back_from<'a>(&'a self, e: T::E) -> BackwardEdgeIterator<'a, T> {
        BackwardEdgeIterator::<'a, T>::new(self.edge(e).clone(), self)
    }
}
