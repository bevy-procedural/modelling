use crate::{
    math::{HasZero, IndexType, Scalar, Vector, Vector3D},
    representation::{
        payload::VertexPayload, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
{
    /// Create a uv sphere with a given `radius`.
    /// `n` is the number of rings (including the two made of triangular faces).
    /// `m` is the number of columns.
    pub fn uv_sphere(radius: T::S, n: usize, m: usize) -> Self {
        // TODO: https://catlikecoding.com/unity/tutorials/procedural-meshes/uv-sphere/
        assert!(n >= 2);
        assert!(m >= 3);

        let mut mesh = Self::new();
        let sn = T::S::from_usize(n);
        let sm = T::S::from_usize(m);

        let make_vp = |i, j| {
            // i goes from the top of the sphere to the bottom. Hence, phi goes from 0 to PI.
            let phi = T::S::PI * T::S::from_usize(i) / sn;

            // j goes around the sphere. Hence, theta goes from 0 to 2*PI.
            let theta = -T::S::PI * T::S::from_usize(2 * j + 4) / sm;

            T::VP::from_pos(T::Vec::from_xyz(
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin(),
            ))
        };

        // top pole
        let mut prev = mesh.insert_loop((0..m).map(|j| make_vp(1, j)));
        mesh.fill_hole_with_vertex(mesh.edge(prev).twin_id(), make_vp(0, 0));

        // normal squares
        for i in 1..(n - 1) {
            prev = mesh.loft_quads_closed(prev, (0..m).map(|j| make_vp(i + 1, j)));
        }

        // bottom pole
        mesh.fill_hole_with_vertex(prev, make_vp(n, 0));

        mesh
    }

    /// Create a dodecahedron with a given `radius`.
    pub fn dodecahedron(radius: T::S) -> Self {
        // https://en.wikipedia.org/wiki/Regular_dodecahedron#/media/File:Dodecahedron_vertices.svg

        let phi = radius * T::S::PHI;
        let iphi = radius / T::S::PHI;
        let one = radius;
        let zero = T::S::ZERO;

        let mut mesh = Self::polygon(&[
            T::Vec::from_xyz(one, one, one),    // orange
            T::Vec::from_xyz(zero, phi, iphi),  // green
            T::Vec::from_xyz(-one, one, one),   // orange
            T::Vec::from_xyz(-iphi, zero, phi), // blue
            T::Vec::from_xyz(iphi, zero, phi),  // blue
        ]);

        // TODO: polygon should return something more helpful
        let start = mesh.shared_edge_id(T::V::new(1), T::V::new(0)).unwrap();

        let make_vp = |x, y, z| T::VP::from_pos(T::Vec::from_xyz(x, y, z));
        let start_middle = mesh.loft_polygon(
            start,
            3,
            2,
            [
                make_vp(phi, iphi, zero),   // pink
                make_vp(one, one, -one),    // orange
                make_vp(zero, phi, -iphi),  // green
                make_vp(-one, one, -one),   // orange
                make_vp(-phi, iphi, zero),  // pink
                make_vp(-phi, -iphi, zero), // pink
                make_vp(-one, -one, one),   // orange
                make_vp(zero, -phi, iphi),  // green
                make_vp(one, -one, one),    // orange
                make_vp(phi, -iphi, zero),  // pink
            ],
        );

        let start_bottom = mesh.loft_polygon(
            mesh.edge(start_middle).next_id(),
            2,
            3,
            [
                make_vp(one, -one, -one),   // orange
                make_vp(iphi, zero, -phi),  // blue
                make_vp(-iphi, zero, -phi), // blue
                make_vp(-one, -one, -one),  // orange
                make_vp(zero, -phi, -iphi), // green
            ],
        );

        mesh.close_hole(start_bottom, Default::default(), false);

        mesh
    }

    /*pub fn cubesphere(radius: T::S, n: usize) -> Self {
        todo!("cubesphere")
    }
    pub fn octasphere(radius: T::S, n: usize) -> Self {
        todo!("octasphere")
    }
    pub fn geodesic_octaspere(radius: T::S, n: usize) -> Self {
        todo!("geodesic_octaspere")
    }
    pub fn seamless_cubesphere(radius: T::S, n: usize) -> Self {
        todo!("seamless_cubesphere")
    }
    pub fn icosphere(radius: T::S, n: usize) -> Self {
        todo!("icosphere")
    }*/
}
