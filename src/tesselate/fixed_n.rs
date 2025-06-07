//! Functions for quickly triangulating polygons with a fixed number of vertices.
//! The number of triangulations for n vertices is given by the (n-2)nd Catalan number.

use crate::{
    math::{IndexType, Polygon, Vector2D},
    mesh::{Face3d, FaceBasics, IndexedVertex2D, MeshType3D, Triangulation},
};
use itertools::Itertools;

/// Quickly min-weight triangulates a face. Returns true if the face was small enough to be efficiently triangulated.
pub fn try_min_weight_small<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) -> bool {
    if face.has_islands() {
        return false; // TODO: handle islands
    }

    let n = face.num_vertices(mesh);
    if n == 3 {
        let (a, b, c) = face.vertex_ids(mesh).collect_tuple().unwrap();
        indices.insert_triangle(a, b, c);
        return true;
    }

    // Skip early if the face is too small or too large
    if n <= 3 || n > 6 {
        return false;
    }

    let vs: Vec<IndexedVertex2D<T::V, T::Vec2>> = face
        .vertices_2d(mesh)
        .map(|(vec, v)| IndexedVertex2D::new(vec, v))
        .collect_vec();
    try_min_weight_small_direct::<T::V, T::Vec2, T::Poly>(&vs, indices)
}

/// Quickly min-weight triangulates a face. Returns true if the face was small enough to be efficiently triangulated.
pub fn try_min_weight_small_direct<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) -> bool {
    let n = vs.len();
    if n == 3 {
        let (a, b, c) = vs.iter().map(|v| v.index).collect_tuple().unwrap();
        indices.insert_triangle(a, b, c);
        return true;
    }
    match n {
        4 => {
            min_weight_quad::<V, Vec2, Poly>(&vs, indices);
            true
        }
        //5 => min_weight_pent::<V, Vec2, Poly>(vs, indices),
        //6 => min_weight_hex::<V, Vec2, Poly>(vs, indices),
        _ => false,
    }
}

/// Quickly min-weight triangulates a (not necessarily convex) quadrilateral.
#[inline]
pub fn min_weight_quad<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    assert!(vs.len() == 4);

    let weight_diagonal_0_2 = vs[0].vec.distance_squared(&vs[2].vec);
    let weight_diagonal_1_3 = vs[1].vec.distance_squared(&vs[3].vec);

    // TODO: Are the convexity checks correct? Or do they check the exact opposite?

    if weight_diagonal_0_2 <= weight_diagonal_1_3 {
        let vs1_3_convex =
            vs[1].vec.convex(vs[2].vec, vs[0].vec) && vs[3].vec.convex(vs[0].vec, vs[2].vec);
        if vs1_3_convex {
            // insert the diagonal 0-2
            indices.insert_triangle(vs[0].index, vs[1].index, vs[2].index);
            indices.insert_triangle(vs[0].index, vs[2].index, vs[3].index);
        } else {
            // insert the diagonal 1-3
            indices.insert_triangle(vs[1].index, vs[2].index, vs[3].index);
            indices.insert_triangle(vs[1].index, vs[3].index, vs[0].index);
        }
    } else {
        let vs_0_2_convex =
            vs[0].vec.convex(vs[3].vec, vs[1].vec) && vs[2].vec.convex(vs[1].vec, vs[3].vec);

        if vs_0_2_convex {
            // insert the diagonal 1-3
            indices.insert_triangle(vs[1].index, vs[2].index, vs[3].index);
            indices.insert_triangle(vs[1].index, vs[3].index, vs[0].index);
        } else {
            // insert the diagonal 0-2
            indices.insert_triangle(vs[0].index, vs[1].index, vs[2].index);
            indices.insert_triangle(vs[0].index, vs[2].index, vs[3].index);
        }
    }
}

/// Quickly min-weight triangulates a (not necessarily convex) pentagon.
#[inline]
pub fn min_weight_pent<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    _indices: &mut Triangulation<V>,
) {
    assert!(vs.len() == 5);
    // TODO: make specialized implementations for n=5
    // both diagonals start at the same vertex, so you only have to test 5 configs and find:
    // 1) the shortest sum of edge lengths
    // 2) ...that doesn't intersect any boundary edge (because the intersecting case can be the optimum)

    //let poly = Poly::from_iter(vs.iter().map(|v| v.vec));
    //poly.

    todo!("pent triangulate");
}

/// Quickly  min-weight triangulates a (not necessarily convex) hexagon.
pub fn min_weight_hex<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    _indices: &mut Triangulation<V>,
) {
    assert!(vs.len() == 6);
    // TODO: make specialized implementations for n=6
    // Iterate the 14 configurations: 6 for fans, 3 for Zs, 3 for mirrored Zs, and 2 Triangles.
    // Every time a new favorite is found, check if it intersects with one of the two (!) non-adjacent boundary edges.
    // Once a chord has been excluded, it cannot ever be used again and we can quickly skip configurations using it.
    // Test, whether this is faster than sweep or delaunay.

    todo!("hex triangulate");
}

#[cfg(test)]
#[cfg(feature = "nalgebra")]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        extensions::nalgebra::*, math::IndexType, mesh::EuclideanMeshType,
        tesselate::minweight_dynamic_direct,
    };

    fn verify_min_weight<T: EuclideanMeshType<2>>(vs: &Vec<T::Vec2>) {
        let vs: Vec<IndexedVertex2D<T::V, T::Vec2>> = vs
            .iter()
            .enumerate()
            .map(|(i, v)| IndexedVertex2D::new(*v, T::V::new(i)))
            .collect();
        let hm: HashMap<T::V, T::Vec2> = vs.iter().map(|v| (v.index, v.vec)).collect();

        let w1 = {
            let mut indices = Vec::new();
            let mut tri = Triangulation::<T::V>::new(&mut indices);
            try_min_weight_small_direct::<T::V, T::Vec2, T::Poly>(&vs, &mut tri);
            println!("Triangulation: {:?}", tri);
            tri.verify_full::<T::Vec2, T::Poly>(&vs);
            tri.total_edge_weight(&hm)
        };

        let w2 = {
            let mut indices = Vec::new();
            let mut tri = Triangulation::<T::V>::new(&mut indices);
            minweight_dynamic_direct::<T::V, T::Vec2, T::Poly>(&vs, &mut tri);
            tri.verify_full::<T::Vec2, T::Poly>(&vs);
            tri.total_edge_weight(&hm)
        };

        // w1 should be the same as w2.
        // Though, it can include degenerate triangles, so it might be even smaller!
        assert!((w1 - w2) <= T::S::from(1e-6));
    }

    #[test]
    fn test_min_weight_quad_1() {
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);
    }

    #[test]
    fn test_min_weight_quad_2() {
        // this one is very concave
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.05, 0.9),
        ]);
    }

    #[test]
    fn test_min_weight_quad_3() {
        // For this one the invalid diagonals are shorter
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.5, 0.5),
        ]);
    }

    #[test]
    fn test_min_weight_quad_4() {
        // Similar to 3 but rotated to test the other branch
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.5, 0.5),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(0.0, 0.0),
        ]);
    }

    #[test]
    fn test_min_weight_quad_5() {
        // Similar to 4, but now it is fine to use the shorter diagonal
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.5, -0.5),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(0.0, 0.0),
        ]);
    }

    /*
    #[test]
    fn test_min_weight_quad_6() {
        // Nasty case where a degenerate triangle is part of the best triangulation
        verify_min_weight::<MeshType2d64PNU>(&vec![
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(10.0, 0.0),
        ]);
    }*/
}
