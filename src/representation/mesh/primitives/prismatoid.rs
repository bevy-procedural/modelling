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
    T::VP: Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans, S = T::S>
        + HasPosition<T::Vec, S = T::S>,
    T::Vec: Vector3D<S = T::S>,
{
    // Waiting for https://github.com/rust-lang/rust/issues/8995
    // type S = T::S;

    /// Creates a prism by inserting the flat polygon given by `vp` and inserting an
    /// translated copy at `height` along the normal of the face.
    /// Uses quads for the sides.
    pub fn insert_prism(&mut self, vp: impl IntoIterator<Item = T::VP>, height: T::S) -> T::E {
        let first = self.insert_polygon(vp);
        let f = self
            .edge(first)
            .face(self)
            .expect("The polygon must have a face");
        let normal = f.normal(self);
        let e = self.extrude(
            self.edge(first).twin_id(),
            T::Trans::from_translation(-normal * height),
        );
        e
    }

    /// calls `insert_prism` on a new mesh
    pub fn prism(vp: impl IntoIterator<Item = T::VP>, height: T::S) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_prism(vp, height);
        mesh
    }

    /// Creates an antiprism by placing the new vertices at the middle of the given
    /// polygon edges translated by `height` along the normal of the face.
    /// Uses triangles for the sides.
    ///
    /// WARNING: This doesn't produce a proper regular antiprism since the radius
    /// of the top polygon will be slightly smaller!
    pub fn insert_antiprism(&mut self, vp: impl IntoIterator<Item = T::VP>, height: T::S) -> T::E {
        let first = self.insert_polygon(vp);
        let f = self
            .edge(first)
            .face(self)
            .expect("The polygon must have a face");
        let normal = f.normal(self);
        let e = self.extrude_tri(
            self.edge(first).twin_id(),
            T::Trans::from_translation(-normal * height),
        );
        e
    }

    /// calls `insert_antiprism` on a new mesh
    pub fn antiprism(vp: impl IntoIterator<Item = T::VP>, height: T::S) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_antiprism(vp, height);
        mesh
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Creates an antiprism by connecting the two polygons given by `vp` and `vp2` with triangles.
    pub fn insert_antiprism_iter(
        &mut self,
        vp: impl IntoIterator<Item = T::VP>,
        vp2: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        let first = self.insert_polygon(vp);
        let e = self.loft_tri_closed(self.edge(first).twin_id(), vp2);
        self.close_hole(e, Default::default(), false);
        e
    }

    /// calls `insert_antiprism_iter` on a new mesh
    pub fn antiprism_iter(
        vp: impl IntoIterator<Item = T::VP>,
        vp2: impl IntoIterator<Item = T::VP>,
    ) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_antiprism_iter(vp, vp2);
        mesh
    }

    /// Creates a pyramid by connecting the polygon given by `vp` with the point `apex`.
    pub fn insert_pyramid(&mut self, base: impl IntoIterator<Item = T::VP>, apex: T::VP) -> T::E {
        let first = self.insert_polygon(base);
        self.fill_hole_apex(first, apex);
        first        
    }

    /// calls `insert_pyramid` on a new mesh
    pub fn pyramid(base: impl IntoIterator<Item = T::VP>, apex: T::VP) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_pyramid(base, apex);
        mesh
    }


    /*
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
        todo!("regular_prism")

        /*let z = T::S::ZERO;

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
        }*/
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
