use super::basics::FaceBasics;
use crate::{
    math::{LineSegment2D, Polygon, Scalar, TransformTrait, Vector, Vector3D, Vector3DIteratorExt},
    mesh::{IndexedVertex2D, MeshType3D, VertexBasics},
};
use itertools::Itertools;

// TODO: Many Face3d functions should be part of n dimensions, not just 3d.

/// A face with vertices that have 3d positions.
pub trait Face3d<T: MeshType3D<Face = Self>>: FaceBasics<T> {
    /// Get an iterator over the cross products of the vertices of the face.
    fn vertices_crossed<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::Vec> + 'a + Clone + ExactSizeIterator
    where
        T::Vertex: 'a,
    {
        self.vertices(mesh)
            .circular_tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| (b.pos() - a.pos()).cross(&(c.pos() - a.pos())))
    }

    /// Whether the face is convex. Ignores order.
    fn is_convex(&self, mesh: &T::Mesh) -> bool {
        // TODO: is this correct?
        // TODO: collinear points cause problems
        self.vertices_crossed(mesh)
            .circular_tuple_windows::<(_, _)>()
            .map(|w| w.0.dot(&w.1).is_positive())
            .all_equal()
    }

    /// Whether the face is planar.
    fn is_planar(&self, mesh: &T::Mesh, eps: T::S) -> bool {
        // TODO: is this correct?
        // TODO: collinear points cause problems

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| v.pos()).collect();
        let n = (three[1] - three[0]).cross(&(three[2] - three[0]));

        self.vertices(mesh).skip(2).all(|v| {
            let v = v.pos();
            (v - three[0]).dot(&n).abs() < eps
        })
    }

    /// Whether the face is planar.
    fn is_planar2(&self, mesh: &T::Mesh) -> bool {
        // TODO: eps?
        self.is_planar(mesh, T::S::EPS.sqrt())
    }

    /// Whether the face is self-intersecting.
    /// This is a quite slow O(n^2) method. Use with caution.
    fn has_self_intersections(&self, mesh: &T::Mesh) -> bool {
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
    fn is_simple(&self, mesh: &T::Mesh) -> bool {
        !self.has_holes() && !self.has_self_intersections(mesh)
    }

    /// A fast methods to get the surface normal, but will only work for convex faces.
    fn normal_naive(&self, mesh: &T::Mesh) -> T::Vec {
        debug_assert!(self.is_planar2(mesh));
        debug_assert!(self.is_convex(mesh));

        let three: Vec<_> = self.vertices(mesh).take(3).map(|v| v.pos()).collect();
        three[1].normal(three[0], three[2])
    }

    /// Get the normal of the face. Assumes the face is planar.
    /// Uses Newell's method to handle concave faces.
    /// PERF: Why not faster? Can't we find the normal using 3 vertices?
    fn normal(&self, mesh: &T::Mesh) -> T::Vec {
        // TODO: overload this in a way that allows different dimensions
        // TODO: allows only for slight curvature...
        debug_assert!(
            self.may_be_curved() || self.is_planar2(mesh),
            "Face is not planar {:?}",
            self
        );

        let normal = self.vertices(mesh).map(|v| v.pos()).normal();

        /* assert!(
            normal.length_squared() >= T::S::EPS,
            "Degenerated face {} {:?}",
            self.id(),
            self.vertices(mesh).map(|v| v.pos()).collect::<Vec<_>>()
        );*/

        normal
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
    fn vertices_2d<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = (T::Vec2, T::V)> + Clone + ExactSizeIterator + 'a {
        let z_axis = T::Vec::new(0.0.into(), 0.0.into(), 1.0.into());
        let rotation = <T::Trans as TransformTrait<T::S, 3>>::from_rotation_arc(
            Face3d::normal(self, mesh).normalize(),
            z_axis.normalize(),
        );
        self.vertices(mesh)
            .map(move |v| (rotation.apply(v.pos()).vec2(), v.id()))
    }

    /// Get a vector of 2d vertices of the face rotated to the XY plane.
    fn vec2s<'a>(&'a self, mesh: &'a T::Mesh) -> Vec<IndexedVertex2D<T::V, T::Vec2>> {
        self.vertices_2d(mesh)
            .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
            .collect()
    }

    /// Returns the polygon of the face rotated to the XY plane.
    fn as_polygon(&self, mesh: &T::Mesh) -> T::Poly {
        <T::Poly as Polygon<T::Vec2>>::from_iter(self.vec2s(mesh).iter().map(|v| v.vec))
    }
}
