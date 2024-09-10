use crate::{
    math::{HasZero, Scalar, Transform},
    representation::{payload::VertexPayload, Mesh, MeshType},
};

impl<T: MeshType> Mesh<T> {
    // Waiting for https://github.com/rust-lang/rust/issues/8995
    // type S = T::S;

    /// Creates a (conical) frustum
    pub fn frustum(r1: T::S, r2: T::S, h: T::S, n: usize) -> Mesh<T> {
        let z = T::S::ZERO;

        assert!(r1 >= z && r2 >= z && h > z && n >= 3);
        assert!(r2 > z || r1 > z, "Must have positive volume");

        let h2 = h * T::S::from(0.5);

        if r1 == z {
            // TODO: use approximate comparison
            assert!(r2 > z, "r2 must be positive");
            let mut mesh = Mesh::<T>::regular_polygon(r2, n);
            mesh.translate(&T::Vec::from_xyz(z, h2, z));
            mesh.extrude_to_center_point(T::E::new(1), T::Vec::from_xyz(z, -h, z));
            mesh
        } else if r2 == z {
            // TODO: use approximate comparison
            assert!(r1 > z, "r1 must be positive");
            let mut mesh = Mesh::<T>::regular_polygon(r1, n);
            mesh.flip();
            mesh.translate(&T::Vec::from_xyz(z, -h2, z));
            mesh.extrude_to_center_point(T::E::new(1), T::Vec::from_xyz(z, h, z));
            mesh
        } else {
            let mut mesh = Mesh::<T>::regular_polygon(r2, n);
            mesh.translate(&T::Vec::from_xyz(z, h2, z));
            mesh.extrude_ex(
                T::E::new(1),
                T::Trans::from_translation(T::Vec::from_xyz(z, -h, z))
                    .with_scale(T::Vec::from_xyz(r1 / r2, 1.0.into(), r1 / r2)),
                true,
                false,
            );
            mesh
        }
    }

    /// Creates a regular pyramid
    pub fn pyramid(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::frustum(radius, T::S::ZERO, height, n)
    }

    /// Creates a regular cone
    pub fn cone(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::frustum(radius, T::S::ZERO, height, n)
    }

    /// Creates a regular cylinder
    pub fn cylinder(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::frustum(radius, radius, height, n)
    }

    /// Creates a regular tetrahedron centered at the origin
    pub fn tetrahedron(radius: T::S) -> Mesh<T> {
        let mut mesh = Self::cone(radius, radius * T::S::from(4.0 / 3.0), 3);
        mesh.translate(&T::Vec::from_xyz(T::S::ZERO, T::S::from(0.25), T::S::ZERO));
        mesh
    }

    /// Creates a regular octahedron centered at the origin
    pub fn octahedron(radius: T::S) -> Mesh<T> {
        let z = T::S::ZERO;
        let h = radius * T::S::from(4.0 / 3.0 / 2.0f32.sqrt());
        let mut mesh = Self::frustum(radius, z, h, 4);
        mesh.translate(&T::Vec::from_xyz(z, h * 0.5.into(), z));
        mesh.extrude_to_center_point(T::E::new(0), T::Vec::from_xyz(z, -h, z));
        mesh
    }
}
