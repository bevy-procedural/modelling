use crate::{
    halfedge::{HalfEdgeMesh, HalfEdgeMeshType},
    math::{Scalar, Transformable},
    mesh::{DefaultEdgePayload, DefaultFacePayload, EdgeBasics, MeshBasics, VertexBasics},
};
use itertools::Itertools;

//pub trait Extrusions<T: MeshType<Mesh = Self>>: MeshTrait<T = T>
impl<T: HalfEdgeMeshType> HalfEdgeMesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: Transformable<Trans = T::Trans, S = T::S>,
{
    /// Extrudes the given edge using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses one row of quad faces.
    pub fn extrude(&mut self, e: T::E, transform: T::Trans) -> T::E {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let vps: Vec<_> = self
            .edges_back_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .collect();
        let start = self.loft_polygon_back(e, 2, 2, vps);
        self.close_hole(start, Default::default(), false);
        start
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    pub fn extrude_face(&mut self, f: T::F, transform: T::Trans) -> T::E {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude(e, transform);
    }

    /// Remove the given face and extrude the boundary using the given transformation.
    pub fn extrude_tri_face(&mut self, f: T::F, transform: T::Trans) -> T::E {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude_tri(e, transform);
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    pub fn extrude_tri(&mut self, e: T::E, transform: T::Trans) -> T::E {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let vps: Vec<_> = self
            .edges_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .collect();
        let start = self.loft_tri_closed(e, vps);
        self.close_hole(start, Default::default(), false);
        start
    }

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    pub fn extrude_tri2(&mut self, e: T::E, transform: T::Trans) -> T::E {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let mut vps: Vec<_> = self
            .edges_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transformed(&transform))
            .circular_tuple_windows()
            .map(|(a, b)| a.lerped(&b, T::S::HALF))
            .collect();
        vps.rotate_right(1);
        let start = self.loft_tri_closed(e, vps);
        self.close_hole(start, Default::default(), false);
        start
    }
}

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `apex` with the given vp and fill the hole along the boundary with triangles connected to the apex vertex.
    /// Returns the id of the apex vertex.
    pub fn fill_hole_apex(&mut self, start: T::E, apex: T::VP) -> T::V {
        // TODO: replace with loft n=1
        let e0 = self.edge(start);
        let origin = e0.origin_id();
        let mut input = self.edge(start).prev_id();
        let (v, _, _) = self.add_vertex_via_edge_default(input, start, apex);
        loop {
            let e = self.edge(input);
            if e.origin_id() == origin {
                break;
            }
            input = e.prev_id();
            self.close_face_default(self.edge(input).next(&self).next_id(), input, false);
        }
        self.close_hole(input, Default::default(), false);
        v
    }
}
