//! This whole module is deprecated


use crate::{math::IndexType, representation::Face};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DualVertexId(usize);

impl DualVertexId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the index of the dual vertex.
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DualVertex {
    /// starting index of the indices of the triangle
    start: usize,

    /// it's own id in the data structure
    id: DualVertexId,

    /// indices of the neighboring dual vertices (faces)
    n1: Option<DualVertexId>,
    n2: Option<DualVertexId>,
    n3: Option<DualVertexId>,
}

impl DualVertex {
    /// Creates a new dual vertex (face).
    pub fn new(id: DualVertexId, start: usize) -> Self {
        Self {
            id,
            start,
            n1: None,
            n2: None,
            n3: None,
        }
    }

    /// Returns the id of the dual vertex (face).
    #[inline(always)]
    pub fn id(&self) -> DualVertexId {
        self.id
    }

    /// Returns the starting index of the indices of the triangle.
    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the indices of the neighbors
    #[inline(always)]
    pub fn neighbors(
        &self,
    ) -> (
        Option<DualVertexId>,
        Option<DualVertexId>,
        Option<DualVertexId>,
    ) {
        (self.n1, self.n2, self.n3)
    }

    /// Iterates the neighbor's ids
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = DualVertexId> {
        self.neighbors_array().into_iter()
    }

    /// Tries to remove the neighbor or panic if it is not a neighbor
    #[inline(always)]
    pub fn try_remove(&mut self, neighbor: Option<DualVertexId>) {
        if neighbor.is_none() {
            return;
        }
        if self.n1 == neighbor {
            self.n1 = None;
        } else if self.n2 == neighbor {
            self.n2 = None;
        } else if self.n3 == neighbor {
            self.n3 = None;
        } else {
            panic!(
                "Tried to remove a non-neighbor from a dual face {:?}",
                neighbor
            );
        }
    }

    /// Tries to replace the neighbor.
    pub fn try_replace(&mut self, old: Option<DualVertexId>, new: Option<DualVertexId>) {
        self.try_remove(old);
        if let Some(n) = new {
            self.add_neighbor(n);
        }
    }

    /// Returns the indices of the neighbors
    #[inline(always)]
    pub fn neighbors_array(&self) -> Vec<DualVertexId> {
        let mut result = Vec::new();
        if let Some(n) = self.n1 {
            result.push(n);
        }
        if let Some(n) = self.n2 {
            result.push(n);
        }
        if let Some(n) = self.n3 {
            result.push(n);
        }
        result
    }

    /// Whether the vertex contains that edge
    pub fn has_edge<V: IndexType>(&self, a: usize, b: usize, indices: &Vec<V>) -> bool {
        let start = self.start();
        indices[start..start + 3].contains(&indices[a])
            && indices[start..start + 3].contains(&indices[b])
    }

    /// Add a neighbor or panic if there are already three neighbors.
    #[inline(always)]
    pub fn add_neighbor(&mut self, neighbor: DualVertexId) {
        if self.n1.is_none() {
            self.n1 = Some(neighbor);
        } else if self.n2.is_none() {
            self.n2 = Some(neighbor);
        } else if self.n3.is_none() {
            self.n3 = Some(neighbor);
        } else {
            panic!("Tried to add a fourth neighbor to a dual face");
        }
    }
}

/// A pair of neighbored dual vertices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NeighboredDualVertices {
    /// non-shared vertex
    pub self_ns: usize,
    /// first shared vertex
    pub self_s1: usize,
    /// second shared vertex
    pub self_s2: usize,

    // other
    pub other_ns: usize,
    pub other_s1: usize,
    pub other_s2: usize,

    pub s: DualVertexId,
    pub o: DualVertexId,
}

impl NeighboredDualVertices {
    pub fn new<V: IndexType>(
        s: [usize; 3],
        this: &DualVertexId,
        o: [usize; 3],
        other: &DualVertexId,
        indices: &Vec<V>,
    ) -> Self {
        // they have to share an edge
        assert!(indices[s[2]] == indices[o[1]]);
        assert!(indices[s[1]] == indices[o[2]]);

        Self {
            self_ns: s[0],
            self_s1: s[1],
            self_s2: s[2],
            s: *this,
            other_ns: o[0],
            other_s1: o[1],
            other_s2: o[2],
            o: *other,
        }
    }

    pub fn flip_indices<V: IndexType>(&self, indices: &mut Vec<V>) {
        let v0 = indices[self.self_ns];
        let v1 = indices[self.self_s1];
        let v2 = indices[self.self_s2];
        let w = indices[self.other_ns];

        assert!(indices[self.self_ns] == v0);
        indices[self.self_s1] = w;
        assert!(indices[self.self_s2] == v2);

        assert!(indices[self.other_ns] == w);
        indices[self.other_s1] = v0;
        assert!(indices[self.other_s2] == v1);
    }
}

pub struct DualTriangulation {
    /// dual vertices (faces) of the triangulation
    vertices: Vec<DualVertex>,
}

impl DualTriangulation {
    /// Creates a new dual triangulation.
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    /// Returns the dual vertices (faces) of the triangulation.
    #[inline(always)]
    pub fn vertices(&self) -> &Vec<DualVertex> {
        &self.vertices
    }

