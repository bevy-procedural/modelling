use super::VertexBasics;
use crate::mesh::{EdgeBasics, HalfEdge, MeshType};
use itertools::Itertools;

/// Basic vertex functionality for a mesh
pub trait HalfEdgeVertex<T: MeshType>: VertexBasics<T>
where
    T::Edge: HalfEdge<T>,
{
    /// Changes the representative of the outgoing edges
    fn set_edge(&mut self, edge: T::E);

    /// Returns an outgoing boundary edge incident to the vertex
    fn outgoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        // TODO: Assumes a manifold vertex. Otherwise, there can be multiple boundary edges!
        debug_assert!(
            self.edges_out(mesh)
                .filter(|e| e.is_boundary_self())
                .exactly_one()
                .is_ok(),
            "Vertex {} is not manifold",
            self.id()
        );

        self.edges_out(mesh).find_map(|e| {
            if e.is_boundary_self() {
                Some(e.id())
            } else {
                None
            }
        })
    }

    /// Returns an ingoing boundary edge incident to the vertex
    fn ingoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        debug_assert!(
            self.edges_in(mesh)
                .filter(|e| e.is_boundary_self())
                .exactly_one()
                .is_ok(),
            "Vertex {} is not manifold",
            self.id()
        );

        self.edges_in(mesh).find_map(|e| {
            if e.is_boundary_self() {
                Some(e.id())
            } else {
                None
            }
        })
    }
}
