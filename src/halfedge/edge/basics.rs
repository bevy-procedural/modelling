use super::{BackwardEdgeIterator, ForwardEdgeIterator, HalfEdgeImpl, HalfEdgeImplMeshType};
use crate::mesh::{EdgeBasics, HalfEdge, MeshBasics};

impl<T: HalfEdgeImplMeshType> EdgeBasics<T> for HalfEdgeImpl<T> {
    /// Returns the index of the half-edge
    #[inline(always)]
    fn id(&self) -> T::E {
        self.id
    }

    /// Returns the source vertex of the half-edge
    #[inline(always)]
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.origin_id)
    }

    /// Returns the target vertex of the half-edge. Reached via the next half-edge, not the twin.
    #[inline(always)]
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex {
        mesh.vertex(self.next(mesh).origin_id())
    }

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    #[inline(always)]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.is_boundary_self() || self.twin(mesh).is_boundary_self()
    }

    /// Returns the face payload.
    #[inline(always)]
    fn payload(&self) -> &T::EP {
        &self.payload
    }

    /// Returns a mutable reference to the face payload.
    #[inline(always)]
    fn payload_mut(&mut self) -> &mut T::EP {
        &mut self.payload
    }

    /// Iterates all half-edges incident to the same face (counter-clockwise)
    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn edges_face<'a>(&'a self, mesh: &'a T::Mesh) -> ForwardEdgeIterator<'a, T> {
        ForwardEdgeIterator::new(self.clone(), mesh)
    }

    /// Iterates all half-edges incident to the same face (clockwise)
    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn edges_face_back<'a>(&'a self, mesh: &'a T::Mesh) -> BackwardEdgeIterator<'a, T> {
        BackwardEdgeIterator::new(self.clone(), mesh)
    }
}
