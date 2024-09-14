use crate::{
    math::{Scalar, Vector},
    representation::{
        payload::HasPosition, DefaultEdgePayload, DefaultFacePayload, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
    /// Generate a subdivided plane made of triangles with given width and height and n and m subdivisions
    pub fn triangle_plane(width: T::S, height: T::S, n: usize, m: usize) -> Self {
        let mut mesh = Self::new();
        let vertical_step = height / T::S::from_usize(m - 1);
        let half_horizontal_step = width / T::S::from_usize(n - 1) / T::S::from_usize(2);
        let iter = |j: usize| {
            (0..n).map(move |i| {
                T::VP::from_pos(T::Vec::from_xy(
                    half_horizontal_step * T::S::from_usize(i * 2 + (j % 2)),
                    vertical_step * T::S::from_usize(j),
                ))
            })
        };

        let (mut first, _) = mesh.insert_path(iter(0));
        for j in 1..m {
            let e = mesh.loft_tri(first, j % 2 == 0, iter(j));
            first = mesh.edge(e).prev_id();
        }

        mesh
    }

    /// Generate a subdivided plane made of quads with given width and height and n and m subdivisions
    pub fn quad_plane(width: T::S, height: T::S, n: usize, m: usize) -> Self {
        let mut mesh = Self::new();
        let vertical_step = height / T::S::from_usize(m - 1);
        let horizontal_step = width / T::S::from_usize(n - 1);
        let iter = |j: usize| {
            (0..n).map(move |i| {
                T::VP::from_pos(T::Vec::from_xy(
                    horizontal_step * T::S::from_usize(i),
                    vertical_step * T::S::from_usize(j),
                ))
            })
        };

        let (mut first, _) = mesh.insert_path(iter(0));
        for j in 1..m {
            first = mesh.loft_quads(first, iter(j));
        }

        mesh
    }

    // TODO: Hexa plane and other common plane tilings!
}
