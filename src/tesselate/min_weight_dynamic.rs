use crate::{
    math::{HasPosition, Vector3D},
    mesh::{Face3d, MeshType, Triangulation},
};


/// The [min-weight triangulation problem](https://en.wikipedia.org/wiki/Minimum-weight_triangulation)
/// is, in general, NP-hard. But there are quasi-polynomial approximation schemes.
/// Delaunay is within Theta(n) and Greedy within Theta(sqrt(n)) of the minimum weight triangulation.
/// However, for random point sets they are both within O(log n).
///
/// a min-weight dynamic programming algorithm that considers all possible simple cycle
/// separators of O(sqrt n) points within the triangulation, recursively finds the optimal
/// triangulation on each side of the cycle, and chooses the cycle separator leading to the
/// smallest total weight. It should run in O(2^(sqrt(n)*log(n))) time.
///
/// See Lingas, Andrzej (1998), "Subexponential-time algorithms for minimum weight triangulations and related problems"
pub fn minweight_dynamic<T: MeshType>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    todo!("min_weight_dynamic");
}

