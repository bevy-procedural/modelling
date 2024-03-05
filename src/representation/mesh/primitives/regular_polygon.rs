use super::super::{IndexType, Mesh};
use crate::representation::{
    builder::{AddVertex, CloseFace},
    payload::{Payload, Vector3D},
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
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        let a0 = 0.0 as f32;
        let a1 = pi2n;
        let mut mesh = Mesh::<E, V, F, P>::new();

        let (v0, mut current) = mesh.add_isolated_edge(
            P::from_vec(P::Vec::from_xyz(
                radius * P::S::from(a0.sin()),
                P::S::default(),
                radius * P::S::from(a0.cos()),
            )),
            P::from_vec(P::Vec::from_xyz(
                radius * P::S::from(a1.sin()),
                P::S::default(),
                radius * P::S::from(a1.cos()),
            )),
        );
        let mut prev = v0;

        for i in 2..n {
            let r = if i % 2 == 0 { radius } else { radius * 0.8.into() };
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

        mesh.close_face((prev, current, v0));

        mesh
    }
}
