use super::{Face, Mesh, Payload, Triangulation};
use crate::{
    math::{Vector2D, Vector3D},
    representation::IndexType,
};
use itertools::Itertools;

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle fan. Only works for convex planar faces.
    pub fn fan_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Triangulation<V>,
    ) {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));
        debug_assert!(self.is_convex(mesh));

        let center = self.vertices(mesh).next().unwrap();
        self.vertices(mesh)
            .skip(1)
            .tuple_windows::<(_, _)>()
            .for_each(|(a, b)| indices.insert_triangle(center.id(), a.id(), b.id()));
    }

    /// Quickly triangulates a (not necessarily convex) quadrilateral.
    #[inline(always)]
    pub fn quad_triangulate<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Triangulation<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        let vs: Vec<(P::Vec2, V)> = self.vertices_2d::<V, P>(mesh).collect();
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
}
