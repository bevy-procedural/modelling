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

    /// Returns an outgoing boundary edge incident to the vertex if it exists and is unique
    fn outgoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        if let Ok(e) = self
            .edges_out(mesh)
            .filter(|e| e.is_boundary_self())
            .exactly_one()
        {
            debug_assert_eq!(e.origin_id(), self.id());
            Some(e.id())
        } else {
            None
        }
    }

    /// Returns an ingoing boundary edge incident to the vertex if it exists and is unique
    fn ingoing_boundary_edge(&self, mesh: &T::Mesh) -> Option<T::E> {
        if let Ok(e) = self
            .edges_in(mesh)
            .filter(|e| e.is_boundary_self())
            .exactly_one()
        {
            debug_assert_eq!(e.target_id(mesh), self.id());
            Some(e.id())
        } else {
            None
        }
    }
}
