use crate::{
    math::{HasPosition, Vector2D, Vector3D},
    mesh::{Face3d, FaceBasics, MeshType, Vertex},
};
use itertools::Itertools;

use super::Triangulation;

/// Converts the face into a triangle fan. Only works for convex planar faces.
pub fn fan_triangulation<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));
    debug_assert!(face.is_convex(mesh));

    let center = face.vertices(mesh).next().unwrap();
    face.vertices(mesh)
        .skip(1)
        .tuple_windows::<(_, _)>()
        .for_each(|(a, b)| indices.insert_triangle(center.id(), a.id(), b.id()));
}

/// Quickly triangulates a (not necessarily convex) quadrilateral.
#[inline(always)]
pub fn quad_triangulate<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    let vs: Vec<(T::Vec2, T::V)> = face.vertices_2d(mesh).collect();
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
