use super::{Mesh, MeshType};
use crate::representation::{Deletable, Face, HalfEdge, IncidentToFaceBackIterator, IncidentToFaceIterator, Vertex};

impl<T: MeshType> Mesh<T> {
    /// Returns an iterator over all non-deleted vertices
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<T::E, T::V, T::VP>> {
        self.vertices.iter()
    }

    /// Returns an mutable iterator over all non-deleted vertices
    pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut Vertex<T::E, T::V, T::VP>> {
        self.vertices.iter_mut()
    }

    /// Returns an iterator over all non-deleted halfedges
    pub fn halfedges(&self) -> impl Iterator<Item = &HalfEdge<T::E, T::V, T::F, T::EP>> {
        self.halfedges.iter()
    }

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    pub fn edges(
        &self,
    ) -> impl Iterator<
        Item = (
            &HalfEdge<T::E, T::V, T::F, T::EP>,
            &HalfEdge<T::E, T::V, T::F, T::EP>,
        ),
    > {
        self.halfedges.iter().filter_map(move |e| {
            if e.is_deleted() {
                None
            } else if e.twin_id() < e.id() {
                None
            } else {
                Some((e, self.halfedges.get(e.twin_id())))
            }
        })
    }

    /// Returns an iterator over all non-deleted faces
    pub fn faces(&self) -> impl Iterator<Item = &Face<T::E, T::F, T::FP>> {
        self.faces.iter()
    }

    pub fn edges_from<'a>(&'a self, e: T::E) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::<'a, T>::new(*self.edge(e), self)
    }

    pub fn edges_back_from<'a>(&'a self, e: T::E) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::<'a, T>::new(*self.edge(e), self)
    }
}
