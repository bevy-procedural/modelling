use super::{ChainDirection, MonotoneTriangulator};
use crate::{
    math::{IndexType, Vector2D},
    mesh::{IndexedVertex2D, Triangulation},
    tesselate::ear_clipping_direct,
};

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

#[derive(Clone, Debug)]
pub struct DynamicMonoTriangulator<V: IndexType, Vec2: Vector2D> {
    left: Vec<usize>,
    right: Vec<usize>,
    d: ChainDirection,
    last: usize,

    /// Bind the types to the chain. There is no need to mix the types and it simplifies the type signatures.
    phantom: std::marker::PhantomData<(V, Vec2)>,
}

impl<V: IndexType, Vec2: Vector2D> MonotoneTriangulator for DynamicMonoTriangulator<V, Vec2> {
    type V = V;
    type Vec2 = Vec2;

    /// Create a new reflex chain with a single value
    fn new(v: usize) -> Self {
        DynamicMonoTriangulator {
            left: vec![v],
            right: vec![],
            d: ChainDirection::None,
            last: v,
            phantom: std::marker::PhantomData,
        }
    }

    /// Get the first element of the chain (the last inserted vertex)
    fn first(&self) -> usize {
        self.last
    }

    /// Whether the chain is oriented to the right
    fn is_right(&self) -> bool {
        self.d == ChainDirection::Right
    }

    fn sanity_check(&self, _: usize, _: usize, _: &Option<Self>) {
        // fine
    }

    #[inline]
    fn right(&mut self, value: usize, _: &mut Triangulation<V>, _: &Vec<IndexedVertex2D<V, Vec2>>) {
        self.right.push(value);
        self.last = value;
        self.d = ChainDirection::Right;
    }

    /// Add a new value to the left reflex chain
    #[inline]
    fn left(&mut self, value: usize, _: &mut Triangulation<V>, _: &Vec<IndexedVertex2D<V, Vec2>>) {
        self.left.push(value);
        self.last = value;
        self.d = ChainDirection::Left;
    }

    /// Finish triangulating the reflex chain
    fn finish(&mut self, indices: &mut Triangulation<V>, vec2s: &Vec<IndexedVertex2D<V, Vec2>>) {
        /*
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

        let mut vs = Vec::<(Self::Vec2, Self::V)>::new();
        for &v in self.left.iter() {
            vs.push((vec2s[v].vec, vec2s[v].index));
        }
        for &v in self.right.iter().rev() {
            vs.push((vec2s[v].vec, vec2s[v].index));
        }
        ear_clipping_direct(&vs, indices, false);
    }
}
