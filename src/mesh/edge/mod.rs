mod basics;
mod halfedge;
mod payload;

pub use basics::*;
pub use halfedge::*;
pub use payload::*;

use super::MeshType;

/// A directed or undirected edge or halfedge in a mesh.
///
/// More specifically:
/// - In an undirected graph, there is exactly one edge id for each undirected edge. Instances of the edge should can be different, i.e., `target` and `origin` can be swapped.
/// - In a directed graph, there is exactly one edge id for each directed edge. An undirected edge therefore is a pair of `Edge`s, each with their own id.
/// - In a halfedge mesh, there is exactly one edge id for each halfedge. An undirected edge therefore is a pair of `Edge`s, each with their own id.
pub trait Edge: EdgeBasics<Self::T> {
    /// Associated mesh type
    type T: MeshType<Edge = Self>;
}
