//! This module contains the builder functions for the mesh representation.

mod extrude;
mod face;
mod vertex;
use crate::representation::{payload::Payload, HalfEdge, IndexType, Mesh, Vertex};
pub use face::CloseFace;
pub use vertex::AddVertex;

// The simplest non-empty mesh: a single edge with two vertices
impl<E, V, F, P> From<(P, P)> for Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    fn from((a, b): (P, P)) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_isolated_edge(a, b);
        return mesh;
    }
}

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Inserts vertices a and b and adds an isolated edge between a and b.
    pub fn add_isolated_edge(&mut self, a: P, b: P) -> (V, V) {
        let v0 = self.vertices.allocate();
        let v1 = self.vertices.allocate();
        let (e0, e1) = self.insert_full_edge(
            (IndexType::max(), IndexType::max(), v0, IndexType::max()),
            (IndexType::max(), IndexType::max(), v1, IndexType::max()),
        );
        self.vertices.set(v0, Vertex::new(e0, a));
        self.vertices.set(v1, Vertex::new(e1, b));

        return (v0, v1);
    }

    /// Will allocate two edges and return them as a tuple.
    /// You can set next and prev to IndexType::max() to insert the id of the twin edge there.
    pub fn insert_full_edge(
        &mut self,
        (next1, prev1, origin1, face1): (E, E, V, F),
        (next2, prev2, origin2, face2): (E, E, V, F),
    ) -> (E, E) {
        let e1 = self.edges.allocate();
        let e2 = self.edges.allocate();
        self.edges.set(
            e1,
            HalfEdge::new(
                if next1 == IndexType::max() { e2 } else { next1 },
                e2,
                if prev1 == IndexType::max() { e2 } else { prev1 },
                origin1,
                face1,
            ),
        );
        self.edges.set(
            e2,
            HalfEdge::new(
                if next2 == IndexType::max() { e1 } else { next2 },
                e1,
                if prev2 == IndexType::max() { e1 } else { prev2 },
                origin2,
                face2,
            ),
        );
        return (e1, e2);
    }

    /// Removes the provided face.
    pub fn remove_face(&mut self, f: F) {
        let face = self.face(f);

        let edge_ids: Vec<_> = face.edges(self).map(|e| e.id()).collect();
        for e in edge_ids {
            self.edge_mut(e).delete_face();
        }
        self.faces.delete_internal(f);
    }
}
