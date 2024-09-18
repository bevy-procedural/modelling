use super::HalfEdgeMesh;
use crate::{halfedge::{HalfEdgeMeshType, IncidentToFaceBackIterator, IncidentToFaceIterator}, mesh::{Edge, Mesh}, util::Deletable};

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
   

    /// Returns an iterator over all non-deleted halfedges
    pub fn halfedges(&self) -> impl Iterator<Item = &T::Edge> {
        self.halfedges.iter()
    }

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    pub fn edges(&self) -> impl Iterator<Item = (&T::Edge, &T::Edge)> {
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
    pub fn faces(&self) -> impl Iterator<Item = &T::Face> {
        self.faces.iter()
    }

    /// Iterates forwards over the half-edge chain starting at the given edge
    pub fn edges_from<'a>(&'a self, e: T::E) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::<'a, T>::new(*self.edge(e), self)
    }

    /// Iterates backwards over the half-edge chain starting at the given edge
    pub fn edges_back_from<'a>(&'a self, e: T::E) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::<'a, T>::new(*self.edge(e), self)
    }
}
