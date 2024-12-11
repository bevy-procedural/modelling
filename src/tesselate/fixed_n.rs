//! Functions for quickly triangulating polygons with a fixed number of vertices.
//! The number of triangulations for n vertices is given by the (n-2)nd Catalan number.

use crate::{
    math::{IndexType, Polygon, Vector2D},
    mesh::{Face3d, FaceBasics, IndexedVertex2D, MeshType3D, Triangulation, VertexBasics},
};
use itertools::Itertools;

/// Quickly min-weight triangulates a face. Returns true if the face was small enough to be efficiently triangulated.
pub fn try_min_weight_small<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) -> bool {
    let n = face.num_vertices(mesh);
    if n == 3 {
        let (a, b, c) = face.vertices(mesh).map(|v| v.id()).collect_tuple().unwrap();
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
#[inline(always)]
pub fn min_weight_quad<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    indices: &mut Triangulation<V>,
) {
    assert!(vs.len() == 4);
    let vs1_convex = vs[1].vec.convex(vs[0].vec, vs[2].vec);
    let vs3_convex = !vs1_convex || vs[3].vec.convex(vs[2].vec, vs[0].vec);
    if vs1_convex && vs3_convex {
        indices.insert_triangle(vs[0].index, vs[1].index, vs[2].index);
        indices.insert_triangle(vs[0].index, vs[2].index, vs[3].index);
    } else {
        // Apparently, either vs[1] or vs[3] is a reflex vertex.
        // Hence, we split the quadrilateral the other way.
        indices.insert_triangle(vs[1].index, vs[2].index, vs[3].index);
        indices.insert_triangle(vs[1].index, vs[3].index, vs[0].index);
    }
}

/// Quickly min-weight triangulates a (not necessarily convex) pentagon.
#[inline(always)]
pub fn min_weight_pent<V: IndexType, Vec2: Vector2D, Poly: Polygon<Vec2>>(
    vs: &Vec<IndexedVertex2D<V, Vec2>>,
    _indices: &mut Triangulation<V>,
) {
    assert!(vs.len() == 5);
    // TODO: make specialized implementations for n=5
    // both inner edges start at the same vertex, so you only have to test 5 configs and find:
    // 1) the shortest sum of edge lengths or largest angle
    // 2) ...that doesn't intersect any boundary edge (because the intersecting case can be the optimum)

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
