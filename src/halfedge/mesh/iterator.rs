use super::HalfEdgeMesh;
use crate::{
    halfedge::{HalfEdgeMeshType, IncidentToFaceBackIterator, IncidentToFaceIterator},
    mesh::{EdgeBasics, MeshBasics},
    util::Deletable,
};

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn twin_edges<'a>(&'a self) -> impl Iterator<Item = (&'a T::Edge, &'a T::Edge)>
    where
        T::Edge: 'a,
    {
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

    /// Iterates forwards over the half-edge chain starting at the given edge
    pub fn edges_from<'a>(&'a self, e: T::E) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::<'a, T>::new(*self.edge(e), self)
    }

    /// Iterates backwards over the half-edge chain starting at the given edge
    pub fn edges_back_from<'a>(&'a self, e: T::E) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::<'a, T>::new(*self.edge(e), self)
    }
}
