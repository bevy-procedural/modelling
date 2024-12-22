use super::EdgeBasics;
use crate::mesh::MeshType;

/// Basic halfedge traits.
pub trait HalfEdge<T: MeshType<Edge = Self>>: EdgeBasics<T> {
    /// Creates a new half-edge
    fn new(next: T::E, twin: T::E, prev: T::E, origin: T::V, face: T::F, payload: Option<T::EP>) -> Self;

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

    /// Sets the origin vertex of the half-edge
    fn set_origin(&mut self, origin: T::V);

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

    /// Flips the direction of the edge and its twin.
    /// Updates the neighboring edges, vertices, and faces.
    fn flip(e: T::E, mesh: &mut T::Mesh);
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use super::*;
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_halfedge_triangle() {
        let mut mesh = Mesh3d64::regular_polygon(1.0, 3);
        for edge in mesh.edges() {
            assert!(edge.is_boundary_self() ^ (edge.twin(&mesh).is_boundary_self()));
            if edge.is_boundary_self() {
                assert!(edge.other_face(&mesh).is_some());
            }
        }

        let e0 = mesh.edges().find(|e| !e.is_boundary_self()).unwrap().id();
        let f0 = mesh.edge(e0).face_id();
        mesh.edge_mut(e0).delete_face();
        let edge = mesh.edge(e0);
        assert!(edge.is_boundary_self());

        mesh.edge_mut(e0).set_face(f0);
        let edge = mesh.edge(e0);
        assert!(!edge.is_boundary_self());

        assert!(mesh.flipped().check().is_ok());
        assert!(mesh
            .flipped()
            .flipped()
            .is_trivially_isomorphic_pos(&mesh, 1e-6)
            .eq());
    }

    #[test]
    fn test_halfedge_cube() {
        let mesh = Mesh3d64::cube(1.0);
        for edge in mesh.edges() {
            assert!(!edge.is_boundary_self());
            assert!(edge.other_face(&mesh).is_some());
        }

        for face in mesh.faces() {
            face.edges(&mesh).for_each(|e1| {
                face.edges(&mesh).for_each(|e2| {
                    assert!(e1.same_face(&mesh, e2.origin_id()));
                });
            });
        }

        let flipped = mesh.flipped();
        assert!(flipped.check().is_ok());
        assert!(flipped
            .flipped()
            .is_trivially_isomorphic_pos(&mesh, 1e-6)
            .eq());
        assert!(flipped
            .is_isomorphic_by_pos::<_, 3, _, MeshType3d64PNU>(&mesh, 1e-6)
            .eq());
    }
}
