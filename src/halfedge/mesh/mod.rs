mod iterator;
mod topology;

use std::collections::HashMap;

use super::{HalfEdge, HalfEdgeMeshType};
use crate::{
    math::IndexType,
    mesh::{payload::VertexPayload, Edge, Mesh, Vertex},
    tesselate::Triangulation,
    util::{Deletable, DeletableVector},
};

/// A halfedge-inspired mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are currently not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
///
/// Currently only euclidean geometry is supported.
#[derive(Clone)]
pub struct HalfEdgeMesh<T: HalfEdgeMeshType> {
    // TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
    // TODO: non-euclidean geometry
    vertices: DeletableVector<T::Vertex, T::V>,
    halfedges: DeletableVector<T::Edge, T::E>,
    faces: DeletableVector<T::Face, T::F>,
    payload: T::MP,
}

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Creates a new empty halfedge mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
            payload: T::MP::default(),
        }
    }

    /// Flips the edge, i.e., swaps the origin and target vertices.
    pub fn flip_edge(&mut self, e: T::E) -> &mut Self {
        HalfEdge::flip(e, self);
        self
    }

    /// Flip all edges (and faces) turning the mesh inside out.
    pub fn flip(&mut self) -> &mut Self {
        // TODO: this is an unnecessary clone
        let ids: Vec<T::E> = self.edges().map(|e| e.id()).collect();
        ids.iter().for_each(|&e| {
            self.flip_edge(e);
        });
        self
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeMesh<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HalfEdgeMeshType> Mesh<T> for HalfEdgeMesh<T> {
    fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    fn vertex(&self, index: T::V) -> &T::Vertex {
        self.vertices.get(index)
    }

    fn edge(&self, index: T::E) -> &T::Edge {
        self.halfedges.get(index)
    }

    fn face(&self, index: T::F) -> &T::Face {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    fn vertex_mut(&mut self, index: T::V) -> &mut T::Vertex {
        self.vertices.get_mut(index)
    }

    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge {
        self.halfedges.get_mut(index)
    }

    fn face_mut(&mut self, index: T::F) -> &mut T::Face {
        self.faces.get_mut(index)
    }

    fn is_open(&self) -> bool {
        self.halfedges.iter().any(|e| e.is_boundary_self())
    }

    fn max_vertex_index(&self) -> usize {
        self.vertices.capacity()
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn num_edges(&self) -> usize {
        self.halfedges.len()
    }

    fn num_faces(&self) -> usize {
        self.faces.len()
    }

    fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.halfedges.clear();
        self.faces.clear();
        self
    }

    fn payload(&self) -> &T::MP {
        &self.payload
    }

    fn payload_mut(&mut self) -> &mut T::MP {
        &mut self.payload
    }

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter()
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter_mut()
    }

    /// Returns an iterator over all non-deleted faces
    fn faces<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T::Face: 'a,
    {
        self.faces.iter()
    }

    /// Returns an iterator over all non-deleted halfedges
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter()
    }

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn get_compact_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP> {
        let mut vertices = Vec::with_capacity(self.num_vertices());

        if self.vertices.len() == self.vertices.capacity() {
            // Vertex buffer is already compact.
            // Since the index map creation is time consuming, we avoid this if possible.
            for _ in 0..self.vertices.capacity() {
                vertices.push(T::VP::allocate());
            }
            for v in self.vertices() {
                vertices[v.id().index()] = v.payload().clone();
            }
        } else {
            // Vertex buffer is sparse.
            // We need to create a map from the old indices to the new compact indices.
            let mut id_map = HashMap::new();
            for v in self.vertices() {
                id_map.insert(v.id(), T::V::new(vertices.len()));
                vertices.push(v.payload().clone());
            }
            Triangulation::new(indices).map_indices(&id_map);
        }

        vertices
    }
}

impl<T: HalfEdgeMeshType> std::fmt::Display for HalfEdgeMesh<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "Mesh:\nvertices:\n{}\n edge --><-- twin   |  face: edge/twin \n{}\n faces: \n{}\n{} ",
            self.vertices()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pair_edges()
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.faces()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            if let Err(msg) = self.check() {
                format!(
                    "⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ERROR ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️\n{}",
                    msg
                )
            } else {
                "".to_string()
            }
        )*/
        todo!("Display not implemented yet");
    }
}
