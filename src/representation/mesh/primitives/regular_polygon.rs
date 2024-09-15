use crate::{
    math::Vector,
    representation::{
        payload::HasPosition, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    pub fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        let first = self.insert_loop(vp);
        self.close_hole(first, Default::default(), false);
        self.edge(first).twin_id()
    }

    /// Construct a polygon from the given vertices.
    pub fn polygon(vp: impl IntoIterator<Item = T::VP>) -> Mesh<T> {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_polygon(vp);
        mesh
    }

    pub fn insert_dihedron(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let first = self.insert_polygon(vp);
        self.close_hole(self.edge(first).twin_id(), Default::default(), false);
        first
    }

    /// Construct a dihedron (flat degenerate polygon with two faces) from the given vertices.
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
    pub fn regular_star(inner_radius: T::S, outer_radius: T::S, n: usize) -> Mesh<T> {
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        Mesh::polygon((0..n).into_iter().map(|i| {
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
}
