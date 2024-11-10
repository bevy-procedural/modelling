use crate::{
    math::{HasPosition, Scalar},
    mesh::{MeshType, VertexBasics},
};

/// Basic edge traits for a mesh. Can be directed or undirected.
pub trait EdgeBasics<T: MeshType<Edge = Self>>: std::fmt::Debug + Clone + Copy + PartialEq {
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
    fn centroid(&self, mesh: &T::Mesh) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * T::S::HALF
    }
}
