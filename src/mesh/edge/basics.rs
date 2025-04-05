use crate::{
    math::Scalar,
    mesh::{EuclideanMeshType, MeshType, VertexBasics},
    util::CreateEmptyIterator,
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

    /// Returns the source vertex of the half-edge
    fn origin_id(&self, mesh: &T::Mesh) -> T::V;

    /// Returns the target vertex of the edge. If it is not directed, can be either vertex but not the same as the origin.
    fn target<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Vertex;

    /// Returns the target vertex id of the half-edge. Reached via the next half-edge, not the twin.
    fn target_id(&self, mesh: &T::Mesh) -> T::V;

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

    type BoundaryIterator<'a>: Iterator<Item = &'a T::Edge> + CreateEmptyIterator + 'a
    where
        Self: 'a,
        T: 'a;

    /// Iterates all (half)edges incident to the same face or boundary (counter-clockwise)
    fn boundary<'a>(&'a self, mesh: &'a T::Mesh) -> Self::BoundaryIterator<'a>
    where
        T: 'a;

    type BoundaryBackIterator<'a>: Iterator<Item = &'a T::Edge> + CreateEmptyIterator + 'a
    where
        Self: 'a,
        T: 'a;

    /// Iterates all (half)edges incident to the same face or boundary (clockwise)
    fn boundary_back<'a>(&'a self, mesh: &'a T::Mesh) -> Self::BoundaryBackIterator<'a>
    where
        T: 'a;

    /// Iterator type for face ids incident to the edge
    type FaceIdIterator<'a>: Iterator<Item = T::F> + CreateEmptyIterator + 'a
    where
        Self: 'a,
        T: 'a;

    /// Iterates all face ids incident to the edge
    /// (even for half-edges, this will return both faces if there are two
    /// or more than that if the edge is non-manifold)
    fn face_ids<'a>(&'a self, mesh: &'a T::Mesh) -> Self::FaceIdIterator<'a>;

    /// Determines whether the edge is manifold, i.e., has one or two incident faces.
    /// Degenerate edges where the same face is incident twice are not considered manifold.
    fn is_manifold(&self, mesh: &T::Mesh) -> bool {
        let mut faces = self.face_ids(mesh);
        let Some(f1) = faces.next() else {
            // No faces at all
            return false;
        };
        let Some(f2) = faces.next() else {
            // Only one face
            return true;
        };
        if faces.next().is_some() {
            // More than two faces
            return false;
        }
        // Two faces, but not the same face twice
        return f1 != f2;
    }
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use crate::{extensions::nalgebra::*, math::impls::EU, prelude::*};

    #[test]
    fn test_edge_basics_triangle() {
        let mesh = Mesh3d64::regular_polygon(1.0, 3);
        let edge = mesh.edge(IndexType::new(0)).unwrap();
        assert_eq!(mesh.check(), Ok(()));
        assert_eq!(edge.origin_id(), IndexType::new(0));
        assert_eq!(edge.target_id(), IndexType::new(1));
        assert_eq!(edge.twin_id(), IndexType::new(1));
        assert_eq!(edge.is_boundary(), true);
        assert_eq!(edge.payload().is_empty(), true);

        // TODO: Cursor
        assert_eq!(edge.face_ids().collect::<Vec<_>>(), vec![IndexType::new(0)]);
        assert!(edge.boundary().count() == 3);
        assert!(edge.boundary_back().count() == 3);
        assert_eq!(
            edge.boundary()
                .map(|e| e.id())
                .collect::<Vec<EU>>()
                .iter()
                .rev()
                .collect::<Vec<&EU>>(),
            edge.boundary_back()
                .map(|e| e.id())
                .collect::<Vec<EU>>()
                .iter()
                .cycle()
                .skip(1)
                .take(3)
                .collect::<Vec<&EU>>()
        );
        for edge in mesh.halfedges() {
            assert!(edge.is_boundary());
            assert_eq!(edge.fork().face_ids().count(), 1);
            assert_eq!(edge.boundary().count(), 3);
        }
    }

    #[test]
    fn test_edge_basics_cube() {
        let mesh = Mesh3d64::cube(1.0);
        assert_eq!(mesh.check(), Ok(()));
        for edge in mesh.halfedges() {
            assert!(!edge.is_boundary());
            assert_eq!(edge.face_ids().count(), 2);
            assert_eq!(edge.boundary().count(), 4);
            assert_eq!(edge.boundary_back().count(), 4);
            assert!(edge
                .face()
                .unwrap()
                .inner()
                .polygon::<2>(&mesh)
                .area()
                .is_about(1.0, 1e-6));
        }
    }
}
