mod payload;

pub use payload::*;

use super::{MeshType, Vertex};
use crate::math::{HasPosition, Scalar};

/// A directed (e.g., halfedge) or undirected edge in a mesh.
pub trait Edge<T: MeshType<Edge = Self>>: std::fmt::Display + Clone + Copy + PartialEq {
    /// Returns the index of the edge
    fn id(&self) -> T::E;

    /// Returns the source vertex of the edge. If it is not directed, can be either vertex but not the same as the target.
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target(&self, mesh: &T::Mesh) -> T::Vertex;

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;

    fn center(&self, mesh: &T::Mesh) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * T::S::HALF
    }
}
