use crate::mesh::{MeshType3D, Triangulation};

/// Simple greedy approach to approximate the min-weight triangulation of a polygon by
/// always inserting the shortest non-intersecting chord from a small local neighborhood.
/// Runs in O(n log^2 n) time.
pub fn minweight_greedy<T: MeshType3D>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
) {
    // TODO: We could also greedily search for the shortest non-intersecting chord and then insert that one. Should give a decent solution.

    // TODO: try to use a segment tree (using axis-parallel rectangles) to speed up the search for intersections (lookup in O(log^2 n) time).

    // TODO: Alternatively (because spatial datastructures might not be nlogn here),
    // we could use the monotone partitioning from the sweep line algo and than optimize edge
    // lengths in each monotone polygon. After that, we could run a few iterations of local
    // search to converge to the next local optimum.

    // TODO: Also try one of the QPTAS algorithms,
    // e.g., Remy, Steger: "A quasi-polynomial time approximation scheme for Minimum Weight Triangulation" (2009), https://arxiv.org/pdf/1301.2253
    // computes a (1+eps)-approximation in n^O((log^8 n) eps^-5) time.
    // even though its probably not suitable for implementation,
    // it might give some interesting theorems to improve the greedy algorithm.

    // TODO: Use the fact, that simple polygons with few inner points can be triangulated fast!
    // in O(6^k n^5 log n) (Hoffmann and Okamoto 2004)
    // in O(n^3 k! k) (Grantson et al 2005)

    // TODO: For a convex polygon, the min-weight triangulation can be found in O(n^3) time using dynamic programming.
    // TODO: For a monotone polygon, it is even possible in O(n^2) time.

    todo!("mwt greedy");
}
