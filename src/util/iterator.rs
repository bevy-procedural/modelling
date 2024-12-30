use crate::{
    math::{HasPosition, Scalar, Vector},
    mesh::EuclideanMeshType,
};

/// Iterator ccw over a regular polygon with `n` sides and radius `r`.
pub fn circle_iter<const D: usize, T: EuclideanMeshType<D>>(
    n: usize,
    r: T::S,
    shift: T::S,
) -> impl Iterator<Item = T::VP> {
    let npi2: T::S = T::S::TWO / T::S::from_usize(n) * T::S::PI;
    (0..n).map(move |i| {
        T::VP::from_pos(T::Vec::from_xy(
            ((T::S::from_usize(i) + shift) * npi2).sin() * r,
            ((T::S::from_usize(i) + shift) * npi2).cos() * r,
        ))
    })
}

/// Iterator cw over a regular polygon with `n` sides and radius `r`.
pub fn circle_iter_back<const D: usize, T: EuclideanMeshType<D>>(
    n: usize,
    r: T::S,
    shift: T::S,
) -> impl Iterator<Item = T::VP> {
    let npi2: T::S = -T::S::TWO / T::S::from_usize(n) * T::S::PI;
    (0..n).map(move |i| {
        T::VP::from_pos(T::Vec::from_xy(
            ((T::S::from_usize(i) + shift) * npi2).sin() * r,
            ((T::S::from_usize(i) + shift) * npi2).cos() * r,
        ))
    })
}
