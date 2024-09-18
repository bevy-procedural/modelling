use super::HalfEdgeMesh;
use crate::{
    halfedge::HalfEdgeMeshType,
    mesh::{Edge, Mesh},
};

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Returns the id of the half edge from `v` to `w` or `None` if they are not neighbors.
    /// Runs in O(n) time since it iterates over all edges of `v`.
    pub fn shared_edge(&self, v: T::V, w: T::V) -> Option<T::Edge> {
        self.vertex(v).edges_out(self).find_map(|e| {
            if e.target_id(self) == w {
                Some(e)
            } else {
                None
            }
        })
    }

    /// Returns the half edge id from v to w. Panics if the edge does not exist.
    pub fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E> {
        self.shared_edge(v, w).map(|e| e.id())
    }
}
