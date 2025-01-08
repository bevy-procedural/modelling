use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::HalfEdgeImplMeshType,
    mesh::{EdgeBasics, FaceBasics, HalfEdge, MeshBasics, MeshChecker, VertexBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
    /// Checks whether the twin of the twin is always the edge itself,
    /// the precursor to the next edge is the same, and the successor of the previous.
    fn check_edge_invariants(&self) -> Result<(), String> {
        for edge in self.edges() {
            edge.validate(self)?;
        }

        Ok(())
    }

    fn check_vertex_invariants(&self) -> Result<(), String> {
        if let Some(bad_vertex) = self.vertices().find(|v| {
            if let Some(e) = v.edge(self) {
                e.origin_id(self) != v.id()
            } else {
                false
            }
        }) {
            return Err(format!(
                "Vertex {} has edge {} with origin {}",
                bad_vertex.id(),
                bad_vertex.edge(self).unwrap().id(),
                bad_vertex.edge(self).unwrap().origin_id(self)
            ));
        }

        Ok(())
    }

    fn check_edges_are_loops(&self) -> Result<(), String> {
        if let Some(bad_edge) = self
            .edges()
            .find(|e| e.next(self).same_boundary(self, e.origin_id(self)).is_none())
        {
            return Err(format!(
                "Successor of edge {} cannot reach it's origin {} during forward search",
                bad_edge.id(),
                bad_edge.origin_id(self)
            ));
        }

        if let Some(bad_edge) = self.edges().find(|e| {
            e.prev(self)
                .same_boundary_back(self, e.target_id(self))
                .is_none()
        }) {
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
