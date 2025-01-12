use super::{HalfEdgeImplMeshType, HalfEdgeMeshImpl};
use crate::{
    math::IndexType,
    mesh::{
        CursorData, EdgeBasics, EdgeCursorBasics, EdgeCursorHalfedgeBasics, FaceBasics, HalfEdge,
        HalfEdgeMesh, MeshBasics, Triangulation, VertexBasics, VertexPayload,
    },
    prelude::IncidentToVertexIterator,
};
use std::collections::HashMap;

impl<T: HalfEdgeImplMeshType> MeshBasics<T> for HalfEdgeMeshImpl<T> {
    //======================= Vertex Operations =======================//

    #[inline]
    fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    #[inline]
    fn get_vertex(&self, index: T::V) -> Option<&T::Vertex> {
        self.vertices.get(index)
    }

    #[inline]
    fn get_vertex_mut(&mut self, index: T::V) -> Option<&mut T::Vertex> {
        self.vertices.get_mut(index)
    }

    #[inline]
    fn max_vertex_index(&self) -> usize {
        self.vertices.capacity()
    }

    #[inline]
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    fn vertex_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter()
    }

    #[inline]
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter_mut()
    }

    #[inline]
    fn vertex_edges_out(&self, v: T::V) -> impl Iterator<Item = T::E> {
        self.get_vertex(v).map_or_else(
            || IncidentToVertexIterator::<'_, T>::empty(self),
            |v| IncidentToVertexIterator::<'_, T>::new(v.edge_id(self), self),
        )
    }

    #[inline]
    fn vertex_edges_in(&self, v: T::V) -> impl Iterator<Item = T::E> {
        self.vertex_edges_out(v).map(|e| self.edge(e).twin_id())
    }

    #[inline]
    fn vertex_neighbors(&self, v: T::V) -> impl Iterator<Item = T::V> {
        self.vertex_edges_out(v)
            .map(move |e| self.edge(e).target_id())
    }

    #[inline]
    fn vertex_faces(&self, v: T::V) -> impl Iterator<Item = T::F> {
        self.vertex_edges_out(v).filter_map(move |e| {
            let f = self.edge(e).face_id();
            if f == IndexType::max() {
                None
            } else {
                Some(f)
            }
        })
    }

    //======================= Edge Operations =======================//

    #[inline]
    fn has_edge(&self, index: T::E) -> bool {
        self.halfedges.has(index)
    }

    #[inline]
    fn get_edge(&self, index: T::E) -> Option<&T::Edge> {
        self.halfedges.get(index)
    }

    #[inline]
    fn get_edge_mut<'a>(&'a mut self, index: T::E) -> Option<&'a mut T::Edge> {
        self.halfedges.get_mut(index)
    }

    #[inline]
    fn num_edges(&self) -> usize {
        let n = self.num_halfedges();
        assert!(n % 2 == 0, "Number of halfedges is not even");
        n / 2
    }

    #[inline]
    fn edge_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter().filter(|e| e.twin_id() > e.id())
    }

    #[inline]
    fn edges_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Edge>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter_mut()
    }

    #[inline]
    fn edge_payload<'a>(&'a self, e: T::E) -> &'a T::EP {
        let edge = self.edge_ref(e);
        if let Some(p) = &edge.payload_self() {
            p
        } else if let Some(p) = &(self.edge_ref(edge.twin_id()).payload_self()) {
            p
        } else {
            panic!("No payload found for edge {}", e);
        }
    }

    #[inline]
    fn edge_payload_mut<'a>(&'a mut self, e: T::E) -> &'a mut T::EP {
        if self.edge_ref(e).payload_self().is_some() {
            let pr: Option<&'a mut T::EP> = self.edge_ref_mut(e).payload_self_mut();
            if let Some(v) = pr {
                return v;
            }
        } else {
            let twin_id = self.edge_ref(e).twin_id();
            let pr: Option<&'a mut T::EP> = self.edge_ref_mut(twin_id).payload_self_mut();
            if let Some(v) = pr {
                return v;
            }
        }
        panic!("No payload found for edge {}", e);
    }

    /// Implements [MeshBasics::shared_edge] for halfedge meshes.
    ///
    /// Runs in O(n) time since it iterates over all edges of `v`.
    #[inline]
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<&T::Edge> {
        self.shared_edge_id(v, w).map(|e| self.edge_ref(e))
    }

    #[inline]
    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E> {
        self.vertex(v).edges_out().find_map(|e| {
            if e.target_id() == w {
                Some(e.id())
            } else {
                None
            }
        })
    }

    #[inline]
    fn shared_edges<'a>(&'a self, v: T::V, w: T::V) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a,
    {
        self.shared_edge_ids(v, w).map(move |e| self.edge_ref(e))
    }

    #[inline]
    fn shared_edge_ids(&self, v: T::V, w: T::V) -> impl Iterator<Item = T::E> {
        self.vertex(v)
            .edges_out()
            .filter(move |e| e.target_id() == w)
            .map(|e| e.id())
    }

    //======================= Face Operations =======================//

    #[inline]
    fn has_face(&self, index: T::F) -> bool {
        self.faces.has(index)
    }

    #[inline]
    fn get_face(&self, index: T::F) -> Option<&T::Face> {
        self.faces.get(index)
    }

    #[inline]
    fn get_face_mut(&mut self, index: T::F) -> Option<&mut T::Face> {
        self.faces.get_mut(index)
    }

    #[inline]
    fn num_faces(&self) -> usize {
        self.faces.len()
    }

    #[inline]
    fn face_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T::Face: 'a,
    {
        self.faces.iter()
    }

    #[inline]
    fn faces_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut <T as crate::prelude::MeshType>::Face>
    where
        T: 'a,
    {
        self.faces.iter_mut()
    }

    #[inline]
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F> {
        // TODO: Currently cannot distinguish between holes and "the outside"
        let w1 = self.vertex(v1);
        self.vertex(v0).faces().find_map(|f0| {
            w1.clone().faces().find_map(|f1| {
                if f0.id() == f1.id() {
                    Some(f0.id())
                } else {
                    None
                }
            })
        })
    }

    //======================= Mesh Operations =======================//

    #[inline]
    fn is_open(&self) -> bool {
        self.halfedges.iter().any(|e| e.is_boundary_self())
    }

    #[inline]
    fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.halfedges.clear();
        self.faces.clear();
        self
    }

    #[inline]
    fn payload(&self) -> &T::MP {
        &self.payload
    }

    #[inline]
    fn payload_mut(&mut self) -> &mut T::MP {
        &mut self.payload
    }

    #[inline]
    fn set_payload(&mut self, payload: T::MP) -> &mut Self {
        self.payload = payload;
        self
    }

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

    fn has_holes(&self) -> bool {
        for e in self.halfedge_refs() {
            if e.is_boundary_self() {
                return true;
            }
        }
        false
    }
}
