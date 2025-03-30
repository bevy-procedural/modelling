use super::EdgeBasics;
use crate::{math::IndexType, mesh::MeshType};

/// Basic halfedge traits.
pub trait HalfEdge<T: MeshType<Edge = Self>>: EdgeBasics<T> {
    /// Creates a new half-edge
    fn new(
        next: T::E,
        twin: T::E,
        prev: T::E,
        origin: T::V,
        face: T::F,
        payload: Option<T::EP>,
    ) -> Self;

    /// Sets the face of the HalfEdge.
    /// Won't panic if the face is already set.
    fn set_face(&mut self, face: T::F);

    /// Removes the face of the HalfEdge.
    /// Won't delete the face itself.
    /// Panics if the face is not set.
    fn remove_face(&mut self) {
        assert!(self.has_face());
        self.set_face(IndexType::max());
    }

    /// Returns whether the half-edge has a face
    fn has_face(&self) -> bool {
        self.face_id() != IndexType::max()
    }

    /// Sets the next half-edge incident to the same face (including the holes)
    fn set_next(&mut self, next: T::E);

    /// Sets the previous half-edge incident to the same face (including the holes)
    fn set_prev(&mut self, prev: T::E);

    /// Sets the twin half-edge
    fn set_twin(&mut self, twin: T::E);

    /// Sets the origin vertex of the half-edge
    fn set_origin(&mut self, origin: T::V);

    /// Returns the next half-edge incident to the same face or boundary
    fn next<'a>(&self, mesh: &'a T::Mesh) -> &'a T::Edge;

    /// Returns the next id
    fn next_id(&self) -> T::E;

    /// Returns the other, opposite half-edge
    fn twin<'a>(&self, mesh: &'a T::Mesh) -> &'a T::Edge;

    /// Returns the twin id
    fn twin_id(&self) -> T::E;

    /// Returns the previous half-edge incident to the same face or boundary
    fn prev<'a>(&self, mesh: &'a T::Mesh) -> &'a T::Edge;

    /// Returns the prev id
    fn prev_id(&self) -> T::E;

    /// Returns the face the half-edge is incident to
    fn face<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Face>;

    /// Returns the face id
    fn face_id(&self) -> T::F;

    /// Returns the other face (incident to the twin)
    fn other_face<'a>(&'a self, mesh: &'a T::Mesh) -> Option<&'a T::Face>;

    /// Returns whether the edge (i.e., this HalfEdge and not necessarily its twin) is a boundary edge
    fn is_boundary_self(&self) -> bool;

    /// Returns an outgoing edge from `v` that is part of the same boundary as `self`.
    /// Returns `None` if no such edge exists.
    /// Traverses the boundary forwards.
    fn same_boundary(&self, mesh: &T::Mesh, v: T::V) -> Option<T::E> {
        self.boundary(mesh)
            .find(|e| e.origin_id(mesh) == v)
            .map(|e| e.id())
    }
    /// Returns an outgoing edge from `v` that is part of the same boundary as `self`.
    /// Returns `None` if no such edge exists.
    /// Traverses the boundary backwards.
    fn same_boundary_back(&self, mesh: &T::Mesh, v: T::V) -> Option<T::E> {
        self.boundary_back(mesh)
            .find(|e| e.origin_id(mesh) == v)
            .map(|e| e.id())
    }

    /// Flips the direction of the edge and its twin.
    /// Updates the neighboring edges, vertices, and faces.
    /// 
    /// Panics if the edge or its twin does not exist.
    fn flip(e: T::E, mesh: &mut T::Mesh);

    /// Checks whether the adjacent edges, the face and the twin exist and don't
    /// contradict the current edge regarding the neighbors, face, and origin.
    /// Returns an error message if the edge is invalid.
    ///
    /// Iterates the edge wheels of the origin and target vertices! So this is not O(1).
    fn check(&self, mesh: &T::Mesh) -> Result<(), String>;
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_halfedge_triangle() {
        let mut mesh = Mesh3d64::regular_polygon(1.0, 3);
        for edge in mesh.halfedges() {
            assert!(edge.is_boundary_self() ^ (edge.fork().twin().is_boundary_self()));
            if edge.is_boundary_self() {
                assert!(edge.fork().twin().has_face());
            }
        }

        let e0 = mesh
            .halfedges()
            .find(|e| !e.is_boundary_self())
            .unwrap()
            .id();
        let f0 = mesh.edge(e0).face_id();
        mesh.edge_mut(e0).remove_face();
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
        for edge in mesh.halfedges() {
            assert!(!edge.is_boundary_self());
            assert!(edge.twin().has_face());
        }

        for face in mesh.faces() {
            face.edges().for_each(|e1| {
                face.edges().for_each(|e2| {
                    assert!(e1.clone().same_boundary(e2.origin_id()).is_some());
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
