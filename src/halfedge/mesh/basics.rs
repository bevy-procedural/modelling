use super::{HalfEdgeImplMeshType, HalfEdgeMeshImpl};
use crate::{
    math::IndexType,
    mesh::{
        cursor::*, Edge2ValidEdgeCursorAdapter, EdgeBasics, FilterIdIterator, HalfEdge, HalfEdgeMesh, MeshBasics, MeshType, Triangulation, VertexBasics, VertexPayload
    },
    prelude::IncidentToVertexIterator,
    util::{CreateEmptyIterator, DeletableVectorIter},
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

    type VertexRefIter<'a>
        = DeletableVectorIter<'a, T::Vertex>
    where
        T: 'a;

    #[inline]
    fn vertex_refs<'a>(&'a self) -> Self::VertexRefIter<'a>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter()
    }

    type VertexIdIter<'a>
        = std::iter::Map<DeletableVectorIter<'a, T::Vertex>, fn(&'a T::Vertex) -> T::V>
    where
        T: 'a;

    #[inline]
    fn vertex_ids<'a>(&'a self) -> Self::VertexIdIter<'a>
    where
        T: 'a,
    {
        self.vertex_refs().map(|v| v.id())
    }

    #[inline]
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter_mut()
    }

    type VertexEdgesOutIterator<'a>
        = IncidentToVertexIterator<'a, T>
    where
        T: 'a;

    #[inline]
    fn vertex_edges_out<'a>(&'a self, v: T::V) -> Self::VertexEdgesOutIterator<'a>
    where
        T: 'a,
    {
        self.get_vertex(v).map_or_else(
            || CreateEmptyIterator::create_empty(),
            |v| IncidentToVertexIterator::<'a, T>::new(v.edge_id(self), self),
        )
    }

    type VertexEdgesInIterator<'a>
        = std::iter::Map<
        Self::VertexEdgesOutIterator<'a>,
        fn(ValidEdgeCursor<'a, T>) -> ValidEdgeCursor<'a, T>,
    >
    where
        T: 'a;

    #[inline]
    fn vertex_edges_in<'a>(&'a self, v: T::V) -> Self::VertexEdgesInIterator<'a>
    where
        T: 'a,
    {
        self.vertex_edges_out(v).map(|e| e.twin().unwrap())
    }

    type VertexNeighborsIterator<'a>
        = std::iter::Map<
        Self::VertexEdgesOutIterator<'a>,
        fn(ValidEdgeCursor<'a, T>) -> ValidVertexCursor<'a, T>,
    >
    where
        T: 'a;

    #[inline]
    fn vertex_neighbors<'a>(&'a self, v: T::V) -> Self::VertexNeighborsIterator<'a>
    where
        T: 'a,
    {
        self.vertex_edges_out(v).map(|e| e.target().unwrap())
    }

    type VertexFacesIterator<'a>
        = std::iter::FilterMap<
        Self::VertexEdgesOutIterator<'a>,
        fn(ValidEdgeCursor<'a, T>) -> Option<ValidFaceCursor<'a, T>>,
    >
    where
        T: 'a;

    #[inline]
    fn vertex_faces<'a>(&'a self, v: T::V) -> Self::VertexFacesIterator<'a>
    where
        T: 'a,
    {
        self.vertex_edges_out(v).filter_map(|e| e.face().load())
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

    type EdgeRefIter<'a>
        = std::iter::Filter<DeletableVectorIter<'a, T::Edge>, fn(&&T::Edge) -> bool>
    where
        T: 'a;

    #[inline]
    fn edge_refs<'a>(&'a self) -> Self::EdgeRefIter<'a>
    where
        T::Edge: 'a,
    {
        self.halfedges.iter().filter(|e| e.twin_id() > e.id())
    }

    type EdgeIter<'a>
        = Edge2ValidEdgeCursorAdapter<'a, T, Self::EdgeRefIter<'a>>
    where
        T: 'a;

    #[inline]
    fn edges<'a>(&'a self) -> Self::EdgeIter<'a>
    where
        T: 'a,
    {
        Edge2ValidEdgeCursorAdapter::new(self, self.edge_refs())
        //self.edge_refs().map(move |edge| ValidEdgeCursor::new(self, edge))
    }

    type EdgeIdIter<'a>
        = std::iter::Map<Self::EdgeRefIter<'a>, fn(&'a T::Edge) -> T::E>
    where
        T: 'a;

    #[inline]
    fn edge_ids<'a>(&'a self) -> Self::EdgeIdIter<'a>
    where
        T: 'a,
    {
        self.edge_refs().map(|e| e.id())
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

    type SharedEdgeIter<'a>
        = FilterIdIterator<'a, T, <T::Mesh as MeshBasics<T>>::VertexEdgesOutIterator<'a>>
    where
        T: 'a;

    #[inline]
    fn shared_edges<'a>(&'a self, v: T::V, w: T::V) -> Self::SharedEdgeIter<'a>
    where
        T: 'a,
    {
        let Some(v) = self.vertex(v).load() else {
            return CreateEmptyIterator::create_empty();
        };
        let (mesh, id) = v.destructure();
        FilterIdIterator::new(mesh.vertex_ref(id).edges_out(mesh), w)
    }

    #[inline]
    fn shared_edge_ids(&self, v: T::V, w: T::V) -> impl Iterator<Item = T::E> {
        self.shared_edges(v, w).map(|e| e.id())
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
