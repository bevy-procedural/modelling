use super::{Mesh, MeshType};
use crate::mesh::{Face, HalfEdge};
use std::collections::{HashMap, HashSet, VecDeque};

impl<T: MeshType> Mesh<T> {
   

    /// Returns the face shared by the two vertices or `None` if they are not neighbors.
    /// TODO: Currently cannot distinguish between holes and "the outside"
    pub fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F> {
        let w0 = self.vertex(v0);
        let w1 = self.vertex(v1);
        w0.faces(self).find_map(|f0| {
            w1.faces(self).find_map(
                |f1: Face<<T as MeshType>::E, <T as MeshType>::F, <T as MeshType>::FP>| {
                    if f0.id() == f1.id() {
                        Some(f0.id())
                    } else {
                        None
                    }
                },
            )
        })
    }

    /// Returns the shortest path (least number of edges) between two vertices
    /// or returns `None` if no path exists.
    /// Uses Breadth-First Search (BFS) to find the shortest path.
    pub fn shortest_path(&self, v0: T::V, v1: T::V) -> Option<Vec<T::V>> {
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
            for edge in self.vertex(current).edges_out(self) {
                let neighbor = edge.target_id(self);
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

    /*
    /// Whether the mesh has non-manifold vertices
    pub fn has_nonmanifold_vertices(&self) -> bool {
        self.vertices.iter().any(|v| !v.is_manifold())
    }

    /// Whether the mesh is manifold, i.e., has no boundary edges and no non-manifold vertices
    pub fn is_manifold(&self) -> bool {
        !self.is_open() && !self.has_nonmanifold_vertices()
    }*/
}
