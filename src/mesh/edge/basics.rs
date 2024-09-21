use crate::mesh::MeshType;

/// Basic edge traits for a mesh
pub trait EdgeBasics<T: MeshType<Edge = Self>>: std::fmt::Debug + Clone + Copy + PartialEq {
    /// Returns the index of the edge
    fn id(&self) -> T::E;

    /// Returns the source vertex of the edge. If it is not directed, can be either vertex but not the same as the target.
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;
}
