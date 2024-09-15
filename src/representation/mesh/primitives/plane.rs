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
    /// Generate a subdivided plane made of triangles with given `width` and `height` and
    /// `n` and `m` vertices used for the subdivisions, i.e., to subdivide the plane into
    /// four columns, use `n = 5`.
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
            let e = mesh.loft_tri_back(first, j % 2 == 0, iter(j));
            first = mesh.edge(e).prev_id();
        }

        mesh
    }

    /// Generate a subdivided plane made of quads with given `width` and `height` and
    /// `n` and `m` vertices used for the subdivisions, i.e., to subdivide the plane into
    /// four columns, use `n = 5`.
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
            first = mesh.loft_polygon_back(first, 2, 2, iter(j));
        }

        mesh
    }

    /// Generate a subdivided plane made of hexagons with `n` and `m` vertices used for the subdivisions.
    /// TODO: Make this more quadratic and less parallelogram.
    pub fn hex_plane(n: usize, m: usize) -> Self {
        assert!(n % 2 == 0);
        assert!(m >= 2);
        let mut mesh = Self::new();
        let row_height = T::S::from_usize(3) / T::S::from_usize(3).sqrt();
        let width = T::S::ONE;
        let hex_offset = row_height - T::S::from_usize(2) / T::S::from_usize(3).sqrt();
        let iter = |offset: usize, j: usize| {
            (0..n).map(move |i| {
                T::VP::from_pos(T::Vec::from_xy(
                    width * T::S::from_usize(i + offset),
                    row_height * T::S::from_usize(j)
                        + T::S::from_usize((i + j + 1 + offset) % 2) * hex_offset,
                ))
            })
        };

        let (mut first, _) = mesh.insert_path(iter(0, 0));
        for j in 1..m {
            if j >= 2 {
                first = mesh.edge(first).prev_id();
            }
            first = mesh.loft_polygon_back(first, 3, 3, iter(j - 1, j));
        }

        mesh
    }
}
