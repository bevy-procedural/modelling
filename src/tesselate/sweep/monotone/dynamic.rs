use super::{ChainDirection, MonotoneTriangulator};
use crate::{
    math::{HasZero, IndexType, Scalar, Vector2D},
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
        let mut vs = Vec::<(Self::Vec2, Self::V)>::new();
        for &v in self.left.iter() {
            vs.push((vec2s[v].vec, vec2s[v].index));
        }
        for &v in self.right.iter().rev() {
            vs.push((vec2s[v].vec, vec2s[v].index));
        }

        let n = vs.len();
        let mut M = vec![vec![Vec2::S::ZERO; n]; n];
        let mut S = vec![vec![0; n]; n];

        for l in 2..n {
            for i in 0..(n - l) {
                let j = i + l;
                M[i][j] = Vec2::S::INFINITY;
                for k in (i + 1)..j {
                    if !Self::is_valid_diagonal(i, k, &vs)
                        || !Self::is_valid_diagonal(k, j, &vs)
                        || !Self::is_valid_triangle(i, j, k, &vs)
                    {
                        continue;
                    }

                    let weight = triangle_weight(&vs[i].0, &vs[j].0, &vs[k].0);
                    let cost = M[i][k] + M[k][j] + weight;
                    if cost < M[i][j] {
                        M[i][j] = cost;
                        S[i][j] = k;
                    }
                }
            }
        }

        // In finish function, call:
        Self::traceback(0, n - 1, &S, indices, &vs);

        //ear_clipping_direct(&vs, indices, false);
    }
}

fn triangle_weight<Vec2: Vector2D>(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2::S {
    let ab = a.distance(b);
    let bc = b.distance(c);
    let ca = c.distance(a);
    ab + bc + ca
}

impl<V: IndexType, Vec2: Vector2D> DynamicMonoTriangulator<V, Vec2> {
    fn traceback(
        i: usize,
        j: usize,
        s: &Vec<Vec<usize>>,
        indices: &mut Triangulation<V>,
        vs: &Vec<(Vec2, V)>,
    ) {
        if j - i < 2 {
            return;
        }
        let k = s[i][j];
        // Add triangle (vi, vk, vj)
        indices.insert_triangle(vs[i].1, vs[k].1, vs[j].1);
        // Recurse on subpolygons
        Self::traceback(i, k, s, indices, vs);
        Self::traceback(k, j, s, indices, vs);
    }

    fn is_valid_diagonal(i: usize, j: usize, vs: &Vec<(Vec2, V)>) -> bool {
        // Implement validity check for diagonal (v_i, v_j)
        // For x-monotone polygons, check if the diagonal is inside the polygon
        // and does not intersect any edges.
        // Use properties of x-monotone polygons to optimize this check.
        // ...

        // TODO

        true
    }

    fn is_valid_triangle(i: usize, j: usize, k: usize, vs: &Vec<(Vec2, V)>) -> bool {
        // Check if triangle (v_i, v_j, v_k) is valid
        // Ensure no other vertex lies inside the triangle
        for m in (i + 1)..j {
            if m == k {
                continue;
            }
            if point_in_triangle(&vs[m].0, &vs[i].0, &vs[j].0, &vs[k].0) {
                return false;
            }
        }
        true
    }
}

pub fn point_in_triangle<Vec2: Vector2D>(p: &Vec2, a: &Vec2, b: &Vec2, c: &Vec2) -> bool {
    let area = Vec2::S::HALF
        * (-b.y() * c.x() + a.y() * (-b.x() + c.x()) + a.x() * (b.y() - c.y()) + b.x() * c.y());
    let s = Vec2::S::ONE / (Vec2::S::TWO * area)
        * (a.y() * c.x() - a.x() * c.y() + (c.y() - a.y()) * p.x() + (a.x() - c.x()) * p.y());
    let t = Vec2::S::ONE / (Vec2::S::TWO * area)
        * (a.x() * b.y() - a.y() * b.x() + (a.y() - b.y()) * p.x() + (b.x() - a.x()) * p.y());
    s > Vec2::S::ZERO && t > Vec2::S::ZERO && Vec2::S::ONE - s - t > Vec2::S::ZERO
}
