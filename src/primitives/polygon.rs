use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::{
        CurvedEdge, DefaultEdgePayload, DefaultFacePayload, EdgeCursorMut,
        EuclideanMeshType, HalfEdge, MeshBuilder, MeshTrait, MeshType, MeshTypeHalfEdge,
        PathBuilder,
    },
};

/// Calculate the side length of a regular polygon with `n` sides and a given circum`radius`.
pub fn regular_polygon_sidelength<S: Scalar>(radius: S, n: usize) -> S {
    S::TWO * radius * (S::PI / S::from_usize(n)).sin()
}

/// Calculate the area of a regular polygon with `n` sides and a given circum`radius`.
pub fn regular_polygon_area<S: Scalar>(radius: S, n: usize) -> S {
    S::HALF * S::from_usize(n) * radius * radius * (S::TWO * S::PI / S::from_usize(n)).sin()
}

/// Methods to insert 2D shapes into a mesh.
pub trait Make2dShape<T: MeshType<Mesh = Self>>: MeshTrait<T = T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Construct a polygon from the given vertices and
    /// Return the edge from the first to the second inserted vertex.
    /// Panics if the Iterator has less than 2 elements.
    fn insert_polygon<'a>(
        &'a mut self,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> EdgeCursorMut<'a, T>;

    /// Calls `insert_polygon` on a default mesh.
    fn polygon(vp: impl IntoIterator<Item = T::VP>) -> Self {
        let mut mesh = Self::default();
        mesh.insert_polygon(vp);
        mesh
    }

    /// Construct a dihedron (flat degenerate polygon with two faces) from the given vertices.
    fn insert_dihedron(&mut self, _vp: impl IntoIterator<Item = T::VP>) -> EdgeCursorMut<'_, T>;

    /// Calls `insert_dihedron` on a default mesh.
    fn dihedron(vp: impl IntoIterator<Item = T::VP>) -> Self {
        let mut mesh = Self::default();
        mesh.insert_dihedron(vp);
        mesh
    }

    /// create a regular star, i.e., a regular polygon with two radii
    fn insert_regular_star<'a, const D: usize>(
        &'a mut self,
        inner_radius: T::S,
        outer_radius: T::S,
        n: usize,
    ) -> EdgeCursorMut<'a, T>
    where
        T: EuclideanMeshType<D>,
    {
        assert!(
            n >= 2,
            "Cannot build a shape with less than 2 vertices edges."
        );
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        self.insert_polygon((0..n).into_iter().map(|i| {
            let r = if i % 2 == 1 {
                outer_radius
            } else {
                inner_radius
            };
            let angle = pi2n * (i as f32);
            T::VP::from_pos(T::Vec::from_xy(
                r * T::S::from(angle.sin()),
                r * T::S::from(angle.cos()),
            ))
        }))
    }

    /// Inserts a regular polygon with `n` sides and a given `radius`.
    fn insert_regular_polygon<'a, const D: usize>(
        &'a mut self,
        radius: T::S,
        n: usize,
    ) -> EdgeCursorMut<'a, T>
    where
        T: EuclideanMeshType<D>,
    {
        self.insert_regular_star(radius, radius, n)
    }

    /// create a regular polygon
    fn regular_polygon<const D: usize>(radius: T::S, n: usize) -> Self
    where
        T: EuclideanMeshType<D>,
    {
        Self::regular_star(radius, radius, n)
    }

    /// Create a circle from four cubic Bezier curves.
    fn cubic_circle<const D: usize>(radius: T::S) -> Self
    where
        T::Edge: CurvedEdge<D, T> + HalfEdge<T>,
        T: EuclideanMeshType<D> + MeshTypeHalfEdge,
        T::Mesh: MeshBuilder<T>,
    {
        let mut mesh = Self::default();
        let circle_len = radius * T::S::FOUR / T::S::THREE * (T::S::TWO.sqrt() - T::S::ONE);
        PathBuilder::<T, _>::start(&mut mesh, T::Vec::from_xy(radius, T::S::ZERO))
            .cubic(
                T::Vec::from_xy(radius, -circle_len),
                T::Vec::from_xy(circle_len, -radius),
                T::Vec::from_xy(T::S::ZERO, -radius),
            )
            .cubic(
                T::Vec::from_xy(-circle_len, -radius),
                T::Vec::from_xy(-radius, -circle_len),
                T::Vec::from_xy(-radius, T::S::ZERO),
            )
            .cubic(
                T::Vec::from_xy(-radius, circle_len),
                T::Vec::from_xy(-circle_len, radius),
                T::Vec::from_xy(T::S::ZERO, radius),
            )
            .cubic(
                T::Vec::from_xy(circle_len, radius),
                T::Vec::from_xy(radius, circle_len),
                T::Vec::from_xy(radius, T::S::ZERO),
            )
            .close(Default::default());
        mesh
    }

    /// Calls `insert_regular_star` on a new mesh.
    fn regular_star<const D: usize>(inner_radius: T::S, outer_radius: T::S, n: usize) -> Self
    where
        T: EuclideanMeshType<D>,
    {
        let mut mesh = Self::default();
        mesh.insert_regular_star(inner_radius, outer_radius, n);
        mesh
    }
}

#[cfg(test)]
mod tests {
    use crate::{extensions::nalgebra::*, prelude::*};

    #[test]
    fn test_regular_polygon() {
        for n in [2, 3, 4, 10] {
            let mut mesh = Mesh3d64::default();
            let e0 = mesh.insert_regular_polygon(1.0, n).id();
            assert_eq!(mesh.num_edges(), n);
            assert_eq!(mesh.num_faces(), 1);
            assert_eq!(mesh.num_vertices(), n);
            assert_eq!(mesh.check(), Ok(()));
            assert_eq!(mesh.is_open_2manifold(), true);
            assert_eq!(mesh.is_connected(), true);
            assert_eq!(mesh.edge(e0).has_face(), false);
            assert_eq!(mesh.edge(e0).twin().has_face(), true);
            let f = mesh.the_face();
            for i in 0..n {
                let ei = mesh.edge(e0).next_n(i);
                assert_eq!(ei.twin().face_id(), f.id());
            }
        }
    }
}
