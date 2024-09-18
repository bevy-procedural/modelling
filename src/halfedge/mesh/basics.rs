use std::collections::HashMap;

use super::{HalfEdgeMesh, HalfEdgeMeshType};
use crate::{
    math::IndexType,
    mesh::{payload::VertexPayload, MeshBasics, Vertex},
    tesselate::Triangulation,
    util::Deletable,
};

impl<T: HalfEdgeMeshType> MeshBasics<T> for HalfEdgeMesh<T> {
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
