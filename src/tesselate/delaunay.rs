use super::Triangulation;
use crate::{
    math::{IndexType, Scalar, Vector, Vector2D},
    mesh::{Face, Face3d, FaceBasics, MeshType3D},
};
use itertools::Itertools;
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as _};
use std::collections::HashMap;

/// Converts the face into a triangle list using the delaunay triangulation.
pub fn delaunay_triangulation<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    tri: &mut Triangulation<T::V>,
) {
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));
    debug_assert!(!face.has_self_intersections(mesh));

    let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::default();
    // PERF: faster: ConstrainedDelaunayTriangulation::bulk_load()
    // PERF: allow Delaunay refinements!
    let mut last = None;
    let mut first = None;
    let mut i2v: Vec<T::V> = Vec::new();
    let vec2s = face.vertices_2d(mesh).collect_vec();
    for (i2, (vec2, global_index)) in vec2s.iter().enumerate() {
        i2v.push(*global_index);
        let spade_vertex = cdt
            .insert(Point2::new(vec2.x().as_f64(), vec2.y().as_f64()))
            .unwrap();
        // TODO: Handle meshes with vertices with the same position
        assert!(spade_vertex.index() == i2);
        if let Some(j) = last {
            assert!(cdt.add_constraint(j, spade_vertex));
        } else {
            first = Some(spade_vertex);
        }
        last = Some(spade_vertex);
    }
    assert!(cdt.add_constraint(last.unwrap(), first.unwrap()));

    //let v2d = face.vertices_2d(mesh).collect::<Vec<_>>();
    //let poly = T::Poly::from_iter(v2d.iter().map(|(v, _)| v.clone()));

    cdt.inner_faces().for_each(|f| {
        let [p0, p1, p2] = f.vertices();
        let v0 = i2v[p0.index()];
        let v1 = i2v[p1.index()];
        let v2 = i2v[p2.index()];
        let r = face.triangle_touches_boundary(mesh, v0, v1, v2);
        if r.is_none() || r.unwrap() {
            if r.is_some() {
                tri.insert_triangle(v0, v1, v2);
                return;
            }

            /*
            // For triangles fully within or without the face, we need to check the centroid of the triangle
            // TODO: is there a better way? this is inefficient
            let mut triangle: Vec<T::Vec2> = Vec::new();
            for (v, i) in &v2d {
                if *i == v0 || *i == v1 || *i == v2 {
                    triangle.push(*v);
                }
            }
            let triangle = T::Poly::from_iter(triangle);
            if poly.contains(&triangle.centroid()) {
                tri.insert_triangle(v0, v1, v2);
            }*/

            // faster check: if the angle in direction of the face boundary is smaller than the boundary angle, it's inside

            fn wrap_angle<S: Scalar>(x: S) -> S {
                if x < S::zero() {
                    x + S::PI * S::TWO
                } else {
                    x
                }
            }

            fn is_edge_inside<V: IndexType, V2: Vector2D>(
                vec2s: &[(V2, V)],
                v: usize,
                other_v: usize,
            ) -> bool {
                let n = vec2s.len();
                let prev = vec2s[(v + n - 1) % n].0;
                let next = vec2s[(v + 1) % n].0;
                let triangle_angle = wrap_angle(vec2s[v].0.angle_tri(vec2s[other_v].0, prev));
                let boundary_angle = wrap_angle(vec2s[v].0.angle_tri(next, prev));
                triangle_angle <= boundary_angle
            }

            {
                let is_inside = is_edge_inside(&vec2s, p0.index(), p1.index());
                if is_inside {
                    tri.insert_triangle(v0, v1, v2);
                }

                // we expect the same result for all 6 edge orientations
                for (v, other_v) in &[(p0, p1), (p1, p2), (p2, p0)] {
                    debug_assert_eq!(
                        is_edge_inside(&vec2s, v.index(), other_v.index()),
                        is_inside
                    );
                    debug_assert_eq!(
                        is_edge_inside(&vec2s, other_v.index(), v.index()),
                        is_inside
                    );
                }
            }
        }
    });

    // TODO: make tests to perform these tests. This is too slow, even for debug builds!
    debug_assert!({
        let vec2s = face.vec2s(mesh);
        let vec_hm: HashMap<T::V, T::Vec2> = vec2s.iter().map(|v| (v.index, v.vec)).collect();
        tri.verify_indices(&vec_hm);
        tri.verify_all_indices_used(&vec2s);
        tri.verify_no_intersections(&vec_hm);
        tri.verify_non_degenerate_triangle(&vec_hm);
        true
    });
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use crate::extensions::nalgebra::*;
    use crate::prelude::*;

    fn verify_triangulation<T: MeshType3D>(mesh: &T::Mesh, f: T::F) {
        let face = mesh.face(f);
        let vec2s = face.vec2s(mesh);
        assert!(
            T::Poly::from_iter(vec2s.iter().map(|v| v.vec)).is_ccw(),
            "Polygon must be counterclockwise"
        );
        let mut indices = Vec::new();
        let mut tri = Triangulation::new(&mut indices);
        delaunay_triangulation::<T>(face, &mesh, &mut tri);
        tri.verify_full::<T::Vec2, T::Poly>(&vec2s);
    }

    #[test]
    #[cfg(feature = "fonts")]
    fn test_font() {
        let mut mesh2d = Mesh2d64Curved::new();
        Font::new(include_bytes!("../../assets/Cochineal-Roman.otf"), 1.0)
            .layout_text::<2, MeshType2d64PNUCurved>("F", &mut mesh2d);
        let mesh3d = mesh2d.to_nd(0.01);
        self::verify_triangulation::<MeshType3d64PNU>(&mesh3d, 0);
    }
}
