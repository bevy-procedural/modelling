use crate::{
    math::{HasPosition, Vector3D},
    mesh::{Face3d, FaceBasics, MeshType, Triangulation, Vertex, VertexBasics},
};
use itertools::Itertools;

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
