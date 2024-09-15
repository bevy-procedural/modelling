use itertools::Itertools;

use crate::{
    math::Scalar,
    representation::{
        payload::{HasPosition, Transformable},
        DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
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
            .map(|v| v.origin(self).payload().transform(&transform))
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

    /// Extrudes the given boundary using the given transformation.
    /// Returns an edge on the boundary of the extrusion.
    ///
    /// Uses two rows of triangle faces.
    pub fn extrude_tri(&mut self, e: T::E, transform: T::Trans) -> T::E {
        assert!(self.edge(e).is_boundary_self());
        // TODO: avoid collecting
        let mut vps: Vec<_> = self
            .edges_from(self.edge(e).next_id())
            .map(|v| v.origin(self).payload().transform(&transform))
            .circular_tuple_windows()
            .map(|(a, b)| a.lerp(&b, T::S::from(0.5)))
            .collect();
        vps.rotate_right(1);
        let start = self.loft_tri_closed(e, vps);
        self.close_hole(start, Default::default(), false);
        start
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `center` with the given vp and fill the hole along the boundary with triangles connected to the center vertex.
    /// Returns the vertex.
    pub fn fill_hole_with_vertex(&mut self, start: T::E, center: T::VP) -> T::V {
        // TODO: replace with loft n=1
        let e0 = self.edge(start);
        let origin = e0.origin_id();
        let mut input = self.edge(start).prev_id();
        let (v, _, _) = self.add_vertex_via_edge_default(input, start, center);
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

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Create a vertex by translating the center of the given face and connect the given face to that vertex.
    pub fn extrude_to_center_point(&mut self, e: T::E, translation: T::Vec) -> T::V {
        let f = if self.edge(e).is_boundary_self() {
            self.close_hole(e, Default::default(), true)
        } else {
            self.edge(e).face_id()
        };
        let p = T::VP::from_pos(self.face(f).center(self) + translation);
        self.remove_face(f);
        self.fill_hole_with_vertex(e, p)
    }
}
