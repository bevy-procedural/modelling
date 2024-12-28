use super::{HalfEdgeImplMeshType, HalfEdgeMeshImpl};
use crate::{
    math::IndexType,
    mesh::{
        EdgeBasics, FaceBasics, HalfEdge, MeshBasics, Triangulation, VertexBasics, VertexPayload,
    },
};
use std::collections::HashMap;

impl<T: HalfEdgeImplMeshType> MeshBasics<T> for HalfEdgeMeshImpl<T> {
    #[inline(always)]
    fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    #[inline(always)]
    fn has_edge(&self, index: T::E) -> bool {
        self.halfedges.has(index)
    }

    #[inline(always)]
    fn has_face(&self, index: T::F) -> bool {
        self.faces.has(index)
    }

    #[inline(always)]
    fn vertex(&self, index: T::V) -> &T::Vertex {
        self.vertices.get(index)
    }

    #[inline(always)]
    fn edge<'a>(&'a self, index: T::E) -> &'a T::Edge {
        self.halfedges.get(index)
    }

    #[inline(always)]
    fn face(&self, index: T::F) -> &T::Face {
        self.faces.get(index)
    }

    #[inline(always)]
    fn vertex_mut(&mut self, index: T::V) -> &mut T::Vertex {
        self.vertices.get_mut(index)
    }

    #[inline(always)]
    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge {
        self.halfedges.get_mut(index)
    }

    #[inline(always)]
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

    fn set_payload(&mut self, payload: T::MP) -> &mut Self {
        self.payload = payload;
        self
    }

    #[inline(always)]
    fn edge_payload<'a>(&'a self, edge: &'a T::Edge) -> &'a T::EP {
        if let Some(p) = &edge.payload_self() {
            p
        } else if let Some(p) = &(self.edge(edge.twin_id()).payload_self()) {
            p
        } else {
            panic!("No payload found for edge {}", edge.id());
        }
    }

    #[inline(always)]
    fn edge_payload_mut<'a>(&'a mut self, edge: &'a T::Edge) -> &'a mut T::EP {
        if edge.payload_self().is_some() {
            let pr: Option<&'a mut T::EP> = self.edge_mut(edge.id()).payload_self_mut();
            if let Some(v) = pr {
                return v;
            }
        } else {
            let twin_id = edge.twin_id();
            let pr: Option<&'a mut T::EP> = self.edge_mut(twin_id).payload_self_mut();
            if let Some(v) = pr {
                return v;
            }
        }
        panic!("No payload found for edge {}", edge.id());
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

    /// Returns an mutable iterator over all non-deleted faces
    fn faces_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut <T as crate::prelude::MeshType>::Face>
    where
        T: 'a,
    {
        self.faces.iter_mut()
    }

    /// Returns an iterator over all non-deleted halfedges
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter()
    }

    /// Returns an mutable iterator over all non-deleted halfedges
    fn edges_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Edge>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter_mut()
    }

    /// Returns the id of the half edge from `v` to `w` or `None` if they are not neighbors.
    /// Runs in O(n) time since it iterates over all edges of `v`.
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<&T::Edge> {
        self.vertex(v).edges_out(self).find_map(|e| {
            if e.target_id(self) == w {
                Some(e)
            } else {
                None
            }
        })
    }

    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E> {
        self.shared_edge(v, w).map(|e| e.id())
    }

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn dense_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP> {
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

    /// Returns the face shared by the two vertices or `None`.
    /// TODO: Currently cannot distinguish between holes and "the outside"
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F> {
        let w0 = self.vertex(v0);
        let w1 = self.vertex(v1);
        w0.faces(self).find_map(|f0| {
            w1.faces(self).find_map(|f1: &T::Face| {
                if f0.id() == f1.id() {
                    Some(f0.id())
                } else {
                    None
                }
            })
        })
    }
}
