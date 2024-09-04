use super::{Face, Mesh, Payload};
use crate::{
    math::{Scalar, Vector, Vector3D},
    representation::IndexType,
};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as _};

impl<E, F> Face<E, F>
where
    E: IndexType,
    F: IndexType,
{
    /// Converts the face into a triangle list using the delaunay triangulation.
    pub fn delaunay_triangulation<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        indices: &mut Vec<V>,
    ) where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));
        debug_assert!(!self.has_self_intersections(mesh));

        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
        //TODO: faster: ConstrainedDelaunayTriangulation::bulk_load()
        let mut last = None;
        let mut first = None;
        for (i2, (vec2, _)) in self.vertices_2d::<V, P>(mesh).enumerate() {
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

        let i0 = indices.len();
        cdt.inner_faces().for_each(|face| {
            let [p0, p1, p2] = face.vertices();
            let v0 = V::new(p0.index());
            let v1 = V::new(p1.index());
            let v2 = V::new(p2.index());

            //if self.is_inside(mesh, i2v[p0.index()], i2v[p1.index()], i2v[p2.index()]) {
            if self.is_inside(
                mesh,
                V::new(i0 + p0.index()),
                V::new(i0 + p1.index()),
                V::new(i0 + p2.index()),
            ) {
                indices.extend(&[v0, v1, v2]);
            }
        });
    }
}
