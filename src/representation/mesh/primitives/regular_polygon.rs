use crate::{
    math::{HasZero, Scalar, Vector},
    representation::{
        payload::VertexPayload, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// create a regular polygon
    pub fn regular_polygon(radius: T::S, n: usize) -> Mesh<T> {
        Self::regular_star(radius, radius, n)
    }

    /// Draw a polygon from the given vertices
    pub fn polygon(v: &[T::Vec]) -> Mesh<T> {
        // TODO: create this directly without the builder functions
        // TODO: assertions
        let mut mesh = Mesh::<T>::new();
        assert!(v.len() >= 3);
        let (v0, mut current) = mesh.add_isolated_edge(
            T::VP::from_pos(v[0]),
            Default::default(),
            T::VP::from_pos(v[1]),
            Default::default(),
        );
        let mut last = current;
        for i in 2..v.len() {
            last = current;
            current = mesh
                .add_vertex_via_vertex(
                    current,
                    T::VP::from_pos(v[i]),
                    Default::default(),
                    Default::default(),
                )
                .0;
        }
        mesh.close_face_vertices(
            last,
            Default::default(),
            current,
            Default::default(),
            v0,
            Default::default(),
            false,
        );
        mesh
    }

    /// create a regular star, i.e., a regular polygon with two radii
    pub fn regular_star(inner_radius: T::S, outer_radius: T::S, n: usize) -> Mesh<T> {
        let pi2n = 2.0 * std::f32::consts::PI / (n as f32);
        let a0 = 0.0 as f32;
        let a1 = pi2n;
        let mut mesh = Mesh::<T>::new();

        let (v0, mut current) = mesh.add_isolated_edge(
            T::VP::from_pos(T::Vec::from_xy(
                inner_radius * T::S::from(a0.sin()),
                inner_radius * T::S::from(a0.cos()),
            )),
            Default::default(),
            T::VP::from_pos(T::Vec::from_xy(
                outer_radius * T::S::from(a1.sin()),
                outer_radius * T::S::from(a1.cos()),
            )),
            Default::default(),
        );
        let mut prev = v0;

        for i in 2..n {
            let r = if i % 2 == 1 {
                outer_radius
            } else {
                inner_radius
            };
            let angle = pi2n * (i as f32);
            prev = current;
            current = mesh
                .add_vertex_via_vertex(
                    current,
                    T::VP::from_pos(T::Vec::from_xy(
                        r * T::S::from(angle.sin()),
                        r * T::S::from(angle.cos()),
                    )),
                    Default::default(),
                    Default::default(),
                )
                .0;
        }

        mesh.close_face_vertices(
            prev,
            Default::default(),
            current,
            Default::default(),
            v0,
            Default::default(),
            false,
        );

        mesh
    }

    /// Generate a subdivided plane made of triangles with given width and height and n and m subdivisions
    pub fn triangle_plane(width: T::S, height: T::S, n: usize, m: usize) -> Self {
        let mut mesh = Self::new();
        let vertical_step = height / T::S::from_usize(m - 1);
        let half_horizontal_step = width / T::S::from_usize(n - 1) / T::S::from_usize(2);
        let mut vs = mesh.insert_line(T::Vec::ZERO, T::Vec::from_xy(width, T::S::ZERO), n);
        let mut top_end = vs[n - 2];
        for j in 1..n {
            let js = T::S::from_usize(j);
            let y = vertical_step * js;
            // TODO: simplify this by not generating a line here
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
}
