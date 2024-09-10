use crate::{
    math::Vector,
    representation::{
        builder::{AddVertex, CloseFace},
        payload::VertexPayload,
        DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// create a regular polygon
    pub fn regular_polygon(radius: T::S, n: usize) -> Mesh<T> {
        Self::regular_star(radius, radius, n)
    }

    /// Draw a polygon from the given vertices
    pub fn polygon(v: &[T::Vec]) -> Mesh<T> {
        let mut mesh = Mesh::<T>::new();
        assert!(v.len() >= 3);
        let (v0, mut current) = mesh.add_isolated_edge(
            T::VP::from_vec(v[0]),
            Default::default(),
            T::VP::from_vec(v[1]),
            Default::default(),
        );
        let mut last = current;
        for i in 2..v.len() {
            last = current;
            current = mesh
                .add_vertex((
                    current,
                    T::VP::from_vec(v[i]),
                    Default::default(),
                    Default::default(),
                ))
                .0;
        }
        mesh.close_face((
            last,
            Default::default(),
            current,
            Default::default(),
            v0,
            Default::default(),
            false,
        ));
        mesh
    }

    /// create a regular star, i.e., a regular polygon with two radii
    pub fn regular_star(inner_radius: T::S, outer_radius: T::S, n: usize) -> Mesh<T> {
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        let a0 = 0.0 as f32;
        let a1 = pi2n;
        let mut mesh = Mesh::<T>::new();

        let (v0, mut current) = mesh.add_isolated_edge(
            T::VP::from_vec(T::Vec::from_xy(
                inner_radius * T::S::from(a0.sin()),
                inner_radius * T::S::from(a0.cos()),
            )),
            Default::default(),
            T::VP::from_vec(T::Vec::from_xy(
                outer_radius * T::S::from(a1.sin()),
                outer_radius * T::S::from(a1.cos()),
            )),
            Default::default(),
        );
        let mut prev = v0;

        for i in 2..n {
            let r = if i % 2 == 1 {
                outer_radius
            } else {
                inner_radius
            };
            let angle = pi2n * (i as f32);
            prev = current;
            current = mesh
                .add_vertex((
                    current,
                    T::VP::from_vec(T::Vec::from_xy(
                        r * T::S::from(angle.sin()),
                        r * T::S::from(angle.cos()),
                    )),
                    Default::default(),
                    Default::default(),
                ))
                .0;
        }

        mesh.close_face((
            prev,
            Default::default(),
            current,
            Default::default(),
            v0,
            Default::default(),
            false,
        ));

        mesh
    }
}
