use super::{
    super::{payload::Payload, IndexType, Mesh},
    tesselate::IndexedVertex2D,
    Face,
};
use crate::math::{LineSegment2D, Scalar, Transform, Vector, Vector3D};
use itertools::Itertools;

impl<E: IndexType, F: IndexType> Face<E, F> {
    fn vertices_crossed<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = P::Vec> + 'a + Clone + ExactSizeIterator {
        self.vertices(mesh)
            .circular_tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| (*b.vertex() - *a.vertex()).cross(&(*c.vertex() - *a.vertex())))
    }

    /// Whether the face is convex. Ignores order.
    pub fn is_convex<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        // TODO: is this correct?
        // TODO: collinear points cause problems
        self.vertices_crossed(mesh)
            .circular_tuple_windows::<(_, _)>()
            .map(|w| w.0.dot(&w.1).is_positive())
            .all_equal()
    }

    /// Whether the face is planar.
    pub fn is_planar<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>, eps: P::S) -> bool {
        if P::Vec::dimensions() <= 2 {
            return true;
        }

        // TODO: is this correct?
        // TODO: collinear points cause problems

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.vertex()).collect();
        let n = (three[1] - three[0]).cross(&(three[2] - three[0]));

        self.vertices(mesh).skip(2).all(|v| {
            let v = *v.vertex();
            (v - three[0]).dot(&n).abs() < eps
        })
    }

    /// Whether the face is planar.
    pub fn is_planar2<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool {
        self.is_planar(mesh, P::S::EPS * 10.0.into())
    }

    /// Whether the face has holes.
    /// The data structure cannot represent holes, so this is always false.
    pub fn has_holes(&self) -> bool {
        return false;
    }

    /// Whether the face is self-intersecting.
    /// This is a quite slow O(n^2) method. Use with caution.
    pub fn has_self_intersections<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool
    where
        P::Vec: Vector3D<S = P::S>,
    {
        // TODO: Test this
        self.vertices_2d(mesh)
            .circular_tuple_windows::<(_, _)>()
            .tuple_combinations::<(_, _)>()
            .any(|(((a1, _), (a2, _)), ((b1, _), (b2, _)))| {
                let l1 = LineSegment2D::<P::Vec2>::new(a1, a2);
                let l2 = LineSegment2D::<P::Vec2>::new(b1, b2);
                let res = l1.intersect_line(&l2, P::S::EPS, -P::S::EPS);
                res.is_some()
            })
    }

    /// Whether the face is simple, i.e., doesn't self-intersect or have holes.
    /// Testing this is quite slow O(n^2). Use with caution.
    pub fn is_simple<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> bool
    where
        P::Vec: Vector3D<S = P::S>,
    {
        !self.has_holes() && !self.has_self_intersections(mesh)
    }

    /// A fast methods to get the surface normal, but will only work for convex faces.
    pub fn normal_naive<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec
    where
        P::Vec: Vector3D<S = P::S>,
    {
        debug_assert!(self.is_planar2(mesh));
        debug_assert!(self.is_convex(mesh));

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.vertex()).collect();
        three[1].normal(three[0], three[2])
    }

    /// Get the normal of the face. Assumes the face is planar.
    /// Uses Newell's method to handle concave faces.
    /// PERF: Why not faster? Can't we find the normal using 3 vertices?
    pub fn normal<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec
    where
        P::Vec: Vector3D<S = P::S>,
    {
        // TODO: overload this in a way that allows different dimensions
        // TODO: allows only for slight curvature...
        debug_assert!(self.may_be_curved() || self.is_planar2(mesh));

        let normal = P::Vec::sum(
            self.vertices(mesh)
                .map(|v| *v.vertex())
                .circular_tuple_windows::<(_, _)>()
                .map(|(a, b)| {
                    P::Vec::from_xyz(
                        (a.z() + b.z()) * (b.y() - a.y()),
                        (a.x() + b.x()) * (b.z() - a.z()),
                        (a.y() + b.y()) * (b.x() - a.x()),
                    )
                }),
        );
        normal * P::Vec::splat(P::S::from(-0.5))
    }

    // TODO: check for degenerated faces; empty triangles, collinear points etc...

    /*pub fn vertices_2d<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, usize, F, P>,
    ) -> impl Iterator<Item = P::Vec> + Clone + ExactSizeIterator + 'a
    where
        P::Vec: Vector2D<S = P::S>,
    {
        assert!(self.is_planar2(mesh));
        assert!(P::Vec::dimensions() == 2);
        self.vertices(mesh).map(|v| *v.vertex())
    }*/

    /// Get an iterator over the 2d vertices of the face rotated to the XY plane.
    pub fn vertices_2d<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = (P::Vec2, V)> + Clone + ExactSizeIterator + 'a
    where
        P::Vec: Vector3D<S = P::S>,
    {
        // TODO: overload this in a way that allows different dimensions
        assert!(P::Vec::dimensions() == 3);

        let z_axis = P::Vec::from_xyz(0.0.into(), 0.0.into(), 1.0.into());
        let rotation =
            P::Trans::from_rotation_arc(self.normal(mesh).normalize(), z_axis.normalize());
        self.vertices(mesh)
            .map(move |v| (rotation.apply(*v.vertex()).xy(), v.id()))
    }

    /// Get a vector of 2d vertices of the face rotated to the XY plane.
    pub fn vec2s<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> Vec<IndexedVertex2D<V, <P as Payload>::Vec2>>
    where
        P::Vec: Vector3D<S = P::S>,
    {
        self.vertices_2d::<V, P>(mesh)
            .map(|(p, i)| IndexedVertex2D::<V, P::Vec2>::new(p, i))
            .collect()
    }

    /// Naive method to get the center of the face by averaging the vertices.
    pub fn center<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> P::Vec {
        P::Vec::mean(self.vertices(mesh).map(|v| *v.vertex()))
    }
}
