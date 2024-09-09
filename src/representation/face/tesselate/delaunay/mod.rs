use std::collections::HashMap;

use super::{Face, Mesh, Payload, Triangulation};
use crate::{
    math::{Scalar, Vector, Vector3D},
    representation::IndexType,
};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as _};

// TODO: allow Delaunay refinements!

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle list using the delaunay triangulation.
    pub fn delaunay_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        tri: &mut Triangulation<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));
        debug_assert!(!self.has_self_intersections(mesh));

        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
        //PERF: faster: ConstrainedDelaunayTriangulation::bulk_load()
        let mut last = None;
        let mut first = None;
        let mut i2v = Vec::new();
        for (i2, (vec2, global_index)) in self.vertices_2d::<V, P>(mesh).enumerate() {
            i2v.push(global_index);
            let i = cdt
                .insert(Point2::new(vec2.x().to_f64(), vec2.y().to_f64()))
                .unwrap();
            assert!(i.index() == i2);
            if let Some(j) = last {
                assert!(cdt.add_constraint(j, i));
            } else {
                first = Some(i);
            }
            last = Some(i);
        }
        assert!(cdt.add_constraint(last.unwrap(), first.unwrap()));

        cdt.inner_faces().for_each(|face| {
            let [p0, p1, p2] = face.vertices();
            let v0 = i2v[p0.index()];
            let v1 = i2v[p1.index()];
            let v2 = i2v[p2.index()];
            let r = self.triangle_touches_boundary(mesh, v0, v1, v2);
            if r.is_none() || r.unwrap() {
                tri.insert_triangle(v0, v1, v2);
            }
        });

        debug_assert!({
            let vec2s = self.vec2s(mesh);
            let vec_hm: HashMap<V, P::Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();
            tri.verify_indices(&vec_hm);
            tri.verify_all_indices_used(&vec2s);
            tri.verify_no_intersections(&vec_hm);
            tri.verify_non_degenerate_triangle(&vec_hm);
            true
        });
    }
}
