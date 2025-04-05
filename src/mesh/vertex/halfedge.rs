use super::VertexBasics;
use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshType},
};
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
            debug_assert_eq!(e.target_id(), self.id());
            Some(e.id())
        } else {
            None
        }
    }

    /// Finds the unique shortest path from this vertex to another vertex using BFS.
    /// Returns `None` if there is no path or if there are multiple shortest paths.
    /// Returns the outgoing edge id from this vertex, the incoming edge id to the other vertex, and the length of the path.
    fn shortest_path(&self, mesh: &T::Mesh, goal: T::V) -> Option<(T::E, T::E, usize)> {
        // TODO: Duplicate of [`MeshTopology::shortest_path`]
        let v0 = self.id();

        let mut m = None;
        let starts: Vec<T::E> = self.edges_out(mesh).map(|e| e.id()).collect_vec();
        let mut paths = starts.clone();
        let mut len = 0;
        while m.is_none() {
            let mut productive = false;
            len += 1;

            for i in 0..paths.len() {
                let Some(e) = mesh.edge(paths[i]).load() else {
                    continue;
                };
                productive = true;

                if e.target_id() == v0 {
                    // walking through the start again - we should void this path
                    paths[i] = IndexType::max();
                } else if e.target_id() == goal {
                    // found a path
                    if m.is_some() {
                        // there is more than one path of shortest length
                        return None;
                    }
                    m = Some((paths[i], starts[i], len));
                } else {
                    // continue searching
                    paths[i] = e.next_id();
                }
            }
            if !productive {
                // None of the paths reached `a` before reaching `b` again
                return None;
            }
        }
        m
    }
}
