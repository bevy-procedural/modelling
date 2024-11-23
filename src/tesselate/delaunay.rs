use super::Triangulation;
use crate::{
    math::{HasPosition, Polygon, Scalar, Vector, Vector3D},
    mesh::{Face, Face3d, FaceBasics, MeshType},
};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as _};
use std::collections::HashMap;

/// Converts the face into a triangle list using the delaunay triangulation.
pub fn delaunay_triangulation<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    tri: &mut Triangulation<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));
    debug_assert!(!face.has_self_intersections(mesh));

    let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::default();
    //PERF: faster: ConstrainedDelaunayTriangulation::bulk_load()
    // PERF: allow Delaunay refinements!
    let mut last = None;
    let mut first = None;
    let mut i2v = Vec::new();
    for (i2, (vec2, global_index)) in face.vertices_2d(mesh).enumerate() {
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

            // TODO: is there a better way? this is inefficient

            let v2d = face.vertices_2d(mesh).collect::<Vec<_>>();
            let mut triangle: Vec<T::Vec2> = Vec::new();
            for (v, i) in &v2d {
                if *i == v0 || *i == v1 || *i == v2 {
                    triangle.push(*v);
                }
            }
            let triangle = T::Poly::from_iter(triangle);
            let poly = T::Poly::from_iter(v2d.iter().map(|(v, _)| v.clone()));

            if poly.contains(&triangle.centroid()) {
                tri.insert_triangle(v0, v1, v2);
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
mod tests {
    use crate::prelude::*;

    fn verify_triangulation<T: MeshType>(mesh: &T::Mesh, f: T::F)
    where
        T::Face: Face3d<T>,
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
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
    #[cfg(feature = "bevy")]
    fn test_font() {
        let mut mesh2d = BevyMesh2d::new();
        Font::new(include_bytes!("../../assets/Cochineal-Roman.otf"), 0.004)
            .layout_text::<BevyMeshType2d32>("F", &mut mesh2d);
        let mesh3d = mesh2d.to_3d(0.01);
        self::verify_triangulation::<BevyMeshType3d32>(&mesh3d, 0);
    }
}
