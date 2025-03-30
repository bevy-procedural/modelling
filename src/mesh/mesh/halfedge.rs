use crate::mesh::{cursor::*, EdgeBasics, HalfEdge};

use super::{MeshBasics, MeshType};

/// Some basic operations to retrieve information about the mesh.
pub trait HalfEdgeMesh<T: MeshType<Mesh = Self>>: MeshBasics<T>
where
    T::Edge: HalfEdge<T>,
{
    /// Returns an iterator over all non-deleted halfedges.
    #[must_use]
    fn halfedge_refs<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted halfedges.
    #[must_use]
    fn halfedges<'a>(&'a self) -> impl Iterator<Item = ValidEdgeCursor<'a, T>>
    where
        T: 'a,
    {
        self.halfedge_ids()
            .map(move |e| ValidEdgeCursor::new(self, self.edge_ref(e)))
    }

    /// Returns an iterator over all non-deleted halfedge's ids
    #[inline]
    #[must_use]
    fn halfedge_ids<'a>(&'a self) -> impl Iterator<Item = T::E>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.halfedge_refs().map(|e| e.id())
    }

    /// Returns the number of halfedges in the mesh
    #[must_use]
    fn num_halfedges(&self) -> usize;

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn twin_edges<'a>(&'a self) -> impl Iterator<Item = (&'a T::Edge, &'a T::Edge)>
    where
        T::Edge: 'a,
        T: 'a,
    {
        self.halfedge_refs().filter_map(move |e| {
            if e.twin_id() < e.id() {
                None
            } else {
                Some((e, self.edge_ref(e.twin_id())))
            }
        })
    }

    /// Iterates forwards over the half-edge chain starting at the given edge.
    /// Notice that this only makes sense for half-edge meshes since, unless there is a face,
    /// edge-connectivity at vertices is not stored in the mesh.
    fn edges_from<'a>(&'a self, e: T::E) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Iterates backwards over the half-edge chain starting at the given edge
    fn edges_back_from<'a>(&'a self, e: T::E) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Flips the edge, i.e., swaps the origin and target vertices.
    fn flip_edge(&mut self, e: T::E) -> &mut Self {
        HalfEdge::<T>::flip(e, self);
        self
    }

    /// Returns a flipped clone of the mesh.
    fn flipped(&self) -> Self {
        let mut mesh = self.clone();
        mesh.flip();
        mesh
    }

    /// Flip all edges. The mesh won't change its topology, but the indices of all edges and their payloads will be swapped.
    fn flip(&mut self) -> &mut Self {
        // PERF: this is an unnecessary clone
        let ids: Vec<T::E> = self.twin_edges().map(|(e, _)| e.id()).collect();
        ids.iter().for_each(|&e| {
            self.flip_edge(e);
        });
        self
    }
}
