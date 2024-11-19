use super::{HalfEdgeImplMeshType, HalfEdgeVertexImpl, IncidentToVertexIterator};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics, VertexBasics},
};

impl<T: HalfEdgeImplMeshType> VertexBasics<T> for HalfEdgeVertexImpl<T> {
    /// Returns the index of the vertex
    #[inline(always)]
    fn id(&self) -> T::V {
        self.id
    }

    /// Returns the payload of the vertex
    #[inline(always)]
    fn payload(&self) -> &T::VP {
        &self.payload
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline(always)]
    fn payload_mut(&mut self) -> &mut T::VP {
        &mut self.payload
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline(always)]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.edges_out(mesh).any(|e| e.is_boundary(mesh))
    }

    /*
    /// Returns whether the vertex is manifold
    #[inline(always)]
    fn is_manifold(&self) -> bool {
        self.next == IndexType::max()
    }*/

    /// Returns whether the vertex has only one edge incident to it
    #[inline(always)]
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool {
        // self.edges(mesh).count() == 1
        if let Some(e) = self.edge(mesh) {
            e.prev_id() == e.twin_id()
        } else {
            false
        }
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline(always)]
    fn edge_id(&self, _mesh: &T::Mesh) -> T::E {
        self.edge
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline(always)]
    fn edge(&self, mesh: &T::Mesh) -> Option<T::Edge> {
        // PERF: avoid clone
        if self.edge == IndexType::max() {
            None
        } else {
            Some(mesh.edge(self.edge).clone())
        }
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Vertex> + 'a {
        // TODO: slightly inefficient because of the clone and target being indirect
        self.edges_out(mesh).map(|e| e.target(mesh).clone())
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Face> + 'a
    where
        T: 'a,
    {
        self.edges_out(mesh).filter_map(|e| e.face(mesh).cloned())
    }

    #[inline(always)]
    fn edges_out<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Edge> + 'a {
        if let Some(e) = self.edge(mesh) {
            IncidentToVertexIterator::<T>::new(e, mesh)
        } else {
            return IncidentToVertexIterator::<T>::empty(mesh);
        }
    }

    #[inline(always)]
    fn edges_in<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Edge> + 'a {
        (if let Some(e) = self.edge(mesh) {
            IncidentToVertexIterator::<T>::new(e, mesh)
        } else {
            IncidentToVertexIterator::<T>::empty(mesh)
        })
        .map(|e| e.twin(mesh))
    }
}
