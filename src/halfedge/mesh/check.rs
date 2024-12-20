use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::HalfEdgeImplMeshType,
    mesh::{EdgeBasics, FaceBasics, HalfEdge, MeshBasics, MeshChecker, VertexBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
    /// Checks whether the twin of the twin is always the edge itself,
    /// the precursor to the next edge is the same, and the successor of the previous.
    fn check_edge_invariants(&self) -> Result<(), String> {
        if let Some(unmatched_twin) = self.edges().find(|e| e.twin(self).twin_id() != e.id()) {
            return Err(format!(
                "HalfEdge {} has a twin {} with twin {}",
                unmatched_twin.id(),
                unmatched_twin.twin_id(),
                unmatched_twin.twin(self).twin_id()
            ));
        }

        if let Some(prev_next) = self
            .edges()
            .find(|e| e.next(self).prev(self).id() != e.id() || e.prev(self).next_id() != e.id())
        {
            return Err(format!(
                "HalfEdge {} has prev(next) {} and next(prev) {}",
                prev_next.id(),
                prev_next.next(self).prev_id(),
                prev_next.prev(self).next_id()
            ));
        }

        if let Some(face_next) = self.edges().find(|e| {
            let f1 = e.face(self);
            let f2 = e.next(self).face(self).cloned();
            (f1.is_none() ^ f2.is_none())
                || (f1.is_some() && f2.is_some() && f1.unwrap().id() != f2.unwrap().id())
        }) {
            return Err(format!(
                "HalfEdge {} has face {} but next has face {}",
                face_next.id(),
                face_next.face_id(),
                face_next.next(self).face_id()
            ));
        }

        if let Some(bad_edge) = self.edges().find(|e| {
            e.next(self).origin_id() != e.twin(self).origin_id()
                || e.target_id(self) != e.twin(self).origin_id()
        }) {
            return Err(format!(
                "HalfEdge {} has next origin {} and target {} but twin origin {}",
                bad_edge.id(),
                bad_edge.next(self).origin_id(),
                bad_edge.target_id(self),
                bad_edge.twin(self).origin_id()
            ));
        }

        Ok(())
    }

    fn check_vertex_invariants(&self) -> Result<(), String> {
        if let Some(bad_vertex) = self.vertices().find(|v| {
            if let Some(e) = v.edge(self) {
                e.origin_id() != v.id()
            } else {
                false
            }
        }) {
            return Err(format!(
                "Vertex {} has edge {} with origin {}",
                bad_vertex.id(),
                bad_vertex.edge(self).unwrap().id(),
                bad_vertex.edge(self).unwrap().origin_id()
            ));
        }

        Ok(())
    }

    fn check_edges_are_loops(&self) -> Result<(), String> {
        if let Some(bad_edge) = self
            .edges()
            .find(|e| !e.next(self).same_face(self, e.origin_id()))
        {
            return Err(format!(
                "Successor of edge {} cannot reach it's origin {} during forward search",
                bad_edge.id(),
                bad_edge.origin_id()
            ));
        }

        if let Some(bad_edge) = self
            .edges()
            .find(|e| !e.prev(self).same_face_back(self, e.target_id(self)))
        {
            return Err(format!(
                "Precursor of edge {} cannot reach it's target {} during backward search",
                bad_edge.id(),
                bad_edge.target_id(self)
            ));
        }

        Ok(())
    }

    fn check_faces_nondegenerate(&self) -> Result<(), String> {
        // TODO: this and many other checks would also work without half edges!
        if let Some(bad_face) = self.faces().find(|f| f.edges(self).count() < 3) {
            return Err(format!(
                "Face {} has only {} faces!",
                bad_face.id(),
                bad_face.edges(self).count()
            ));
        }

        Ok(())
    }

    /// This is somewhat optional; the algorithms shouldn't break when using this, but there isn't really a reason for it existing in a wellformed mesh
    fn check_edges_have_face(&self) -> Result<(), String> {
        if let Some(bad_edge) = self
            .edges()
            .find(|e| e.is_boundary_self() && e.twin(self).is_boundary_self())
        {
            return Err(format!("HalfEdge {} has no face!", bad_edge.id()));
        }
        Ok(())
    }

    fn check_face_invariants(&self) -> Result<(), String> {
        if let Some(bad_face) = self.faces().find(|f| f.edge(self).face_id() != f.id()) {
            return Err(format!(
                "Face {} has edge {} with face {}",
                bad_face.id(),
                bad_face.edge(self).id(),
                bad_face.edge(self).face_id()
            ));
        }
        Ok(())
    }
}

impl<T: HalfEdgeImplMeshType> MeshChecker<T> for HalfEdgeMeshImpl<T> {
    /// Checks the mesh for consistency
    fn check(&self) -> Result<(), String> {
        self.check_edge_invariants()?;
        self.check_face_invariants()?;
        self.check_vertex_invariants()?;
        self.check_edges_are_loops()?;
        self.check_faces_nondegenerate()?;
        self.check_edges_have_face()?;
        // TODO: check_faces_planar
        // TODO: check_faces_convex
        // TODO: check_faces_oriented
        // TODO: check for references to delete items
        Ok(())
    }
}

impl<T: HalfEdgeImplMeshType> std::fmt::Debug for HalfEdgeMeshImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mesh:\nvertices:\n{}\n edge --><-- twin   |  face: edge/twin \n{}\n faces: \n{}\n{} ",
            self.vertices()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pair_edges()
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.faces()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            if let Err(msg) = self.check() {
                format!(
                    "⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ERROR ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️\n{}",
                    msg
                )
            } else {
                "".to_string()
            }
        )
    }
}
