use std::collections::HashMap;

use super::Mesh;
use crate::representation::{payload::Payload, IndexType};

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


impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Checks whether the twin of the twin is always the edge itself
    fn check_edge_twins(&self) -> Result<(), String> {
        if let Some(unmatched_twin) = self
            .edges()
            .find(|e| e.twin(self).twin(self).id() != e.id())
        {
            Err(format!(
                "Edge {} has a twin {} with twin {}",
                unmatched_twin.id(),
                unmatched_twin.twin(self).id(),
                unmatched_twin.twin(self).twin(self).id()
            ))
        } else {
            Ok(())
        }
    }

    // TODO: more checks!

    /// Returns all edges as pseudo-winged edges
    fn pair_edges(&self) -> Vec<PseudoWingedEdge<E, V, F>> {
        let mut edges: HashMap<E, PseudoWingedEdge<E, V, F>> = HashMap::new();
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

        let mut vec: Vec<PseudoWingedEdge<E, V, F>> = edges.values().cloned().collect();
        vec.sort_by(|a, b| a.id.cmp(&b.id));
        vec
    }

    /// Checks whether there is anything fishy about the mesh
    pub fn check(&self) -> Result<(), String> {
        self.check_edge_twins()?;
        Ok(())
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> std::fmt::Display for Mesh<E, V, F, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(msg) = self.check() {
            return write!(f, "Mesh is invalid: {}", msg);
        }

        write!(
            f,
            "Mesh:\nvertices:\n{}\n edge --><-- twin   |  face: edge/twin \n{}\n faces: \n{}\n ",
            self.vertices()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pair_edges()
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.faces()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
