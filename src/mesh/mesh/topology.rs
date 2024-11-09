use crate::mesh::VertexBasics;

use super::{MeshBasics, MeshType};
use std::collections::{HashMap, HashSet, VecDeque};

/// Methods concerned with mesh topology.
pub trait MeshTopology<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Returns the shortest path (least number of edges) between two vertices
    /// or returns `None` if no path exists.
    /// Uses Breadth-First Search (BFS) to find the shortest path.
    fn shortest_path(&self, v0: T::V, v1: T::V) -> Option<Vec<T::V>> {
        // TODO: This could easily work without halfedge but any graph

        if v0 == v1 {
            return Some(vec![v0]);
        }

        let mut queue = VecDeque::with_capacity(self.num_vertices());
        let mut visited = HashSet::with_capacity(self.num_vertices());
        let mut predecessor = HashMap::with_capacity(self.num_vertices());

        queue.push_back(v1);
        visited.insert(v1);
        predecessor.insert(v1, None);

        while let Some(current) = queue.pop_front() {
            for neighbor in self.vertex(current).neighbor_ids(self) {
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                predecessor.insert(neighbor, Some(current));
                queue.push_back(neighbor);

                if neighbor == v0 {
                    let mut path = Vec::new();
                    let mut step = Some(v0);
                    while let Some(vertex) = step {
                        path.push(vertex);
                        step = predecessor[&vertex];
                    }
                    return Some(path);
                }
            }
        }

        None
    }
}
