use super::{BackwardEdgeIterator, ForwardEdgeIterator, HalfEdgeImpl, HalfEdgeImplMeshType};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, EdgePayload, HalfEdge, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> EdgeBasics<T> for HalfEdgeImpl<T> {
    #[inline]
    fn id(&self) -> T::E {
        self.id
    }

    #[inline]
    fn payload_self_empty(&self) -> bool {
        self.payload.is_none() || self.payload.as_ref().map(|x| x.is_empty()).unwrap()
    }

    #[inline]
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.origin_id(mesh))
    }

    #[inline]
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.target_id(mesh))
    }

    #[inline]
    fn origin_id(&self, _mesh: &T::Mesh) -> T::V {
        self.origin_id
    }

    #[inline]
    fn target_id(&self, mesh: &T::Mesh) -> T::V {
        self.next(mesh).origin_id(mesh)
    }

    #[inline]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.is_boundary_self() || self.twin(mesh).is_boundary_self()
    }

    #[inline]
    fn payload_self(&self) -> Option<&T::EP> {
        self.payload.as_ref()
    }

    #[inline]
    fn payload_self_mut(&mut self) -> Option<&mut T::EP> {
        self.payload.as_mut()
    }

    #[inline]
    fn edges_face<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge> + 'a
    where
        T: 'a,
    {
        ForwardEdgeIterator::<'a, T>::new(self, mesh)
    }

    #[inline]
    #[allow(refining_impl_trait)]
    fn edges_face_back<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge> + 'a
    where
        T: 'a,
    {
        BackwardEdgeIterator::new(self, mesh)
    }

    #[inline]
    fn face_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::F> + 'a {
        // TODO: only works for manifold meshes
        let mut res = Vec::new();
        let id = self.face_id();
        if id != IndexType::max() {
            res.push(id);
        }
        let twin = self.twin(mesh);
        let id = twin.face_id();
        if id != IndexType::max() {
            res.push(id);
        }
        res.into_iter()
    }
}
