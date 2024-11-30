use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::{DefaultEdgePayload, DefaultFacePayload, EuclideanMeshType, MeshTrait, MeshType},
};

/// Calculate the side length of a regular polygon with `n` sides and a given `radius`.
pub fn regular_polygon_sidelength<S: Scalar>(radius: S, n: usize) -> S {
    S::TWO * radius * (S::PI / S::from_usize(n)).sin()
}

/// Methods to insert 2D shapes into a mesh.
pub trait Make2dShape<T: MeshType<Mesh = Self>>: MeshTrait<T = T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Construct a polygon from the given vertices and return the first edge on the outside boundary.
    fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E;

    /// Calls `insert_polygon` on a default mesh.
    fn polygon(vp: impl IntoIterator<Item = T::VP>) -> Self {
        let mut mesh = Self::default();
        mesh.insert_polygon(vp);
        mesh
    }

    /// Construct a dihedron (flat degenerate polygon with two faces) from the given vertices.
    fn insert_dihedron(&mut self, _vp: impl IntoIterator<Item = T::VP>) -> T::E;

    /// Calls `insert_dihedron` on a default mesh.
    fn dihedron(vp: impl IntoIterator<Item = T::VP>) -> Self {
        let mut mesh = Self::default();
        mesh.insert_dihedron(vp);
        mesh
    }

    /// create a regular star, i.e., a regular polygon with two radii
    fn insert_regular_star<const D: usize>(
        &mut self,
        inner_radius: T::S,
        outer_radius: T::S,
        n: usize,
    ) -> T::E
    where
        T: EuclideanMeshType<D>,
    {
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

    /// create a regular polygon
    fn regular_polygon<const D: usize>(radius: T::S, n: usize) -> Self
    where
        T: EuclideanMeshType<D>,
    {
        Self::regular_star(radius, radius, n)
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
