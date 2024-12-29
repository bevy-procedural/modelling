use itertools::Itertools;

use super::HalfEdgeImpl;
use crate::{
    halfedge::{HalfEdgeImplMeshType, HalfEdgeMeshImpl},
    math::IndexType,
    mesh::{FaceBasics, HalfEdge, HalfEdgeVertex, MeshBasics, VertexBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdge<T> for HalfEdgeImpl<T> {
    fn new(
        next: T::E,
        twin: T::E,
        prev: T::E,
        origin: T::V,
        face: T::F,
        payload: Option<T::EP>,
    ) -> Self {
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

    fn set_origin(&mut self, origin: T::V) {
        self.origin_id = origin;
    }

    #[inline(always)]
    fn next<'a>(&self, mesh: &'a HalfEdgeMeshImpl<T>) -> &'a HalfEdgeImpl<T> {
        mesh.edge(self.next)
    }

    #[inline(always)]
    fn next_id(&self) -> T::E {
        self.next
    }

    #[inline(always)]
    fn twin<'a>(&self, mesh: &'a HalfEdgeMeshImpl<T>) -> &'a HalfEdgeImpl<T> {
        mesh.edge(self.twin)
    }

    #[inline(always)]
    fn twin_id(&self) -> T::E {
        self.twin
    }

    #[inline(always)]
    fn prev<'a>(&self, mesh: &'a HalfEdgeMeshImpl<T>) -> &'a HalfEdgeImpl<T> {
        mesh.edge(self.prev)
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

    fn flip(e: T::E, mesh: &mut HalfEdgeMeshImpl<T>) {
        let origin_id = mesh.edge(e).origin_id();
        let target_id = mesh.edge(e).target_id(mesh);

        let edge = mesh.edge(e);
        let next_id = edge.next;
        let prev_id = edge.prev;
        let face_id = edge.face_id();
        let twin_id = edge.twin_id();

        let twin = mesh.edge(twin_id);
        let t_next_id = twin.next;
        let t_prev_id = twin.prev;
        let t_face_id = twin.face_id();

        let edge = mesh.edge_mut(e);
        edge.next = t_next_id;
        edge.prev = t_prev_id;
        edge.face = t_face_id;
        edge.origin_id = target_id;
        mesh.edge_mut(t_next_id).prev = e;
        mesh.edge_mut(t_prev_id).next = e;

        let twin = mesh.edge_mut(twin_id);
        twin.next = next_id;
        twin.prev = prev_id;
        twin.face = face_id;
        twin.origin_id = origin_id;
        mesh.edge_mut(next_id).prev = twin_id;
        mesh.edge_mut(prev_id).next = twin_id;

        mesh.vertex_mut(origin_id).set_edge(twin_id);
        mesh.vertex_mut(target_id).set_edge(e);
        if face_id != IndexType::max() {
            mesh.face_mut(face_id).set_edge(twin_id);
        }
        if t_face_id != IndexType::max() {
            mesh.face_mut(t_face_id).set_edge(e);
        }
    }

    fn is_valid(&self, mesh: &T::Mesh) -> Result<(), String> {
        let oi = self.origin_id();
        let ti = self.target_id(mesh);
        let prev = self.prev(mesh);
        let next = self.next(mesh);
        let twin = self.twin(mesh);
        let id = self.id;
        if next.prev_id() != id {
            return Err(format!("prev(next) = {} != {} = id", next.prev_id(), id));
        }
        if prev.next_id() != id {
            return Err(format!("next(prev) = {} != {} = id", prev.next_id(), id));
        }
        if twin.twin_id() != id {
            return Err(format!("twin(twin) = {} != {} = id", twin.twin_id(), id));
        }
        if self.next_id() == id || self.prev_id() == id {
            return Err(format!(
                "Trivial self-loop in half-edge prev {} id {} next {}",
                self.prev_id(),
                id,
                self.next_id()
            ));
        }
        if oi == IndexType::max() {
            return Err("Origin vertex is invalid".to_string());
        }
        if ti == IndexType::max() {
            return Err("Target vertex is invalid".to_string());
        }
        if self.face_id() != IndexType::max() {
            if !mesh.has_face(self.face_id()) {
                return Err(format!(
                    "Face {} is defined but doesn't exist",
                    self.face_id()
                ));
            }
        }
        if next.face_id() != self.face_id() {
            return Err(format!(
                "Next edge {} has different face {}",
                self.next_id(),
                next.face_id()
            ));
        }
        if prev.face_id() != self.face_id() {
            return Err(format!(
                "Prev edge {} has different face {}",
                self.prev_id(),
                prev.face_id()
            ));
        }
        if !mesh.has_vertex(oi) {
            return Err(format!("Origin vertex {} doesn't exist", oi));
        }
        if !mesh.has_vertex(ti) {
            return Err(format!("Target vertex {} doesn't exist", ti));
        }
        if oi == ti {
            return Err(format!("Origin and target vertices are the same {}", oi));
        }
        if !mesh.vertex(oi).edges_out(mesh).any(|e| e.id == id) {
            return Err(format!("Origin vertex {} doesn't have edge {}", oi, id));
        }
        if !mesh.vertex(ti).edges_in(mesh).any(|e| e.id == id) {
            return Err(format!("Target vertex {} doesn't have edge {}", ti, id));
        }
        Ok(())
    }
}
