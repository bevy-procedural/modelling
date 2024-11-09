use super::EdgeBasics;
use crate::mesh::MeshType;

/// Basic halfedge traits.
pub trait HalfEdge<T: MeshType<Edge = Self>>: EdgeBasics<T> {
    /// Creates a new half-edge
    fn new(next: T::E, twin: T::E, prev: T::E, origin: T::V, face: T::F, payload: T::EP) -> Self;

    /// Sets the face of the HalfEdge. Panics if the face is already set.
    fn set_face(&mut self, face: T::F);

    /// Deletes the face of the HalfEdge
    fn delete_face(&mut self);

    /// Sets the next half-edge incident to the same face (including the holes)
    fn set_next(&mut self, next: T::E);

    /// Sets the previous half-edge incident to the same face (including the holes)
    fn set_prev(&mut self, prev: T::E);

    /// Sets the twin half-edge
    fn set_twin(&mut self, twin: T::E);

    /// Returns the next half-edge incident to the same face or boundary
    fn next(&self, mesh: &T::Mesh) -> T::Edge;

    /// Returns the next id
    fn next_id(&self) -> T::E;

    /// Returns the other, opposite half-edge
    fn twin(&self, mesh: &T::Mesh) -> T::Edge;

    /// Returns the twin id
    fn twin_id(&self) -> T::E;

    /// Returns the previous half-edge incident to the same face or boundary
    fn prev(&self, mesh: &T::Mesh) -> T::Edge;

    /// Returns the prev id
    fn prev_id(&self) -> T::E;

    /// Returns the source vertex of the half-edge
    fn origin_id(&self) -> T::V;

    /// Returns the target vertex id of the half-edge. Reached via the next half-edge, not the twin.
    fn target_id(&self, mesh: &T::Mesh) -> T::V;

    /// Returns the face the half-edge is incident to
    fn face<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Face>;

    /// Returns the face id
    fn face_id(&self) -> T::F;

    /// Returns the other face (incident to the twin)
    fn other_face<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Face>;

    /// Returns whether the edge (i.e., this HalfEdge and not necessarily its twin) is a boundary edge
    fn is_boundary_self(&self) -> bool;

    /// Returns whether the edge can reach the vertex when searching counter-clockwise along the face
    fn same_face(&self, mesh: &T::Mesh, v: T::V) -> bool;

    /// Like `same_face` but searches clockwise
    fn same_face_back(&self, mesh: &T::Mesh, v: T::V) -> bool;

    /// Flips the direction of the edge and its twin
    fn flip(e: T::E, mesh: &mut T::Mesh);
}
