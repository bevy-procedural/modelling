mod chain;
mod interval;
mod monotone;
mod point;
mod status;
mod sweep;
mod vertex_type;

use monotone::LinearMonoTriangulator;
pub use sweep::sweep_line_triangulation;
pub use vertex_type::VertexType;

use super::TesselationMeta;
use crate::{
    math::{HasPosition, IndexType, Vector3D},
    mesh::{Face3d, FaceBasics, IndexedVertex2D, MeshType, Triangulation},
};

/// Meta information for debuggin the sweep algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct SweepMeta<V: IndexType> {
    #[cfg(feature = "sweep_debug")]
    /// The type of the vertex in the reflex chain
    pub vertex_type: Vec<(V, VertexType)>,

    phantom: std::marker::PhantomData<V>,
}

impl<V: IndexType> Default for SweepMeta<V> {
    fn default() -> Self {
        SweepMeta {
            #[cfg(feature = "sweep_debug")]
            vertex_type: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V: IndexType> SweepMeta<V> {
    /// Update the type of a vertex
    #[cfg(feature = "sweep_debug")]
    pub fn update_type(&mut self, i: V, t: VertexType) {
        // TODO: Not efficient
        for (j, ty) in self.vertex_type.iter_mut() {
            if *j == i {
                *ty = t;
            }
        }
    }
}

/// Uses the sweep line triangulation
pub fn sweep_line<T: MeshType>(
    face: &T::Face,
    mesh: &T::Mesh,
    indices: &mut Triangulation<T::V>,
    meta: &mut TesselationMeta<T::V>,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    debug_assert!(face.may_be_curved() || face.is_planar2(mesh));

    // TODO: Improve performance by directly using the nd-vertices instead of converting to 2d
    let vec2s: Vec<_> = face
        .vertices_2d(mesh)
        .map(|(p, i)| IndexedVertex2D::<T::V, T::Vec2>::new(p, i))
        .collect();

    sweep_line_triangulation::<LinearMonoTriangulator<T::V, T::Vec2>>(
        indices,
        &vec2s,
        &mut meta.sweep,
    );
}

/// A variant of the sweep-line algorithm that finds the min-weight triangulation for each
/// monotone sub-polygon using dynamic programming, leading to an overall O(n^2) time complexity.
///
/// When using the bound k, the approximation quality decreases the smaller k is, with time O(k^2 n log n).
/// However, for k << n this comes in most cases very quickly close to O(n log n).
///
/// For the quality of the approximation it is generally beneficial to rotate the mesh
/// such that the mesh can be decomposed in a large number of y-monotone components.
pub fn sweep_dynamic<T: MeshType>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
    _k: usize,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    // TODO: Use the fact that we can find the min-weight triangulation of a x-monotone polygon in O(n^2) time using dynamic programming.
    // Basically, just run the sweep algorithm but replace the `ReflexChain` insertion step with a dynamic programming step.

    // Using k we limit the amount of edges to consider in the dynamic programming step, leading to k^2 during the chain insertion step instead of n^2.
    // This is called strip-based triangulation. We should chose the boundaries of the strips using some clever heuristic,
    // maybe based on density. We could also use orthogonal strips if the chains are very far away, i.e., cut the euclidean plane
    // into squares with each around k vertices inside and run the algorithm within each square. Because we still need vertices from both sides,
    // we could include a single vertex from the opposite chain effectively separating this into large triangles that are then triangulated each.
    // This is probably an important optimization since dense but far-away chains are a common scenario if we triangulates faces that consist
    // of simple but high-resolution geometry (e.g., a polygon-approximation of a circle). That would also be a point where we can easily insert
    // additional vertices significantly reducing edge lengths of the result.

    // The k-mechanism should also be available independent of the heuristic that is run in the end.

    /*
    ChatGPT says:

    Enumerate Subpolygons:

        Consider all possible subpolygons formed by vertices vi​ to vj​, where i<j.
        Due to the x-monotonicity, these subpolygons are themselves x-monotone and simple.

    Dynamic Programming Table:

        Create a table M[i][j] that stores the minimum weight triangulation cost for the subpolygon from vi​ to vj.
        Initialize the table for base cases where j−i ≤ 2 (triangles and edges).

    Recurrence Relation:

        For each subpolygon from vi​ to vj​, compute:
        M[i][j] = min {⁡i<k<j} (M[i][k] + M[k][j] + weight(vi,vj,vk))
        Here, weight(vi,vj,vk) is the sum of the edge lengths of triangle △vivjvk.

        Note: also make sure that there are no boundary intersections. This should be
        somewhat fast since we only have to check whether there is no larger y component before that.

    Order of Computation:

        Process the table in order of increasing subpolygon size to ensure that smaller subproblems are solved before larger ones.

    Result Retrieval:

        The minimum weight triangulation cost for the entire polygon is stored in M[1][n].
        */

    // To improve speed of the algorithm, we could use some pruning techniques and lazy evaluation.
    // Also, we could heuristically assume that that triangles that span a large range are not worth exploring.

    todo!("sweep dynamic");
}

/// A variant of the sweep-line algorithm that greedily approximates the min-weight triangulation for each
/// monotone sub-polygon, leading to an overall O(n log n) time complexity.
pub fn sweep_greedy<T: MeshType>(
    _face: &T::Face,
    _mesh: &T::Mesh,
    _indices: &mut Triangulation<T::V>,
    _k: usize,
) where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
    T::Face: Face3d<T>,
{
    // TODO: Use the fact that we can greedily approximate the min-weight triangulation of a x-monotone polygon in O(n log n) time:

    /*
    ChatGPT says:

    Preprocessing:
        Identify the Upper and Lower Chains:
            Since the polygon is x-monotone, the vertices are already sorted by x-coordinate.
            Label each vertex as belonging to either the upper or lower chain.
        Initialize a Stack:
            Start with the first two vertices on the combined chain.

    Iterative Process:
        For each vertex vi​ from the third to the last:
            If vi​ and the top of the stack are on different chains:
                Pop vertices from the stack, creating diagonals to vi, until only one vertex remains.
                Push vi​ onto the stack.
            If vi and the top of the stack are on the same chain:
                While the angle formed is convex, pop vertices from the stack and create diagonals to vi.
                Push vi onto the stack.

    Termination:
        Continue until all vertices are processed.
        The diagonals formed during this process constitute the triangulation.
    */

    todo!("sweep greedy");
}
