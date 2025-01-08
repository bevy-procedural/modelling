use crate::{
    math::{HasPosition, Scalar, TransformTrait, Vector},
    mesh::{
        DefaultEdgePayload, DefaultFacePayload, Face3d, HalfEdge, MeshType3D, MeshTypeHalfEdge,
        VertexPayload,
    },
    operations::{MeshExtrude, MeshLoft, MeshSubdivision},
    primitives::polygon::Make2dShape,
};

use super::regular_polygon_sidelength;

/// Generates an iterator over vertices positioned in a circle.
///
/// # Parameters
///
/// - `r`: The radius of the circle.
/// - `n`: The number of vertices to generate.
/// - `shift`: A phase shift to apply to the angle of each vertex.
/// - `y`: The y-coordinate for all vertices (assuming a 3D space).
fn circle_iter<
    const D: usize,
    S: Scalar,
    Vec: Vector<S, D>,
    VP: VertexPayload + HasPosition<D, Vec, S = S>,
>(
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

// TODO: Reduce type requirements

/// A trait for creating prismatoids.
pub trait MakePrismatoid<T: MeshTypeHalfEdge<Mesh = Self> + MeshType3D<Mesh = Self>>:
    Make2dShape<T> + MeshExtrude<T> + MeshLoft<T> + MeshSubdivision<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    // Waiting for https://github.com/rust-lang/rust/issues/8995
    // type S = T::S;

    /// Creates a prism by inserting the flat polygon given by `vp` and inserting an
    /// translated copy at `height` along the normal of the face.
    /// Uses quads for the sides.
    fn insert_prism(&mut self, vp: impl IntoIterator<Item = T::VP>, height: T::S) -> T::E {
        let first = self.insert_polygon(vp);
        let twin = self.edge_ref(first).twin(self);
        let f = twin.face(self).expect("The polygon must have a face");
        let normal = Face3d::normal(f, self).normalize();
        let e = self.extrude(first, T::Trans::from_translation(-normal * height));
        e
    }

    /// calls `insert_prism` on a new mesh
    fn prism(vp: impl IntoIterator<Item = T::VP>, height: T::S) -> Self {
        let mut mesh = Self::default();
        mesh.insert_prism(vp, height);
        mesh
    }

    /// Creates an antiprism by placing the new vertices at the middle of the given
    /// polygon edges translated by `height` along the normal of the face.
    /// Uses triangles for the sides.
    ///
    /// WARNING: This doesn't produce a proper regular antiprism since the radius
    /// of the top polygon will be slightly smaller!
    fn insert_antiprism(&mut self, vp: impl IntoIterator<Item = T::VP>, height: T::S) -> T::E {
        let first = self.insert_polygon(vp);
        let f = self
            .edge_ref(first)
            .face(self)
            .expect("The polygon must have a face");
        let normal = f.normal(self).normalize();
        let e = self.extrude_tri2(
            self.edge_ref(first).twin_id(),
            T::Trans::from_translation(-normal * height),
        );
        e
    }

    /// calls `insert_antiprism` on a new mesh
    fn antiprism(vp: impl IntoIterator<Item = T::VP>, height: T::S) -> Self {
        let mut mesh = Self::default();
        mesh.insert_antiprism(vp, height);
        mesh
    }

    /// Creates an antiprism by connecting the two polygons given by `vp` and `vp2` with triangles.
    /// Doesn't need to be a antiprism -- can also have a frustum-like shape.
    fn insert_antiprism_iter(
        &mut self,
        vp: impl IntoIterator<Item = T::VP>,
        vp2: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        let first = self.insert_polygon(vp);
        let e = self.loft_tri_closed(first, vp2);
        self.insert_face(e, Default::default());
        e
    }

    /// calls `insert_antiprism_iter` on a new mesh
    fn antiprism_iter(
        vp: impl IntoIterator<Item = T::VP>,
        vp2: impl IntoIterator<Item = T::VP>,
    ) -> Self {
        let mut mesh = Self::default();
        mesh.insert_antiprism_iter(vp, vp2);
        mesh
    }

    /// Creates a pyramid by connecting the polygon given by `vp` with the point `apex`.
    fn insert_pyramid(&mut self, base: impl IntoIterator<Item = T::VP>, apex: T::VP) -> T::E {
        let first = self.insert_polygon(base);
        self.windmill(first, apex);
        self.edge_ref(first).twin_id()
    }

    /// calls `insert_pyramid` on a new mesh
    fn pyramid(base: impl IntoIterator<Item = T::VP>, apex: T::VP) -> Self {
        let mut mesh = Self::default();
        mesh.insert_pyramid(base, apex);
        mesh
    }

    /// Creates a (conical) frustum
    fn insert_frustum(
        &mut self,
        base: impl IntoIterator<Item = T::VP>,
        top: impl IntoIterator<Item = T::VP>,
        smooth: bool,
    ) -> T::E {
        let first = self.insert_polygon(base);
        let top_edge = self.loft(first, 2, 2, top).unwrap().0;
        self.insert_face(top_edge, Default::default());
        // TODO: smooth
        assert!(!smooth, "Smooth frustums not yet implemented");
        top_edge
    }

    /// calls `insert_frustum` on a new mesh
    fn frustum(
        base: impl IntoIterator<Item = T::VP>,
        top: impl IntoIterator<Item = T::VP>,
        smooth: bool,
    ) -> Self {
        let mut mesh = Self::default();
        mesh.insert_frustum(base, top, smooth);
        mesh
    }

    /// create a rectangular cuboid with side lengths `x`, `y`, and `z`
    fn cuboid(size: T::Vec) -> T::Mesh {
        let p = size * T::S::HALF;
        let mut mesh = Self::default();
        let vp = |x, y, z| T::VP::from_pos(T::Vec::from_xyz(x, y, z));

        let bottom_edge = mesh.insert_polygon([
            vp(-p.x(), -p.y(), -p.z()),
            vp(p.x(), -p.y(), -p.z()),
            vp(p.x(), p.y(), -p.z()),
            vp(-p.x(), p.y(), -p.z()),
        ]);
        let top_edge = mesh.loft(
            bottom_edge,
            2,
            2,
            [
                vp(-p.x(), -p.y(), p.z()),
                vp(p.x(), -p.y(), p.z()),
                vp(p.x(), p.y(), p.z()),
                vp(-p.x(), p.y(), p.z()),
            ],
        ).unwrap().0;
        mesh.insert_face(top_edge, Default::default());
        mesh
    }

    /// create a cube with side length `x`
    fn cube(x: T::S) -> T::Mesh {
        Self::cuboid(T::Vec::splat(x))
    }

    /// an alias for `cube`
    fn regular_hexahedron(l: T::S) -> T::Mesh {
        Self::cube(l)
    }

    /// Creates a regular pyramid
    fn regular_pyramid(radius: T::S, height: T::S, n: usize) -> Self {
        Self::pyramid(
            circle_iter(radius, n, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(T::S::ZERO, height, T::S::ZERO)),
        )
    }

    /// Creates a regular cone
    fn cone(radius: T::S, height: T::S, n: usize) -> T::Mesh {
        Self::pyramid(
            circle_iter(radius, n, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(T::S::ZERO, height, T::S::ZERO)),
        )
        // TODO: make it smooth
    }

    /// Creates a regular tetrahedron with edge length `l` centered at the origin
    fn regular_tetrahedron(l: T::S) -> T::Mesh {
        let e = T::S::HALF * l;
        let sq = e / T::S::TWO.sqrt();
        let zero = T::S::ZERO;
        Self::pyramid(
            [
                T::VP::from_pos(T::Vec::from_xyz(-e, zero, -sq)),
                T::VP::from_pos(T::Vec::from_xyz(zero, -e, sq)),
                T::VP::from_pos(T::Vec::from_xyz(e, zero, -sq)),
            ],
            T::VP::from_pos(T::Vec::from_xyz(zero, e, sq)),
        )
    }

    /// Creates a regular octahedron with a given circumscribed `radius` centered at the origin
    fn regular_octahedron(radius: T::S) -> T::Mesh {
        let zero = T::S::ZERO;
        let h = radius;
        let mut mesh = Self::default();
        let e = mesh.insert_pyramid(
            circle_iter(radius, 4, T::S::ZERO, T::S::ZERO),
            T::VP::from_pos(T::Vec::from_xyz(zero, h, zero)),
        );
        mesh.remove_face(mesh.edge_ref(e).face_id());
        mesh.windmill(e, T::VP::from_pos(T::Vec::from_xyz(zero, -h, zero)));
        mesh
    }

    /// Creates a (conical) frustum
    fn regular_frustum(r1: T::S, r2: T::S, h: T::S, n: usize, smooth: bool) -> Self {
        Self::frustum(
            circle_iter(r1, n, T::S::ZERO, T::S::ZERO),
            circle_iter(r2, n, T::S::ZERO, h),
            smooth,
        )
    }

    /// Creates a regular cylinder
    fn cylinder(radius: T::S, height: T::S, n: usize) -> T::Mesh {
        Self::regular_frustum(radius, radius, height, n, true)
    }

    /// Creates a regular prism with given radius `r`, height `h`, and `n` sides.
    fn regular_prism(r: T::S, h: T::S, n: usize) -> Self {
        Self::prism(circle_iter(r, n, T::S::ZERO, T::S::ZERO), h)
    }

    /// Creates a uniform prism with given radius `r` and `n` sides.
    fn uniform_prism(r: T::S, n: usize) -> Self {
        Self::regular_prism(r, regular_polygon_sidelength(r, n), n)
    }

    /// Creates a regular antiprism with given radius `r`, height `h`, and `n` sides.
    fn regular_antiprism(r: T::S, h: T::S, n: usize) -> Self {
        Self::antiprism_iter(
            circle_iter(r, n, T::S::ZERO, T::S::ZERO),
            circle_iter(r, n, T::S::ONE, h),
        )
    }

    /// Creates a uniform antiprism with given radius `r` and `n` sides.
    fn uniform_antiprism(r: T::S, n: usize) -> Self {
        // TODO: isn't this the edge length?
        Self::regular_antiprism(
            r,
            regular_polygon_sidelength(r, n) * T::S::THREE.sqrt() * T::S::HALF,
            n,
        )
    }
}
