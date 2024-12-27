use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::{BackwardEdgeIterator, ForwardEdgeIterator, HalfEdgeImplMeshType},
    mesh::{HalfEdgeMesh, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMesh<T> for HalfEdgeMeshImpl<T> {
    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_from<'a>(&'a self, e: T::E) -> ForwardEdgeIterator<'a, T>
    where
        T: 'a,
    {
        ForwardEdgeIterator::<'a, T>::new(self.edge(e), self)
    }

    #[allow(refining_impl_trait)]
    #[inline(always)]
    fn edges_back_from<'a>(&'a self, e: T::E) -> BackwardEdgeIterator<'a, T>
    where
        T: 'a,
    {
        BackwardEdgeIterator::<'a, T>::new(self.edge(e), self)
    }
}
