use crate::{
    math::{Scalar, Transformable},
    mesh::{
        CursorData, DefaultEdgePayload, DefaultFacePayload, EdgeBasics, EdgeCursorBasics,
        EdgeCursorHalfedgeBasics, EdgeCursorMut, EuclideanMeshType, FaceCursorBasics, HalfEdge,
        MeshTypeHalfEdge, VertexBasics,
    },
    operations::MeshLoft,
};
use itertools::Itertools;

// TODO: Adjust this to not be halfedge-specific

/// Extrude operations for meshes.
pub trait MeshExtrude<T: MeshTypeHalfEdge<Mesh = Self>>: MeshLoft<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Extrudes all boundary edges using the given transformation.
    ///
    /// Uses one row of quad faces.
    fn extrude_boundary<const D: usize>(&mut self, transform: T::Trans)
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        let faces = self.face_ids().collect_vec();
        for f in faces {
            let e = self.face(f).edge().twin_id();
            if self.edge(e).unwrap().is_boundary_self() {
                self.extrude(e, transform);
            }
        }
    }

    /// Extrudes the given edge using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses one row of quad faces.
    fn extrude<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> EdgeCursorMut<'_, T>
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let vps: Vec<_> = self
            .edges_back_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .collect();
        let start = self.loft_back(e, 2, 2, vps).unwrap().0; // TODO
        self.insert_face(start, Default::default()).unwrap(); // TODO
        self.edge_mut(start)
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    fn extrude_face<const D: usize>(&mut self, f: T::F, transform: T::Trans) -> EdgeCursorMut<'_, T>
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        self.extrude(e, transform)
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    fn extrude_tri_face<const D: usize>(
        &mut self,
        f: T::F,
        transform: T::Trans,
    ) -> EdgeCursorMut<'_, T>
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        self.extrude_tri(e, transform)
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    fn extrude_tri<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> EdgeCursorMut<'_, T>
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let vps: Vec<_> = self
            .edges_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .collect();
        let start = self.loft_tri(e, false, true, vps).unwrap();
        self.insert_face(start, Default::default()).unwrap();
        self.edge_mut(start)
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    fn extrude_tri2<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> EdgeCursorMut<'_, T>
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting

        let mut vps: Vec<_> = self
            .edges_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .collect::<Vec<_>>()
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| a.lerped(&b, T::S::HALF))
            .collect();
        vps.rotate_right(1);
        let start = self.loft_tri(e, false, true, vps).unwrap();
        self.insert_face(start, Default::default()).unwrap();
        self.edge_mut(start)
    }

    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `hub` with the given vp and fill the hole along the boundary with triangles connected to the hub vertex.
    /// Returns the id of the hub vertex.
    ///
    /// The result will be a windmill with triangular blades.
    #[must_use]
    fn windmill_back(&mut self, start: T::E, hub: T::VP) -> Option<T::V> {
        // TODO: replace with loft n=1
        let start = self.edge(start);
        let origin = start.origin_id();
        let mut input = start.prev_id();
        let (_, v) = self.insert_vertex_e(input, hub, Default::default())?;
        loop {
            let e = self.edge(input);
            if e.origin_id() == origin {
                break;
            }
            input = e.prev_id();
            self.close_face_ee(e.next_id(), e.id(), Default::default(), Default::default())?;
        }
        self.insert_face(input, Default::default())?;
        Some(v)
    }

    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `hub` with the given vp and fill the hole along the boundary with triangles connected to the hub vertex.
    /// Returns the id of the hub vertex.
    ///
    /// The result will be a windmill with triangular blades.
    #[must_use]
    fn windmill(&mut self, start: T::E, hub: T::VP) -> Option<T::V> {
        // TODO: replace with loft n=1
        let start = self.edge(start);
        let target = start.target_id();
        let mut input = start.next_id();
        let (_, v) = self.insert_vertex_e(start.id(), hub, Default::default())?;
        loop {
            let e = self.edge(input);
            if e.target_id() == target {
                break;
            }
            input = e.next_id();
            self.close_face_ee(e.id(), e.prev_id(), Default::default(), Default::default())?;
        }
        self.insert_face(input, Default::default())?;
        Some(v)
    }
}
