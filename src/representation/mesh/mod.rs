pub mod builder;
mod check;
mod mesh_type;
pub mod primitives;
mod tesselate;

#[cfg(feature = "bevy")]
pub mod bevy;

use super::{Deletable, DeletableVector, Face, HalfEdge, Vertex};
pub use mesh_type::MeshType;
use std::collections::HashSet;

/// A mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
/// The geometry doesn't have to be Euclidean (TODO: But what do we require?).
///
/// TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
#[derive(Debug, Clone)]
pub struct Mesh<T: MeshType> {
    vertices: DeletableVector<Vertex<T::E, T::V, T::VP>, T::V>,
    edges: DeletableVector<HalfEdge<T::E, T::V, T::F, T::EP>, T::E>,
    faces: DeletableVector<Face<T::E, T::F, T::FP>, T::F>,
}

impl<T: MeshType> Mesh<T> {
    /// Creates a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            edges: DeletableVector::new(),
            faces: DeletableVector::new(),
        }
    }

    /// Returns a reference to the requested vertex
    pub fn vertex(&self, index: T::V) -> &Vertex<T::E, T::V, T::VP> {
        &self.vertices.get(index)
    }

    /// Returns a reference to the requested edge
    pub fn edge(&self, index: T::E) -> &HalfEdge<T::E, T::V, T::F, T::EP> {
        &self.edges.get(index)
    }

    /// Returns the half edge from v to w
    pub fn edge_between(&self, v: T::V, w: T::V) -> Option<HalfEdge<T::E, T::V, T::F, T::EP>> {
        let v = self.vertex(v).edges(self).find(|e| e.target_id(self) == w);
        if let Some(vv) = v {
            if vv.is_deleted() {
                None
            } else {
                Some(vv)
            }
        } else {
            None
        }
    }

    /// Returns the half edge id from v to w. Panics if the edge does not exist.
    pub fn edge_id_between(&self, v: T::V, w: T::V) -> T::E {
        self.edge_between(v, w).unwrap().id()
    }

    /// Returns a reference to the requested face
    pub fn face(&self, index: T::F) -> &Face<T::E, T::F, T::FP> {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    /// Returns a mutable reference to the requested vertex
    pub fn vertex_mut(&mut self, index: T::V) -> &mut Vertex<T::E, T::V, T::VP> {
        self.vertices.get_mut(index)
    }

    /// Returns a mutable reference to the requested edge
    pub fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut HalfEdge<T::E, T::V, T::F, T::EP> {
        self.edges.get_mut(index)
    }

    /// Returns a mutable reference to the requested face
    pub fn face_mut(&mut self, index: T::F) -> &mut Face<T::E, T::F, T::FP> {
        self.faces.get_mut(index)
    }

    /// Whether the mesh is open, i.e., has boundary edges
    pub fn is_open(&self) -> bool {
        self.edges.iter().any(|e| e.is_boundary_self())
    }

    /// Whether the mesh has non-manifold vertices
    pub fn has_nonmanifold_vertices(&self) -> bool {
        self.vertices.iter().any(|v| !v.is_manifold())
    }

    /// Whether the mesh is manifold, i.e., has no boundary edges and no non-manifold vertices
    pub fn is_manifold(&self) -> bool {
        !self.is_open() && !self.has_nonmanifold_vertices()
    }

    /// Returns the number of vertices in the mesh
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the maximum vertex index in the mesh
    pub fn max_vertex_index(&self) -> usize {
        self.vertices.max_ind()
    }

    /// Returns the number of edges in the mesh
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of faces in the mesh
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns an iterator over all non-deleted vertices
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<T::E, T::V, T::VP>> {
        self.vertices.iter()
    }

    /// Returns an mutable iterator over all non-deleted vertices
    pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut Vertex<T::E, T::V, T::VP>> {
        self.vertices.iter_mut()
    }

    /// Returns an iterator over all non-deleted edges
    pub fn edges(&self) -> impl Iterator<Item = &HalfEdge<T::E, T::V, T::F, T::EP>> {
        self.edges.iter()
    }

    /// Returns an iterator over all non-deleted faces
    pub fn faces(&self) -> impl Iterator<Item = &Face<T::E, T::F, T::FP>> {
        self.faces.iter()
    }

    /// Transforms all vertices in the mesh
    pub fn transform(&mut self, t: &T::Trans) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.transform(t);
        }
        self
    }

    /// Translates all vertices in the mesh
    pub fn translate(&mut self, t: &T::Vec) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.translate(t);
        }
        self
    }

    /// Rotates all vertices in the mesh
    pub fn rotate(&mut self, rotation: &T::Quat) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.rotate(rotation);
        }
        self
    }

    /// Clears the mesh (deletes all vertices, edges, and faces)
    pub fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.edges.clear();
        self.faces.clear();
        self
    }

    /// Flip all edges (and faces) turning the mesh inside out.
    pub fn flip(&mut self) -> &mut Self {
        // TODO: Not very efficient, but easy to implement
        let edges: Vec<_> = self.edges().map(|e| e.id()).collect();
        let mut flipped = HashSet::new();
        for i in 0..edges.len() {
            if flipped.contains(&edges[i]) {
                continue;
            }
            HalfEdge::flip(edges[i], self);
            flipped.insert(self.edge(edges[i]).twin_id());
        }
        self
    }
}
