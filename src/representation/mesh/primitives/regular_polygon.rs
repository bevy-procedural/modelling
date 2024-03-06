use super::super::{IndexType, Mesh};
use crate::{
    math::Vector3D,
    representation::{
        builder::{AddVertex, CloseFace},
        payload::Payload,
    },
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<P::S>,
{
    /// create a regular polygon
    pub fn regular_polygon(radius: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::regular_star(radius, radius, n)
    }

    /// Draw a polygon from the given vertices
    pub fn polygon(v: &[P::Vec]) -> Mesh<E, V, F, P> {
        let mut mesh = Mesh::<E, V, F, P>::new();
        assert!(v.len() >= 3);
        let (v0, mut current) = mesh.add_isolated_edge(P::from_vec(v[0]), P::from_vec(v[1]));
        let mut last = current;
        for i in 2..v.len() {
            last = current;
            current = mesh.add_vertex((current, P::from_vec(v[i]))).0;
        }
        mesh.close_face((last, current, v0, false));
        mesh
    }

    /// create a regular star, i.e., a regular polygon with two radii
    pub fn regular_star(inner_radius: P::S, outer_radius: P::S, n: usize) -> Mesh<E, V, F, P> {
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        let a0 = 0.0 as f32;
        let a1 = pi2n;
        let mut mesh = Mesh::<E, V, F, P>::new();

        let (v0, mut current) = mesh.add_isolated_edge(
            P::from_vec(P::Vec::from_xyz(
                inner_radius * P::S::from(a0.sin()),
                P::S::default(),
                inner_radius * P::S::from(a0.cos()),
            )),
            P::from_vec(P::Vec::from_xyz(
                outer_radius * P::S::from(a1.sin()),
                P::S::default(),
                outer_radius * P::S::from(a1.cos()),
            )),
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
                    P::from_vec(P::Vec::from_xyz(
                        r * P::S::from(angle.sin()),
                        P::S::default(),
                        r * P::S::from(angle.cos()),
                    )),
                ))
                .0;
        }

        mesh.close_face((prev, current, v0, false));

        mesh
    }
}
