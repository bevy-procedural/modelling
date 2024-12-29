use crate::{
    math::Scalar,
    mesh::{EuclideanMeshType, MeshType, VertexBasics},
};

/// Basic edge traits for a mesh. Can be directed or undirected.
pub trait EdgeBasics<T: MeshType<Edge = Self>>: std::fmt::Debug + Clone {
    /// Returns the identifier of the edge
    fn id(&self) -> T::E;

    /// Whether the edge payload is considered empty or, if this is a half-edge,
    /// the edge payload is stored in the twin.
    fn payload_self_empty(&self) -> bool;

    /// Returns the source vertex of the edge. If it is not directed, can be either vertex but not the same as the target.
    fn origin<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns whether the edge (i.e., this HalfEdge or its twin) is a boundary edge, i.e., adjacent to a hole.
    fn is_boundary(&self, mesh: &T::Mesh) -> bool;

    /// Returns the edge payload if it exists.
    /// Might be `None` if the payload is stored in the twin.
    /// Usually, you should use `Mesh::edge_payload` to get the payload.
    fn payload_self(&self) -> Option<&T::EP>;

    /// Returns the edge payload if it exists.
    /// Might be `None` if the payload is stored in the twin.
    /// Usually, you should use `Mesh::edge_payload_mut` to get the payload.
    fn payload_self_mut<'a>(&'a mut self) -> Option<&'a mut T::EP>;

    /// Returns the centroid of the edge.
    fn centroid<const D: usize>(&self, mesh: &T::Mesh) -> T::Vec
    where
        T: EuclideanMeshType<D>,
    {
        let v1 = self.origin(mesh).pos().clone();
        let v2 = self.target(mesh).pos().clone();
        (v1 + v2) * T::S::HALF
    }

    /// Iterates all (half)edges incident to the same face or boundary (counter-clockwise)
    fn edges_face<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge> + 'a
    where
        T: 'a;

    /// Iterates all (half)edges incident to the same face or boundary (clockwise)
    fn edges_face_back<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = &'a T::Edge> + 'a
    where
        T: 'a;

    /// Iterates all face ids incident to the edge
    /// (even for half-edges, this will return both faces if there are two
    /// or more than that if the edge is non-manifold)
    fn face_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::F> + 'a;
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use itertools::Itertools;

    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_edge_basics_triangle() {
        let mesh = Mesh3d64::regular_polygon(1.0, 3);
        let edge = mesh.edge(0);
        assert_eq!(edge.origin(&mesh).id(), 0);
        assert_eq!(edge.target(&mesh).id(), 1);
        assert_eq!(edge.twin(&mesh).id(), 1);
        assert_eq!(edge.is_boundary(&mesh), true);
        assert_eq!(mesh.edge_payload(edge).is_empty(), true);
        assert_eq!(edge.face_ids(&mesh).collect::<Vec<_>>(), vec![0]);
        assert!(edge.edges_face(&mesh).count() == 3);
        assert!(edge.edges_face_back(&mesh).count() == 3);
        assert_eq!(
            edge.edges_face(&mesh)
                .map(|e| e.id())
                .collect_vec()
                .iter()
                .rev()
                .collect_vec(),
            edge.edges_face_back(&mesh)
                .map(|e| e.id())
                .collect_vec()
                .iter()
                .cycle()
                .skip(1)
                .take(3)
                .collect_vec()
        );
        for edge in mesh.edges() {
            assert!(edge.is_boundary(&mesh));
            assert_eq!(edge.face_ids(&mesh).count(), 1);
            assert!(edge.edges_face(&mesh).count() == 3);
        }
    }

    #[test]
    fn test_edge_basics_cube() {
        let mesh = Mesh3d64::cube(1.0);
        for edge in mesh.edges() {
            assert!(!edge.is_boundary(&mesh));
            assert_eq!(edge.face_ids(&mesh).count(), 2);
            assert!(edge.edges_face(&mesh).count() == 4);
            assert!(edge.edges_face_back(&mesh).count() == 4);
        }
    }
}
