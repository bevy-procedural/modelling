use super::HalfEdgeImpl;
use crate::{
    halfedge::{HalfEdgeMeshImpl, HalfEdgeImplMeshType},
    math::IndexType,
    mesh::{HalfEdge, HalfEdgeVertex, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdge<T> for HalfEdgeImpl<T> {
    fn new(next: T::E, twin: T::E, prev: T::E, origin: T::V, face: T::F, payload: T::EP) -> Self {
        assert!(next != IndexType::max());
        assert!(prev != IndexType::max());
        assert!(twin != IndexType::max());
        Self {
            id: IndexType::max(),
            next,
            twin,
            prev,
            origin_id: origin,
            face,
            payload,
        }
    }

    fn set_face(&mut self, face: T::F) {
        debug_assert!(self.face == IndexType::max());
        self.face = face;
    }

    fn delete_face(&mut self) {
        debug_assert!(self.face != IndexType::max());
        self.face = IndexType::max();
    }

    fn set_next(&mut self, next: T::E) {
        self.next = next;
    }

    fn set_prev(&mut self, prev: T::E) {
        self.prev = prev;
    }

    fn set_twin(&mut self, twin: T::E) {
        self.twin = twin;
    }

    #[inline(always)]
    fn next(&self, mesh: &HalfEdgeMeshImpl<T>) -> HalfEdgeImpl<T> {
        *mesh.edge(self.next)
    }

    #[inline(always)]
    fn next_id(&self) -> T::E {
        self.next
    }

    #[inline(always)]
    fn twin(&self, mesh: &HalfEdgeMeshImpl<T>) -> HalfEdgeImpl<T> {
        // TODO: Make this return a reference?
        *mesh.edge(self.twin)
    }

    #[inline(always)]
    fn twin_id(&self) -> T::E {
        self.twin
    }

    #[inline(always)]
    fn prev(&self, mesh: &HalfEdgeMeshImpl<T>) -> HalfEdgeImpl<T> {
        *mesh.edge(self.prev)
    }

    #[inline(always)]
    fn prev_id(&self) -> T::E {
        self.prev
    }

    #[inline(always)]
    fn origin_id(&self) -> T::V {
        self.origin_id
    }

    #[inline(always)]
    fn target_id(&self, mesh: &HalfEdgeMeshImpl<T>) -> T::V {
        self.next(mesh).origin_id()
    }

    #[inline(always)]
    fn face<'a>(&'a self, mesh: &'a HalfEdgeMeshImpl<T>) -> Option<&'a T::Face> {
        if self.face == IndexType::max() {
            None
        } else {
            Some(mesh.face(self.face))
        }
    }

    #[inline(always)]
    fn face_id(&self) -> T::F {
        self.face
    }

    #[inline(always)]
    fn other_face<'a>(&'a self, mesh: &'a HalfEdgeMeshImpl<T>) -> Option<&'a T::Face> {
        let face = self.twin(mesh).face_id();
        if face == IndexType::max() {
            None
        } else {
            Some(mesh.face(face))
        }
    }

    #[inline(always)]
    fn is_boundary_self(&self) -> bool {
        self.face == IndexType::max()
    }

    fn same_face(&self, mesh: &HalfEdgeMeshImpl<T>, v: T::V) -> bool {
        self.edges_face(mesh).find(|e| e.origin_id() == v).is_some()
    }

    fn same_face_back(&self, mesh: &HalfEdgeMeshImpl<T>, v: T::V) -> bool {
        self.edges_face_back(mesh)
            .find(|e| e.origin_id() == v)
            .is_some()
    }

    fn flip(e: T::E, mesh: &mut HalfEdgeMeshImpl<T>) {
        let origin = mesh.edge(e).origin_id();
        let target = mesh.edge(e).target_id(mesh);

        let twin_id = {
            let edge = mesh.edge_mut(e);
            let tmp = edge.next;
            edge.next = edge.prev;
            edge.prev = tmp;
            edge.origin_id = target;
            edge.twin_id()
        };
        {
            let twin = mesh.edge_mut(twin_id);
            let tmp = twin.next;
            twin.next = twin.prev;
            twin.prev = tmp;
            twin.origin_id = origin;
        }
        let o = mesh.vertex_mut(origin);
        o.set_edge(twin_id);
        let t = mesh.vertex_mut(target);
        t.set_edge(e);
    }
}
