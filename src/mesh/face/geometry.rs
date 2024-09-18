use super::{super::Mesh, Face, FacePayload};
use crate::{
    math::{HasPosition, IndexType, LineSegment2D, Scalar, Transform, Vector, Vector3D, VectorIteratorExt},
    mesh::MeshType,
};
use itertools::Itertools;

impl<E: IndexType, F: IndexType, FP: FacePayload> Face<E, F, FP> {
    fn vertices_crossed<'a, T: MeshType<E = E, F = F, FP = FP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = T::Vec> + 'a + Clone + ExactSizeIterator
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices(mesh)
            .circular_tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| (*b.pos() - *a.pos()).cross(&(*c.pos() - *a.pos())))
    }

    /// Whether the face is convex. Ignores order.
    pub fn is_convex<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> bool
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: is this correct?
        // TODO: collinear points cause problems
        self.vertices_crossed(mesh)
            .circular_tuple_windows::<(_, _)>()
            .map(|w| w.0.dot(&w.1).is_positive())
            .all_equal()
    }

    /// Whether the face is planar.
    pub fn is_planar<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>, eps: T::S) -> bool
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: is this correct?
        // TODO: collinear points cause problems

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.pos()).collect();
        let n = (three[1] - three[0]).cross(&(three[2] - three[0]));

        self.vertices(mesh).skip(2).all(|v| {
            let v = *v.pos();
            (v - three[0]).dot(&n).abs() < eps
        })
    }

    /// Whether the face is planar.
    pub fn is_planar2<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> bool
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: eps?
        self.is_planar(mesh, T::S::EPS.sqrt())
    }

    /// Whether the face has holes.
    /// The data structure (currently!) cannot represent holes, so this is always false.
    pub fn has_holes(&self) -> bool {
        return false;
    }

    /// Whether the face is self-intersecting.
    /// This is a quite slow O(n^2) method. Use with caution.
    pub fn has_self_intersections<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> bool
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: Test this
        self.vertices_2d(mesh)
            .circular_tuple_windows::<(_, _)>()
            .tuple_combinations::<(_, _)>()
            .any(|(((a1, _), (a2, _)), ((b1, _), (b2, _)))| {
                let l1 = LineSegment2D::<T::Vec2>::new(a1, a2);
                let l2 = LineSegment2D::<T::Vec2>::new(b1, b2);
                let res = l1.intersect_line(&l2, T::S::EPS, -T::S::EPS);
                res.is_some()
            })
    }

    /// Whether the face is simple, i.e., doesn't self-intersect or have holes.
    /// Testing this is quite slow O(n^2). Use with caution.
    pub fn is_simple<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> bool
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        !self.has_holes() && !self.has_self_intersections(mesh)
    }

    /// A fast methods to get the surface normal, but will only work for convex faces.
    pub fn normal_naive<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> T::Vec
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        debug_assert!(self.is_planar2(mesh));
        debug_assert!(self.is_convex(mesh));

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| *v.pos()).collect();
        three[1].normal(three[0], three[2])
    }

    /// Get the normal of the face. Assumes the face is planar.
    /// Uses Newell's method to handle concave faces.
    /// PERF: Why not faster? Can't we find the normal using 3 vertices?
    pub fn normal<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> T::Vec
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: overload this in a way that allows different dimensions
        // TODO: allows only for slight curvature...
        debug_assert!(
            self.may_be_curved() || self.is_planar2(mesh),
            "Face is not planar {:?}",
            self
        );

        let normal = self
            .vertices(mesh)
            .map(|v| *v.pos())
            .circular_tuple_windows::<(_, _)>()
            .map(|(a, b)| {
                T::Vec::new(
                    (a.z() + b.z()) * (b.y() - a.y()),
                    (a.x() + b.x()) * (b.z() - a.z()),
                    (a.y() + b.y()) * (b.x() - a.x()),
                )
            })
            .stable_sum();

        assert!(
            normal.length_squared() >= T::S::EPS,
            "Degenerated face {} {:?}",
            self.id(),
            self.vertices(mesh).map(|v| *v.pos()).collect::<Vec<_>>()
        );

        normal * T::Vec::splat(T::S::from(-0.5))
    }

    // TODO: check for degenerated faces; empty triangles, collinear points etc...

    /*pub fn vertices_2d<'a, V: IndexType, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, usize, F, P>,
    ) -> impl Iterator<Item = T::Vec> + Clone + ExactSizeIterator + 'a
    where
        T::Vec: Vector2D<S = T::S>,
    {
        assert!(self.is_planar2(mesh));
        assert!(T::Vec::dimensions() == 2);
        self.vertices(mesh).map(|v| *v.vertex())
    }*/

    /// Get an iterator over the 2d vertices of the face rotated to the XY plane.
    pub fn vertices_2d<'a, T: MeshType<E = E, F = F, FP = FP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = (T::Vec2, T::V)> + Clone + ExactSizeIterator + 'a
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        // TODO: overload this in a way that allows different dimensions
        assert!(T::Vec::dimensions() == 3);

        let z_axis = T::Vec::new(0.0.into(), 0.0.into(), 1.0.into());
        let rotation = <T::Trans as Transform>::from_rotation_arc(
            self.normal(mesh).normalize(),
            z_axis.normalize(),
        );
        self.vertices(mesh)
            .map(move |v| (rotation.apply(*v.pos()).vec2(), v.id()))
    }

    /// Get a vector of 2d vertices of the face rotated to the XY plane.
    pub fn vec2s<'a, T: MeshType<E = E, F = F, FP = FP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> Vec<IndexedVertex2D<T::V, T::Vec2>>
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices_2d::<T>(mesh)
            .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
            .collect()
    }

    /// Naive method to get the center of the face by averaging the vertices.
    pub fn center<T: MeshType<E = E, F = F, FP = FP>>(&self, mesh: &Mesh<T>) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices(mesh).map(|v| *v.pos()).stable_mean()
    }
}