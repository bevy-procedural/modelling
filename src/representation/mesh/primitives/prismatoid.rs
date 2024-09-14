use crate::{
    math::{HasZero, IndexType, Transform, Vector, Vector3D},
    representation::{
        payload::{HasPosition, Transformable},
        DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans>,
{
    // Waiting for https://github.com/rust-lang/rust/issues/8995
    // type S = T::S;

    /// TODO: Instead, generate a prism or antirprism
    pub fn prism(vp: impl IntoIterator<Item = T::VP>, height: T::Vec) -> Mesh<T> {
        let mut mesh = Mesh::new();
        let first = mesh.insert_polygon(vp);
        mesh.extrude(mesh.edge(first).twin_id(), height, true);
        mesh
    }

    /*pub fn antiprism(vp: impl IntoIterator<Item = T::VP>, height: T::Vec) -> Mesh<T> {
        todo!("antiprism")
    }

    pub fn pyramid(base: impl IntoIterator<Item = (T::VP, T::VP)>, apex: T::VP) -> Mesh<T> {
        todo!("pyramid")
    }

    /// Creates a (conical) frustum
    /// TODO: smooth!
    pub fn frustum(
        base: impl IntoIterator<Item = (T::VP, T::VP)>,
        top: impl IntoIterator<Item = (T::VP, T::VP)>,
        smooth: bool,
    ) -> Mesh<T> {
        todo!("frustum")
    }*/
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
    T::VP:
        HasPosition<T::Vec, S = T::S> + Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans>,
{
    pub fn regular_prism(r1: T::S, r2: T::S, h: T::S, n: usize) -> Mesh<T> {
        let z = T::S::ZERO;

        assert!(r1 >= z && r2 >= z && h > z && n >= 3);
        assert!(r2 > z || r1 > z, "Must have positive volume");
        assert!(r1 > z, "r1 must be positive");
        assert!(r2 > z, "r2 must be positive");

        let h2 = h * T::S::from(0.5);

        if r1 == z {
            // TODO: use approximate comparison
            assert!(r2 > z, "r2 must be positive");
            let mut mesh = Mesh::<T>::regular_polygon(r2, n);
            mesh.flip_yz()
                .translate(&T::Vec::from_xyz(z, h2, z))
                .extrude_to_center_point(T::E::new(1), T::Vec::from_xyz(z, -h, z));
            mesh
        } else if r2 == z {
            // TODO: use approximate comparison
            assert!(r1 > z, "r1 must be positive");
            let mut mesh = Mesh::<T>::regular_polygon(r1, n);
            mesh.flip_yz()
                .flip()
                .translate(&T::Vec::from_xyz(z, -h2, z))
                .extrude_to_center_point(T::E::new(1), T::Vec::from_xyz(z, h, z));
            mesh
        } else {
            let mut mesh = Mesh::<T>::regular_polygon(r2, n);
            mesh.flip_yz()
                .translate(&T::Vec::from_xyz(z, h2, z))
                .extrude_ex(
                    T::E::new(1),
                    T::Trans::from_translation(T::Vec::from_xyz(z, -h, z))
                        .with_scale(T::Vec::from_xyz(r1 / r2, 1.0.into(), r1 / r2)),
                    true,
                    false,
                );
            mesh
        }
    }

    /*pub fn uniform_antiprism(r: T::S, h: T::S, n: usize) -> Mesh<T> {
        todo!("uniform_antiprism")
    }

    /// Creates a (conical) frustum
    pub fn regular_frustum(r1: T::S, r2: T::S, h: T::S, n: usize, smooth: bool) -> Mesh<T> {
        todo!("frustum")
    }

    /// Creates a regular pyramid
    pub fn regular_pyramid(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        //Self::regular_prism(radius, T::S::ZERO, height, n)
        todo!("frustum")
    }

    /// Creates a regular cone
    pub fn cone(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::regular_frustum(radius, T::S::ZERO, height, n, true)
    }

    /// Creates a regular cylinder
    pub fn cylinder(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::regular_frustum(radius, radius, height, n, true)
    }

    /// Creates a regular tetrahedron centered at the origin
    pub fn tetrahedron(radius: T::S) -> Mesh<T> {
        let mut mesh = Self::regular_pyramid(radius, radius * T::S::from(4.0 / 3.0), 3);
        mesh.translate(&T::Vec::from_xyz(T::S::ZERO, T::S::from(0.25), T::S::ZERO));
        mesh
    }

    /// Creates a regular octahedron centered at the origin
    pub fn octahedron(radius: T::S) -> Mesh<T> {
        let zero = T::S::ZERO;
        let h = radius * T::S::from(4.0 / 3.0 / 2.0f32.sqrt());
        let mut mesh = Self::regular_pyramid(radius, h, 4);
        mesh.translate(&T::Vec::from_xyz(zero, h * 0.5.into(), zero));
        mesh.extrude_to_center_point(T::E::new(0), T::Vec::from_xyz(zero, -h, zero));
        mesh
    }*/
}
