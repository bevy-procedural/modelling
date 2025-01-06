use super::{HalfEdgeImplMeshType, HalfEdgeVertexImpl, IncidentToVertexIterator};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics, MeshType, VertexBasics},
};

impl<T: MeshType> VertexBasics<T> for HalfEdgeVertexImpl<T>
where
    T: HalfEdgeImplMeshType,
{
    /// Returns the index of the vertex
    #[inline]
    fn id(&self) -> T::V {
        self.id
    }

    fn is_isolated(&self, _mesh: &T::Mesh) -> bool {
        self.edge == IndexType::max()
    }

    /// Returns the payload of the vertex
    #[inline]
    fn payload(&self) -> &T::VP {
        &self.payload
    }

    /// Returns a mutable reference to the payload of the vertex
    #[inline]
    fn payload_mut(&mut self) -> &mut T::VP {
        &mut self.payload
    }

    /// Returns whether the vertex is a boundary vertex
    #[inline]
    fn is_boundary(&self, mesh: &T::Mesh) -> bool {
        self.edges_out(mesh).any(|e| e.is_boundary(mesh))
    }

    /*
    /// Returns whether the vertex is manifold
    #[inline]
    fn is_manifold(&self) -> bool {
        self.next == IndexType::max()
    }*/

    /// Returns whether the vertex has only one edge incident to it
    #[inline]
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool {
        // self.edges(mesh).count() == 1
        if let Some(e) = self.edge(mesh) {
            e.prev_id() == e.twin_id()
        } else {
            false
        }
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline]
    fn edge_id(&self, _mesh: &T::Mesh) -> T::E {
        self.edge
    }

    /// Returns an outgoing half-edge incident to the vertex
    #[inline]
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Edge> {
        // PERF: avoid clone
        if self.edge == IndexType::max() {
            None
        } else {
            Some(mesh.edge(self.edge))
        }
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline]
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a,
    {
        self.edges_out(mesh).map(|e| e.target(mesh))
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline]
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a,
    {
        self.edges_out(mesh).filter_map(|e| e.face(mesh))
    }

    #[inline]
    fn edges_out<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a,
    {
        if let Some(e) = self.edge(mesh) {
            IncidentToVertexIterator::<'a, T>::new(e, mesh)
        } else {
            IncidentToVertexIterator::<'a, T>::empty(mesh)
        }
    }

    #[inline]
    fn edges_in<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a,
    {
        (if let Some(e) = self.edge(mesh) {
            IncidentToVertexIterator::<T>::new(e, mesh)
        } else {
            IncidentToVertexIterator::<T>::empty(mesh)
        })
        .map(|e| e.twin(mesh))
    }
}
