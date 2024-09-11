use crate::{
    math::{HasZero, Scalar, Vector, Vector3D},
    representation::{DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType},
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::Vec: Vector3D<S = T::S>,
{
    /// Generate a plane made of triangles with given width and height and n and m subdivisions
    pub fn triangle_plane(width: T::S, height: T::S, n: usize, m: usize) -> Self {
        let mut mesh = Self::new();
        let vertical_step = height / T::S::from_usize(m - 1);
        let half_horizontal_step = width / T::S::from_usize(n - 1) / T::S::from_usize(2);
        let mut vs = mesh.insert_line(T::Vec::ZERO, T::Vec::from_xy(width, T::S::ZERO), n);
        let mut top_end = vs[n - 2];
        for j in 1..n {
            let js = T::S::from_usize(j);
            let y = vertical_step * js;
            let vs_new = mesh.insert_line(
                T::Vec::from_xy(half_horizontal_step * js, y),
                T::Vec::from_xy(width + half_horizontal_step * js, y),
                n,
            );
            mesh.insert_edge_between(vs[0], Default::default(), vs_new[0], Default::default());

            for i in (0..(n - 1)).step_by(2) {
                let start = if i == 0 { vs[0] } else { vs_new[i - 1] };
                let outside = if i == n - 2 {
                    mesh.shared_edge_id(top_end, vs[i + 1])
                } else {
                    mesh.shared_edge_id(vs[i + 2], vs[i + 1])
                }
                .unwrap();

                mesh.close_face_default(
                    mesh.shared_edge_id(start, vs_new[i]).unwrap(),
                    outside,
                    false,
                );
                mesh.close_face_default(
                    mesh.shared_edge_id(vs_new[i], vs_new[i + 1]).unwrap(),
                    outside,
                    false,
                );
            }
            top_end = vs[n - 1];
            vs = vs_new;
        }
        mesh
    }

    /// Create a uv sphere
    pub fn uv_sphere(radius: T::S, n: usize) -> Self {
        // https://catlikecoding.com/unity/tutorials/procedural-meshes/uv-sphere/
        //todo!("uv_sphere")
        let mut mesh = Self::triangle_plane(radius, radius, n, n);
        mesh.flip_yz();
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
