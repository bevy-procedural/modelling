use std::collections::HashMap;

use super::{Mesh, MeshType};
use crate::mesh::IndexType;

/// A pseudo-winged edge representation of an edge for debugging purposes
#[derive(Clone)]
struct PseudoWingedEdge<E, V, F>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
{
    id: E,
    twin: E,
    origin: V,
    target: V,
    prev: E,
    face: F,
    next: E,
    twin_prev: E,
    twin_face: F,
    twin_next: E,
}

impl<E: IndexType, V: IndexType, F: IndexType> std::fmt::Display for PseudoWingedEdge<E, V, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{: >w$} -{:->w$}--><--{:-<w$}- {: <w$}  |  face: {: >w$} / {: <w$} [{: <w$}]/[{: <w$}] {: >w$} / {: <w$}",
            self.origin,
            self.id,
            self.twin,
            self.target,
            self.prev,
            self.twin_next,
            if self.face == IndexType::max() {
                "na".to_string()
            } else {
                self.face.index().to_string()
            },
            if self.twin_face == IndexType::max() {
                "na".to_string()
            } else {
                self.twin_face.index().to_string()
            },
            self.next,
            self.twin_prev,
            w = 2,
        )
    }
}

impl<T: MeshType> Mesh<T> {
    /// Checks whether the twin of the twin is always the edge itself,
    /// the precursor to the next edge is the same, and the successor of the previous.
    fn check_edge_invariants(&self) -> Result<(), String> {
        if let Some(unmatched_twin) = self.halfedges().find(|e| e.twin(self).twin_id() != e.id()) {
            return Err(format!(
                "HalfEdge {} has a twin {} with twin {}",
                unmatched_twin.id(),
                unmatched_twin.twin_id(),
                unmatched_twin.twin(self).twin_id()
            ));
        }

        if let Some(prev_next) = self
            .halfedges()
            .find(|e| e.next(self).prev(self).id() != e.id() || e.prev(self).next_id() != e.id())
        {
            return Err(format!(
                "HalfEdge {} has prev(next) {} and next(prev) {}",
                prev_next.id(),
                prev_next.next(self).prev_id(),
                prev_next.prev(self).next_id()
            ));
        }

        if let Some(face_next) = self.halfedges().find(|e| {
            let f1 = e.face(self);
            let f2 = e.next(self).face(self);
            (f1.is_none() ^ f2.is_none())
                || (f1.is_some() && f2.is_some() && f1.unwrap().id() != f2.unwrap().id())
        }) {
            return Err(format!(
                "HalfEdge {} has face {:?} but next has face {:?}",
                face_next.id(),
                face_next.face(self),
                face_next.next(self).face(self)
            ));
        }

        if let Some(bad_edge) = self.halfedges().find(|e| {
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
        if let Some(bad_vertex) = self.vertices().find(|v| v.edge(self).origin_id() != v.id()) {
            return Err(format!(
                "Vertex {} has edge {} with origin {}",
                bad_vertex.id(),
                bad_vertex.edge(self).id(),
                bad_vertex.edge(self).origin_id()
            ));
        }

        Ok(())
    }

    fn check_edges_are_loops(&self) -> Result<(), String> {
        if let Some(bad_edge) = self
            .halfedges()
            .find(|e| !e.next(self).same_face(self, e.origin_id()))
        {
            return Err(format!(
                "Successor of edge {} cannot reach it's origin {} during forward search",
                bad_edge.id(),
                bad_edge.origin_id()
            ));
        }

        if let Some(bad_edge) = self
            .halfedges()
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
            .halfedges()
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

    /// Returns all edges as pseudo-winged edges
    fn pair_edges(&self) -> Vec<PseudoWingedEdge<T::E, T::V, T::F>> {
        let mut edges: HashMap<T::E, PseudoWingedEdge<T::E, T::V, T::F>> = HashMap::new();
        self.halfedges().for_each(|edge| {
            let twin = edge.twin(self);
            if edges.contains_key(&twin.id()) {
                return;
            }
            edges.insert(
                edge.id(),
                PseudoWingedEdge {
                    id: edge.id(),
                    twin: twin.id(),
                    origin: edge.origin_id(),
                    target: twin.origin_id(),
                    prev: edge.prev_id(),
                    face: edge.face_id(),
                    next: edge.next_id(),
                    twin_prev: twin.prev_id(),
                    twin_face: twin.face_id(),
                    twin_next: twin.next_id(),
                },
            );
        });

        let mut vec: Vec<PseudoWingedEdge<T::E, T::V, T::F>> = edges.values().cloned().collect();
        vec.sort_by(|a, b| a.id.cmp(&b.id));
        vec
    }

    /// Checks whether there is anything fishy about the mesh
    pub fn check(&self) -> Result<(), String> {
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
