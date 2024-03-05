use super::super::{IndexType, Mesh};
use crate::{
    math::{Scalar, Vector3D},
    representation::payload::Payload,
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<P::S>,
{
    /// create a (conical) frustm
    pub fn frustum(r1: P::S, r2: P::S, h: P::S, n: usize) -> Mesh<E, V, F, P> {
        assert!(r1 >= P::S::ZERO && r2 >= P::S::ZERO && h >= P::S::ZERO && n >= 3);

        // TODO: implement
        //assert!(r1 > P::S::ZERO, "r1 must be positive for now");
        assert!(r2 > P::S::ZERO, "r2 must be positive for now");

        let mut mesh = Mesh::<E, V, F, P>::regular_polygon(r2, n);
        let h2 = h * P::S::from(0.5);
        mesh.translate(&P::Vec::from_xyz(P::S::ZERO, h2, P::S::ZERO));
        //mesh.extrude(E::new(1), P::Vec::from_xyz(P::S::ZERO, -h, P::S::ZERO), true);
        mesh.extrude_to_center_point(
            E::new(1),
            P::Vec::from_xyz(P::S::ZERO, -h, P::S::ZERO),
        );
        mesh
    }

    /// create a regular pyramid
    pub fn pyramid(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(radius, P::S::ZERO, height, n)
    }

    /// create a regular cone
    pub fn cone(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(P::S::ZERO, radius, height, n)
    }

    /// create a regular cylinder
    pub fn cylinder(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(radius, radius, height, n)
    }
}
