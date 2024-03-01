//! This module contains the builder functions for the mesh representation.

mod face;
mod vertex;
mod extrude;
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
        let e0 = E::new(self.edges.len());
        let e1 = E::new(self.edges.len() + 1);
        let v0 = V::new(self.vertices.len());
        let v1 = V::new(self.vertices.len() + 1);

        self.vertices.push(Vertex::new(v0, e0, v0, a));
        self.vertices.push(Vertex::new(v1, e1, v1, b));
        self.edges
            .push(HalfEdge::new(e0, e1, e1, e1, v0, IndexType::max()));
        self.edges
            .push(HalfEdge::new(e1, e0, e0, e0, v1, IndexType::max()));

        return (v0, v1);
    }
}
