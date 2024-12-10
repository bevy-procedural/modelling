//! Functions for quickly triangulating polygons with a fixed number of vertices.
//! The number of triangulations for n vertices is given by the (n-2)nd Catalan number.

use crate::{
    math::Vector2D,
    mesh::{Face3d, MeshType3D, Triangulation},
};

/// Quickly min-weight triangulates a (not necessarily convex) quadrilateral.
#[inline(always)]
pub fn min_weight_quad<T: MeshType3D>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) {
    let vs: Vec<(T::Vec2, T::V)> = face.vertices_2d(mesh).collect();
    // TODO: is it faster to test convex or to test intersection?
    let vs1_convex = vs[1].0.convex(vs[0].0, vs[2].0);
    let vs3_convex = !vs1_convex || vs[3].0.convex(vs[2].0, vs[0].0);
    if vs1_convex && vs3_convex {
        indices.insert_triangle(vs[0].1, vs[1].1, vs[2].1);
        indices.insert_triangle(vs[0].1, vs[2].1, vs[3].1);
    } else {
        // Apparently, either vs[1] or vs[3] is a reflex vertex.
        // Hence, we split the quadrilateral the other way.
        indices.insert_triangle(vs[1].1, vs[2].1, vs[3].1);
        indices.insert_triangle(vs[1].1, vs[3].1, vs[0].1);
    }
}

/// Quickly min-weight triangulates a (not necessarily convex) pentagon.
#[inline(always)]
pub fn min_weight_pent<T: MeshType3D>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
) {
    // TODO: make specialized implementations for n=5
    // both inner edges start at the same vertex, so you only have to test 5 configs and find:
    // 1) the shortest sum of edge lengths or largest angle
    // 2) ...that doesn't intersect any boundary edge (because the intersecting case can be the optimum)

    todo!("pent triangulate");
}

/// Quickly  min-weight triangulates a (not necessarily convex) hexagon.
pub fn min_weight_hex<T: MeshType3D>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
) {
    // TODO: make specialized implementations for n=6
    // Iterate the 14 configurations: 6 for fans, 3 for Zs, 3 for mirrored Zs, and 2 Triangles.
    // Every time a new favorite is found, check if it intersects with one of the two (!) non-adjacent boundary edges.
    // Once a chord has been excluded, it cannot ever be used again and we can quickly skip configurations using it.
    // Test, whether this is faster than sweep or delaunay.

    todo!("hex triangulate");
}

/// Simple dynamic programming approach to calculate the min-weight triangulation of a polygon with a fixed number of vertices.
/// Suitable for very small `n`.
pub fn minweight_dynamic_fixed<T: MeshType3D, const N: usize>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
) {
    // TODO: for the next few larger n (as long, if at all, it is faster than sweep), we could attempt a simplified dynamic programming algo on the sub-polygons of the mesh.

    todo!("hex triangulate");
}
