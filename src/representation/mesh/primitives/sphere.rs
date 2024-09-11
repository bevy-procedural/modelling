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
        let vs0 = mesh.insert_line(T::Vec::ZERO, T::Vec::from_xy(width, T::S::ZERO), n);

        let vs1 = mesh.insert_line(
            T::Vec::from_xy(half_horizontal_step, vertical_step),
            T::Vec::from_xy(width + half_horizontal_step, vertical_step),
            n,
        );

        mesh.insert_edge_between(vs0[0], Default::default(), vs1[0], Default::default());

        mesh.close_face_default(
            mesh.shared_edge_id(vs0[0], vs1[0]).unwrap(),
            mesh.shared_edge_id(vs0[2], vs0[1]).unwrap(),
            false,
        );

        println!("{}",mesh);

        mesh.close_face_default(
            mesh.shared_edge_id(vs1[0], vs1[1]).unwrap(),
            mesh.shared_edge_id(vs0[2], vs0[1]).unwrap(),
            false,
        );

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
