use std::collections::HashMap;

use crate::{
    halfedge::HalfEdgeMeshType,
    math::IndexType,
    mesh::{EdgeBasics, Halfedge, MeshBasics},
};

use super::HalfEdgeMesh;

/// A pseudo-winged edge representation of an edge for debugging purposes
#[derive(Clone)]
pub(crate) struct PseudoWingedEdge<E, V, F>
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

impl<E: IndexType, V: IndexType, F: IndexType> std::fmt::Debug for PseudoWingedEdge<E, V, F> {
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

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Returns all edges as pseudo-winged edges
    pub(crate) fn pair_edges(&self) -> Vec<PseudoWingedEdge<T::E, T::V, T::F>> {
        let mut edges: HashMap<T::E, PseudoWingedEdge<T::E, T::V, T::F>> = HashMap::new();
        self.edges().for_each(|edge| {
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
}
