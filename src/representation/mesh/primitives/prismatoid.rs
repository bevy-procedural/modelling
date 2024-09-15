use crate::{
    math::{HasZero, Scalar, Transform, Vector, Vector3D},
    representation::{
        payload::{HasPosition, Transformable, VertexPayload},
        DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

use super::regular_polygon::regular_polygon_sidelength;

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
            .twin(self)
            .face(self)
            .expect("The polygon must have a face");
        let normal = f.normal(self).normalize();
        let e = self.extrude(first, T::Trans::from_translation(-normal * height));
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
        let normal = f.normal(self).normalize();
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
    /// Doesn't need to be a antiprism -- can also have a frustum-like shape.
    pub fn insert_antiprism_iter(
        &mut self,
        vp: impl IntoIterator<Item = T::VP>,
        vp2: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        let first = self.insert_polygon(vp);
        let e = self.loft_tri_closed(first, vp2);
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
        self.edge(first).twin_id()
    }

    /// calls `insert_pyramid` on a new mesh
    pub fn pyramid(base: impl IntoIterator<Item = T::VP>, apex: T::VP) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_pyramid(base, apex);
        mesh
    }

    /// Creates a (conical) frustum
    pub fn insert_frustum(
        &mut self,
        base: impl IntoIterator<Item = T::VP>,
        top: impl IntoIterator<Item = T::VP>,
        smooth: bool,
    ) -> T::E {
        let first = self.insert_polygon(base);
        let top_edge = self.loft_polygon(first, 2, 2, top);
        self.close_hole(top_edge, Default::default(), false);
        // TODO: smooth
        assert!(!smooth, "Smooth frustums not yet implemented");
        top_edge
    }

    /// calls `insert_frustum` on a new mesh
    pub fn frustum(
        base: impl IntoIterator<Item = T::VP>,
        top: impl IntoIterator<Item = T::VP>,
        smooth: bool,
    ) -> Self {
        let mut mesh = Mesh::<T>::new();
        mesh.insert_frustum(base, top, smooth);
        mesh
    }
}

fn circle_iter<S: Scalar, Vec: Vector<S>, VP: VertexPayload + HasPosition<Vec, S = S>>(
    r: S,
    n: usize,
    shift: S,
    y: S,
) -> impl Iterator<Item = VP> {
    (0..n).map(move |i| {
        let v = S::PI * (S::from_usize(2 * i) + shift) / S::from_usize(n);
        VP::from_pos(Vec::from_xyz(r * v.cos(), y, -r * v.sin()))
    })
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>
        + Transformable<Vec = T::Vec, Rot = T::Rot, Trans = T::Trans, S = T::S>,
{
    /// Creates a regular prism with given radius `r`, height `h`, and `n` sides.
    pub fn regular_prism(r: T::S, h: T::S, n: usize) -> Self {
        Mesh::prism(circle_iter(r, n, T::S::ZERO, T::S::ZERO), h)
    }

    /// Creates a uniform prism with given radius `r` and `n` sides.
    pub fn uniform_prism(r: T::S, n: usize) -> Self {
        Mesh::regular_prism(r, regular_polygon_sidelength(r, n), n)
    }

    /// Creates a regular antiprism with given radius `r`, height `h`, and `n` sides.
    pub fn regular_antiprism(r: T::S, h: T::S, n: usize) -> Self {
        Mesh::antiprism_iter(
            circle_iter(r, n, T::S::ZERO, T::S::ZERO),
            circle_iter(r, n, T::S::ONE, h),
        )
    }

    /// Creates a uniform antiprism with given radius `r` and `n` sides.
    pub fn uniform_antiprism(r: T::S, n: usize) -> Self {
        Mesh::regular_antiprism(
            r,
            regular_polygon_sidelength(r, n) * T::S::THREE.sqrt() * T::S::HALF,
            n,
        )
    }

    /// Creates a (conical) frustum
    pub fn regular_frustum(r1: T::S, r2: T::S, h: T::S, n: usize, smooth: bool) -> Self {
        Mesh::frustum(
            circle_iter(r1, n, T::S::ZERO, T::S::ZERO),
            circle_iter(r2, n, T::S::ZERO, h),
            smooth,
        )
    }

    /// Creates a regular pyramid
    pub fn regular_pyramid(radius: T::S, height: T::S, n: usize) -> Self {
        Mesh::pyramid(
            circle_iter(radius, n, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(T::S::ZERO, height, T::S::ZERO)),
        )
    }

    /// Creates a regular cone
    pub fn cone(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Mesh::pyramid(
            circle_iter(radius, n, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(T::S::ZERO, height, T::S::ZERO)),
        )
        // TODO: make it smooth
    }

    /// Creates a regular cylinder
    pub fn cylinder(radius: T::S, height: T::S, n: usize) -> Mesh<T> {
        Self::regular_frustum(radius, radius, height, n, true)
    }

    /// Creates a regular tetrahedron centered at the origin
    pub fn tetrahedron(radius: T::S) -> Mesh<T> {
        let mut mesh = Self::regular_pyramid(radius, radius * T::S::FOUR / T::S::THREE, 3);
        mesh.translate(&T::Vec::from_xyz(T::S::ZERO, T::S::from(0.25), T::S::ZERO));
        mesh
    }

    /// Creates a regular octahedron centered at the origin
    pub fn octahedron(radius: T::S) -> Mesh<T> {
        let zero = T::S::ZERO;
        let h = radius * T::S::FOUR / T::S::THREE / T::S::TWO.sqrt();
        let mut mesh = Mesh::new();
        let e = mesh.insert_pyramid(
            circle_iter(radius, 4, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(zero, h, zero)),
        );
        mesh.remove_face(mesh.edge(e).face_id());
        mesh.fill_hole_apex(e, T::VP::from_pos(T::Vec::from_xyz(zero, -h, zero)));
        mesh
    }
}
