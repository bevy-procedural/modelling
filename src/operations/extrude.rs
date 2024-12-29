use crate::{
    math::{Scalar, Transformable},
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, EdgeBasics, EuclideanMeshType, FaceBasics,
        HalfEdge, MeshTypeHalfEdge, VertexBasics,
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
            let e = self.face(f).edge(self).twin_id();
            if self.edge(e).is_boundary_self() {
                self.extrude(e, transform);
            }
        }
    }

    /// Extrudes the given edge using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses one row of quad faces.
    fn extrude<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> T::E
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
        let start = self.loft_polygon_back(e, 2, 2, vps);
        self.insert_face(start, Default::default());
        start
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    fn extrude_face<const D: usize>(&mut self, f: T::F, transform: T::Trans) -> T::E
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude(e, transform);
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    fn extrude_tri_face<const D: usize>(&mut self, f: T::F, transform: T::Trans) -> T::E
    where
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
        T: EuclideanMeshType<D, Mesh = Self>,
    {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude_tri(e, transform);
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    fn extrude_tri<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> T::E
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
        let start = self.loft_tri_closed(e, vps);
        self.insert_face(start, Default::default());
        start
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    fn extrude_tri2<const D: usize>(&mut self, e: T::E, transform: T::Trans) -> T::E
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
        let start = self.loft_tri_closed(e, vps);
        self.insert_face(start, Default::default());
        start
    }

    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `hub` with the given vp and fill the hole along the boundary with triangles connected to the hub vertex.
    /// Returns the id of the hub vertex.
    /// 
    /// The result will be a windmill with triangular blades.
    fn windmill(&mut self, start: T::E, hub: T::VP) -> T::V {
        // TODO: replace with loft n=1
        let e0 = self.edge(start);
        let origin = e0.origin_id();
        let mut input = self.edge(start).prev_id();
        let (_, v) = self
            .insert_vertex_e(input, hub, Default::default())
            .unwrap(); // TODO: error handling
        loop {
            let e = self.edge(input);
            if e.origin_id() == origin {
                break;
            }
            input = e.prev_id();
            self.close_face_ee_legacy(
                self.edge(input).next(&self).next_id(),
                input,
                Default::default(),
                Default::default(),
            );
        }
        self.insert_face(input, Default::default());
        v
    }
}
