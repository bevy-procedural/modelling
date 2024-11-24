use crate::mesh::{EdgeBasics, HalfEdge};

use super::{MeshBasics, MeshType};

/// Some basic operations to retrieve information about the mesh.
pub trait HalfEdgeMesh<T: MeshType<Mesh = Self>>: MeshBasics<T>
where
    T::Edge: HalfEdge<T>,
{
    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn twin_edges<'a>(&'a self) -> impl Iterator<Item = (&'a T::Edge, &'a T::Edge)>
    where
        T::Edge: 'a,
        T: 'a,
    {
        self.edges().filter_map(move |e| {
            if e.twin_id() < e.id() {
                None
            } else {
                Some((e, self.edge(e.twin_id())))
            }
        })
    }

    /// Iterates forwards over the half-edge chain starting at the given edge
    fn edges_from<'a>(&'a self, e: T::E) -> impl Iterator<Item = T::Edge>;

    /// Iterates backwards over the half-edge chain starting at the given edge
    fn edges_back_from<'a>(&'a self, e: T::E) -> impl Iterator<Item = T::Edge>;

    /// Flips the edge, i.e., swaps the origin and target vertices.
    fn flip_edge(&mut self, e: T::E) -> &mut Self {
        HalfEdge::<T>::flip(e, self);
        self
    }

    /// Flip all edges (and faces) turning the mesh inside out.
    fn flip(&mut self) -> &mut Self {
        // TODO: this is an unnecessary clone
        let ids: Vec<T::E> = self.edges().map(|e| e.id()).collect();
        ids.iter().for_each(|&e| {
            self.flip_edge(e);
        });
        self
    }
}
