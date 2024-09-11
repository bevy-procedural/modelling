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

        // the edge in the previous layer pointing towards the left appended edge
        let mut last_layer_output = T::E::default();

        for i in 0..n {
            if i == 0 {
                // top pole

                // the tip for the top pole
                let (tip, first) = mesh.add_isolated_edge_default(make_vp(i, 0), make_vp(i + 1, 0));

                let input = mesh.shared_edge_id(first, tip).unwrap();
                let tip2first = mesh.shared_edge_id(tip, first).unwrap();

                // the edge coming from the tip to the last inserted vertex
                let mut output = tip2first;

                for j in 1..m {
                    let (_, outside, _) =
                        mesh.add_vertex_via_edge_default(input, output, make_vp(i + 1, j));
                    let inside = output;
                    mesh.close_face_default(inside, outside, false);
                    output = outside;
                }
                let (_, _, new_edge) = mesh.close_face_default(
                    output,
                    mesh.edge(tip2first).next(&mesh).twin_id(),
                    false,
                );
                last_layer_output = new_edge;
            } else if i == n - 1 {
                // bottom pole

                let mut base_input = mesh.edge(last_layer_output).prev_id();
                mesh.add_vertex_via_edge_default(base_input, last_layer_output, make_vp(i + 1, 0));
                for _ in 1..m {
                    base_input = mesh.edge(base_input).prev_id();
                    mesh.close_face_default(
                        mesh.edge(base_input).next(&mesh).next_id(),
                        base_input,
                        false,
                    );
                }
                mesh.close_hole(base_input, Default::default(), false);
            } else {
                // normal squares

                let mut base_input = mesh.edge(last_layer_output).prev_id();

                mesh.add_vertex_via_edge_default(base_input, last_layer_output, make_vp(i + 1, 0));

                for j in 1..m {
                    base_input = mesh.edge(base_input).prev_id();
                    let output = mesh.edge(base_input).next(&mesh);
                    mesh.add_vertex_via_edge_default(base_input, output.id(), make_vp(i + 1, j));
                    mesh.close_face_default(
                        output.next_id(),
                        mesh.edge(base_input).next_id(),
                        false,
                    );
                }

                let bi = mesh.edge(base_input);
                let (_, _, new_edge) =
                    mesh.close_face_default(bi.next_id(), bi.prev(&mesh).prev_id(), false);
                last_layer_output = new_edge;
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
