// TODO: iterator for neighboring faces

use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::MeshType,
};

/// Basic vertex functionality for a mesh
pub trait VertexBasics<T: MeshType>: std::fmt::Debug + Clone {
    /// Returns the index of the vertex
    fn id(&self) -> T::V;

    /// Returns the payload of the vertex
    fn payload(&self) -> &T::VP;

    /// Returns whether the vertex is isolated, i.e., has no edges incident to it
    fn is_isolated(&self, mesh: &T::Mesh) -> bool {
        self.edges_out(mesh).next().is_none()
    }

    /// Returns the vertex coordinates of the payload
    fn pos<S: Scalar, const D: usize, Vec: Vector<S, D>>(&self) -> Vec
    where
        T::VP: HasPosition<D, Vec, S = S>,
    {
        *self.payload().pos()
    }

    /// Returns a mutable reference to the payload of the vertex
    fn payload_mut(&mut self) -> &mut T::VP;

    /// Returns an outgoing edge incident to the vertex
    fn edge_id(&self, mesh: &T::Mesh) -> T::E;

    /// Returns an outgoing edge incident to the vertex
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Edge>;

    /// Returns whether the vertex is a boundary vertex
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;

    /// Returns whether the vertex has only one edge incident to it
    fn has_only_one_edge(&self, mesh: &T::Mesh) -> bool;

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    fn vertices<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a;

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    fn faces<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Face> + 'a
    where
        T: 'a;

    /// Iterates the ids of all neighbors of the vertex
    fn neighbor_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::V> + 'a
    where
        T: 'a,
    {
        self.vertices(mesh).map(|v| v.id())
    }

    /// Returns the degree of the vertex
    fn degree(&self, mesh: &T::Mesh) -> usize {
        self.edges_out(mesh).count()
    }

    /// Iterates all outgoing (half)edges (resp. all edges in outwards-direction
    /// if undirected) incident to this vertex (clockwise)
    fn edges_out<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Iterates all ingoing (half)edges (resp. all edges in inwards-direction
    /// if undirected) incident to this vertex (clockwise)
    fn edges_in<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /*
    /// Iterates the wheel of vertices (will have length one if the vertex is manifold)
    #[inline(always)]
    pub fn wheel<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = Vertex<E, V, VP>> + 'a {
        NonmanifoldVertexIterator::new(self.clone(), mesh)
    }*/
}
