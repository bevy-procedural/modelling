use crate::{
    math::{Scalar, Vector},
    representation::{
        payload::HasPosition, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

pub fn regular_polygon_sidelength<S: Scalar>(radius: S, n: usize) -> S {
    S::TWO * radius * (S::PI / S::from_usize(n)).sin()
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Construct a polygon from the given vertices and return the first edge on the outside boundary.
    pub fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        let first = self.insert_loop(vp);
        self.close_hole(first, Default::default(), false);
        self.edge(first).twin_id()
    }

    /// Calls `insert_polygon` on a new mesh.
    pub fn polygon(vp: impl IntoIterator<Item = T::VP>) -> Mesh<T> {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_polygon(vp);
        mesh
    }

    /// Construct a dihedron (flat degenerate polygon with two faces) from the given vertices.
    pub fn insert_dihedron(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let first = self.insert_polygon(vp);
        self.close_hole(self.edge(first).twin_id(), Default::default(), false);
        first
    }

    /// Calls `insert_dihedron` on a new mesh.
    pub fn dihedron(vp: impl IntoIterator<Item = T::VP>) -> Mesh<T> {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_dihedron(vp);
        mesh
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// create a regular polygon
    pub fn regular_polygon(radius: T::S, n: usize) -> Mesh<T> {
        Self::regular_star(radius, radius, n)
    }

    /// create a regular star, i.e., a regular polygon with two radii
    pub fn insert_regular_star(
        &mut self,
        inner_radius: T::S,
        outer_radius: T::S,
        n: usize,
    ) -> T::E {
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

    /// Calls `insert_regular_star` on a new mesh.
    pub fn regular_star(inner_radius: T::S, outer_radius: T::S, n: usize) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_regular_star(inner_radius, outer_radius, n);
        mesh
    }
}
