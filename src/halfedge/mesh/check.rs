use super::HalfEdgeMeshImpl;
use crate::{
    halfedge::HalfEdgeImplMeshType,
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdgeMesh, MeshBasics, MeshChecker},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeMeshImpl<T> {
    /// Checks whether the twin of the twin is always the edge itself,
    /// the precursor to the next edge is the same, and the successor of the previous.
    fn check_edge_invariants(&self) -> Result<(), String> {
        for edge in self.halfedges() {
            edge.check()?;
        }

        Ok(())
    }

    fn check_vertex_invariants(&self) -> Result<(), String> {
        if let Some(bad_vertex) = self.vertices().find(|v| {
            if let Some(e) = v.fork().edge().try_inner() {
                e.origin_id(self) != v.id()
            } else {
                false
            }
        }) {
            return Err(format!(
                "Vertex {} has edge {} with origin {}",
                bad_vertex.id(),
                bad_vertex.clone().edge().id_unchecked(),
                bad_vertex
                    .edge()
                    .load_or(IndexType::max(), |v| v.origin_id())
            ));
        }

        Ok(())
    }

    fn check_edges_are_loops(&self) -> Result<(), String> {
        if let Some(bad_edge) = self
            .halfedges()
            .find(|e| e.fork().next().same_chain(e.origin_id()).is_none())
        {
            return Err(format!(
                "Successor of edge {} cannot reach it's origin {} during forward search",
                bad_edge.id(),
                bad_edge.origin_id()
            ));
        }

        if let Some(bad_edge) = self
            .halfedges()
            .find(|e| e.fork().prev().same_chain_back(e.target_id()).is_none())
        {
            return Err(format!(
                "Precursor of edge {} cannot reach it's target {} during backward search",
                bad_edge.id(),
                bad_edge.target_id()
            ));
        }

        Ok(())
    }

    fn check_faces_nondegenerate(&self) -> Result<(), String> {
        // TODO: this and many other checks would also work without half edges!
        // TODO: we should allow 2 faces when the mesh is allowed to be degenerate
        if let Some(bad_face) = self.faces().find(|f| f.num_edges() < 2) {
            return Err(format!(
                "Face {} has only {} edges!",
                bad_face.id(),
                bad_face.num_edges()
            ));
        }

        Ok(())
    }

    /// This is somewhat optional; the algorithms shouldn't break when using this, but there isn't really a reason for it existing in a wellformed mesh
    fn check_edges_have_face(&self) -> Result<(), String> {
        // TODO: unwrap
        if let Some(bad_edge) = self
            .halfedges()
            .find(|e| e.is_boundary_self() && e.fork().twin().unwrap().is_boundary_self())
        {
            return Err(format!("HalfEdge {} has no face!", bad_edge.id()));
        }
        Ok(())
    }

    fn check_face_invariants(&self) -> Result<(), String> {
        // TODO: unwrap
        if let Some(bad_face) = self
            .faces()
            .find(|f| f.fork().edge().unwrap().face_id() != f.id())
        {
            return Err(format!(
                "Face {} has edge {} with face {}",
                bad_face.id(),
                bad_face.fork().edge().id_unchecked(),
                bad_face.edge().unwrap().face_id()
            ));
        }
        Ok(())
    }

    fn vertex_analysis(&self) -> String {
        self.vertex_refs()
            .map(|v| format!("{:?}", v))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn edge_analysis(&self) -> String {
        format!(
            "\n edge --><-- twin   |  face: edge/twin \n{}",
            self.pair_edges()
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn face_analysis(&self) -> String {
        self.faces()
            .map(|f| {
                format!(
                    "{}) {} edges, e.g., {}   {:?}",
                    f.id(),
                    f.num_edges(),
                    f.edge_id(),
                    f.payload(), //f.is_planar(mesh, T::S::EPS.sqrt())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
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
        //self.check_edges_have_face()?;
        // TODO: check_faces_planar
        // TODO: check_faces_convex
        // TODO: check_faces_oriented
        // TODO: check for references to deleted items
        Ok(())
    }
}

impl<T: HalfEdgeImplMeshType> std::fmt::Debug for HalfEdgeMeshImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "vertices:\n{}\n{}\nfaces:\n{}\n{}",
            self.vertex_analysis(),
            self.edge_analysis(),
            self.face_analysis(),
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
