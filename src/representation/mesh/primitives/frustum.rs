use super::super::{IndexType, Mesh};
use crate::{
    math::{HasZero, Scalar, Transform, Vector3D},
    representation::payload::Payload,
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
    P::Vec: Vector3D<S = P::S>,
{
    /// Creates a (conical) frustum
    pub fn frustum(r1: P::S, r2: P::S, h: P::S, n: usize) -> Mesh<E, V, F, P> {
        assert!(r1 >= P::S::ZERO && r2 >= P::S::ZERO && h > P::S::ZERO && n >= 3);
        assert!(
            r2 > P::S::ZERO || r1 > P::S::ZERO,
            "Must have positive volume"
        );

        let h2 = h * P::S::from(0.5);

        if r1 == P::S::ZERO {
            // TODO: use approximate comparison
            assert!(r2 > P::S::ZERO, "r2 must be positive");
            let mut mesh = Mesh::<E, V, F, P>::regular_polygon(r2, n);
            mesh.translate(&P::Vec::from_xyz(P::S::ZERO, h2, P::S::ZERO));
            mesh.extrude_to_center_point(E::new(1), P::Vec::from_xyz(P::S::ZERO, -h, P::S::ZERO));
            mesh
        } else if r2 == P::S::ZERO {
            // TODO: use approximate comparison
            assert!(r1 > P::S::ZERO, "r1 must be positive");
            let mut mesh = Mesh::<E, V, F, P>::regular_polygon(r1, n);
            mesh.flip();
            mesh.translate(&P::Vec::from_xyz(P::S::ZERO, -h2, P::S::ZERO));
            mesh.extrude_to_center_point(E::new(1), P::Vec::from_xyz(P::S::ZERO, h, P::S::ZERO));
            mesh
        } else {
            let mut mesh = Mesh::<E, V, F, P>::regular_polygon(r2, n);
            mesh.translate(&P::Vec::from_xyz(P::S::ZERO, h2, P::S::ZERO));
            mesh.extrude_ex(
                E::new(1),
                P::Trans::from_translation(P::Vec::from_xyz(P::S::ZERO, -h, P::S::ZERO))
                    .with_scale(P::Vec::from_xyz(r1 / r2, 1.0.into(), r1 / r2)),
                true,
                false,
            );
            mesh
        }
    }

    /// Creates a regular pyramid
    pub fn pyramid(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(radius, P::S::ZERO, height, n)
    }

    /// Creates a regular cone
    pub fn cone(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(radius, P::S::ZERO, height, n)
    }

    /// Creates a regular cylinder
    pub fn cylinder(radius: P::S, height: P::S, n: usize) -> Mesh<E, V, F, P> {
        Self::frustum(radius, radius, height, n)
    }

    /// Creates a regular tetrahedron centered at the origin
    pub fn tetrahedron(radius: P::S) -> Mesh<E, V, F, P> {
        let mut mesh = Self::cone(radius, radius * P::S::from(4.0 / 3.0), 3);
        mesh.translate(&P::Vec::from_xyz(P::S::ZERO, P::S::from(0.25), P::S::ZERO));
        mesh
    }

    /// Creates a regular octahedron centered at the origin
    pub fn octahedron(radius: P::S) -> Mesh<E, V, F, P> {
        let h = radius * P::S::from(4.0 / 3.0 / 2.0f32.sqrt());
        let mut mesh = Self::frustum(radius, P::S::ZERO, h, 4);
        mesh.translate(&P::Vec::from_xyz(P::S::ZERO, h * 0.5.into(), P::S::ZERO));
        mesh.extrude_to_center_point(E::new(0), P::Vec::from_xyz(P::S::ZERO, -h, P::S::ZERO));
        mesh
    }
}
