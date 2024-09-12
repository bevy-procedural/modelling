use crate::{
    math::{Scalar, Vector, Vector3D},
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

        // an edge on the boundary of the previous layer
        let mut prev = T::E::default();

        for i in 0..n {
            if i == 0 {
                // top pole
                let (first, last) = mesh.insert_path((0..m).map(|j| make_vp(i + 1, j)));
                mesh.insert_edge(first, Default::default(), last, Default::default());
                mesh.fill_hole_with_vertex(last, make_vp(i, 0));
                prev = first;
            } else if i == n - 1 {
                // bottom pole
                mesh.fill_hole_with_vertex(prev, make_vp(i + 1, 0));
            } else {
                // normal squares
                let new_edge = mesh.quad_hem(prev, (0..m).map(|j| make_vp(i + 1, j)));
                mesh.close_face_default(
                    mesh.edge(new_edge).next(&mesh).next(&mesh).next_id(),
                    new_edge,
                    false,
                );
                prev = new_edge;
            }
        }

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
