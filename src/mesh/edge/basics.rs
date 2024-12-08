use crate::{
    math::Scalar,
    mesh::{EuclideanMeshType, MeshType, VertexBasics},
};

/// Basic edge traits for a mesh. Can be directed or undirected.
pub trait EdgeBasics<T: MeshType<Edge = Self>>: std::fmt::Debug + Clone {
    /// Returns the identifier of the edge
    fn id(&self) -> T::E;

    /// Returns the face payload.
    fn payload(&self) -> &T::EP;

    /// Returns a mutable reference to the face payload.
    fn payload_mut(&mut self) -> &mut T::EP;

    /// Returns the source vertex of the edge. If it is not directed, can be either vertex but not the same as the target.
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge, i.e., adjacent to a hole.
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;

    /// Returns the centroid of the edge.
    fn centroid<const D: usize>(&self, mesh: &T::Mesh) -> T::Vec
    where
        T: EuclideanMeshType<D>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * T::S::HALF
    }

    /// Iterates all (half)edges incident to the same face (counter-clockwise)
    fn edges_face<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Edge>;

    /// Iterates all (half)edges incident to the same face (clockwise)
    fn edges_face_back<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Edge>;

    /// Iterates all face ids incident to the edge 
    /// (even for half-edges, this will return both faces if there are two 
    /// or more than that if the edge is non-manifold)
    fn face_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::F>;
}