    /// Returns the indices of the dual vertices (faces) of the triangulation.
    #[inline(always)]
    pub fn indices(&self) -> Vec<DualVertexId> {
        (0..self.vertices.len()).map(DualVertexId).collect()
    }

    /// Returns the respective vertex.
    #[inline(always)]
    pub fn vertex(&self, id: &DualVertexId) -> &DualVertex {
        &self.vertices[id.index()]
    }

    /// Adds a dual vertex (face) to the graph.
    #[inline(always)]
    pub fn add_vertex(&mut self, start: usize) -> DualVertexId {
        let id = DualVertexId(self.vertices.len());
        self.vertices.push(DualVertex::new(id, start));
        id
    }

    /// Link to dual vertices together.
    pub fn link_vertices(&mut self, a: DualVertexId, b: DualVertexId) {
        self.vertices[a.index()].add_neighbor(b);
        self.vertices[b.index()].add_neighbor(a);
    }

    /// Returns the non-shared vertex of the neighbors
    pub fn neighbor_thirds<V: IndexType>(
        &self,
        center: DualVertexId,
        indices: &Vec<V>,
    ) -> Vec<usize> {
        let mut result = Vec::new();
        let me = self.vertices[center.index()];
        for n_i in me.neighbors_array() {
            let neighbor = self.vertices[n_i.index()];
            let mut found = false;
            for i in 0..3 {
                let this = indices[neighbor.start() + i];
                if (0..3).any(|j| indices[me.start() + j] == this) {
                    continue;
                }
                result.push(neighbor.start() + i);
                assert!(!found, "Found two non-shared vertices");
                found = true;
            }
            assert!(found, "Did not find a non-shared vertex");
        }
        result
    }

    /// Flip the faces (dual vertices) in the dual graph
    pub fn flip<V: IndexType>(&mut self, n: &NeighboredDualVertices, indices: &Vec<V>) {
        let s_to_o = self.vertex(&n.s).iter().find(|&x| {
            // ignore the shared one - they neighbors
            if x == n.o {
                return false;
            }
            // find the edge that is not shared via ns-s1
            self.vertex(&x).has_edge(n.self_ns, n.self_s1, indices)
        });
        let o_to_s = self.vertex(&n.o).iter().find(|&x| {
            // ignore the shared one - they neighbors
            if x == n.s {
                return false;
            }
            // find the edge that is not shared via ns-s1
            self.vertex(&x).has_edge(n.other_ns, n.other_s1, indices)
        });

        assert!(self.vertex(&n.s).neighbors_array().len() != 3 || s_to_o.is_some());
        assert!(self.vertex(&n.o).neighbors_array().len() != 3 || o_to_s.is_some());

        self.vertices[n.s.index()].try_replace(s_to_o, o_to_s);
        self.vertices[n.o.index()].try_replace(o_to_s, s_to_o);
        if let Some(v) = s_to_o {
            self.vertices[v.index()].try_replace(Some(n.s), Some(n.o));
        }
        if let Some(v) = o_to_s {
            self.vertices[v.index()].try_replace(Some(n.o), Some(n.s));
        }
    }

    /// Returns the neighbor and self rotated such that the non-shared vertex is the first one
    pub fn neighbor_rotated<V: IndexType>(
        &self,
        center: &DualVertexId,
        indices: &Vec<V>,
        n_i: &DualVertexId,
    ) -> NeighboredDualVertices {
        let me = self.vertices[center.index()];
        let ms = me.start();

        assert!(self.vertex(center).neighbors_array().contains(&n_i));

        let neighbor = self.vertex(n_i);
        let ns = neighbor.start();

        let other_third = (0..3)
            .find(|i| {
                let this = indices[ns + i];
                (0..3).all(|j| indices[ms + j] != this)
            })
            .unwrap();
        let mut other_rotated = [ns, ns + 1, ns + 2];
        other_rotated.rotate_left(other_third);

        let self_third = (0..3)
            .find(|i| {
                let this = indices[ms + i];
                indices[other_rotated[1]] != this && indices[other_rotated[2]] != this
            })
            .unwrap();
        let mut self_rotated = [ms, ms + 1, ms + 2];
        self_rotated.rotate_left(self_third);

        NeighboredDualVertices::new(self_rotated, center, other_rotated, n_i, indices)
    }
}

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Returns the dual of the triangulation of the face starting in first.
    pub fn dual<V: IndexType>(&self, indices: &mut Vec<V>, first: usize) -> DualTriangulation {
        let mut dual = DualTriangulation::new();
        let mut edge_cache = std::collections::HashMap::new();
        for ((i, a), (_, b), (_, c)) in indices[first..].iter().enumerate().tuples() {
            let v = dual.add_vertex(i.index());
            for edge in [
                (a.min(b), a.max(b)),
                (b.min(c), b.max(c)),
                (a.min(c), a.max(c)),
            ] {
                if let Some(&d) = edge_cache.get(&edge) {
                    dual.link_vertices(d, v);
                    edge_cache.remove(&edge); // TODO: is it faster when removing old edges?
                } else {
                    edge_cache.insert(edge, v);
                }
            }
        }
        dual
    }
}

impl std::fmt::Display for DualTriangulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.vertices.iter().enumerate() {
            writeln!(f, "f {} {} {:?}", i, v.start(), v.neighbors_array())?;
        }
        Ok(())
    }
}
