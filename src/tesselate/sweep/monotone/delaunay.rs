use std::collections::{HashMap, HashSet};

use super::{ChainDirection, MonotoneTriangulator};
use crate::{
    math::{IndexType, Scalar, Vector2D},
    mesh::{IndexedVertex2D, Triangulation},
};

fn determinant_4x4<S: Scalar>(matrix: [[S; 4]; 4]) -> S {
    let mut det = S::zero();

    for i in 0..4 {
        let mut submatrix = [[S::zero(); 3]; 3];
        for row in 1..4 {
            let mut col_index = 0;
            for col in 0..4 {
                if col == i {
                    continue;
                }
                submatrix[row - 1][col_index] = matrix[row][col];
                col_index += 1;
            }
        }
        let sign = if i % 2 == 0 { S::ONE } else { -S::ONE };
        det += sign * matrix[0][i] * determinant_3x3(submatrix);
    }

    det
}

fn determinant_3x3<S: Scalar>(matrix: [[S; 3]; 3]) -> S {
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];

    let det = a * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
        - b * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
        + c * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);

    det
}

// disable automatic formatting:
#[rustfmt::skip]
fn circumcircle_contains<Vec2: Vector2D>(p1: &Vec2, p2: &Vec2, p3: &Vec2, p: &Vec2) -> bool

{
    let x1 = p1.x(); let y1 = p1.y();
    let x2 = p2.x(); let y2 = p2.y();
    let x3 = p3.x(); let y3 = p3.y();
    let xp = p.x();  let yp = p.y();

    // Matrix determinant form:
    // |x1 y1 x1²+y1² 1|
    // |x2 y2 x2²+y2² 1|
    // |x3 y3 x3²+y3² 1|
    // |xp yp xp²+yp² 1|
    let m = [
        [x1, y1, x1*x1+y1*y1, Vec2::S::ONE],
        [x2, y2, x2*x2+y2*y2, Vec2::S::ONE],
        [x3, y3, x3*x3+y3*y3, Vec2::S::ONE],
        [xp, yp, xp*xp+yp*yp, Vec2::S::ONE]
    ];

    // The determinant of the matrix is positive if the point is inside the circumcircle.
    // m.determinant().is_positive()
    // we cannot use `determinant` because it requires `ScalarPlus`
    determinant_4x4(m).is_positive()
}

/// A monotone triangulator that tries to build triangles that are locally Delaunay by
/// checking the inCircle property. When adding a new vertex, it attempts to form
/// diagonals that do not violate the Delaunay condition.
#[derive(Clone)]
pub struct DelaunayMonoTriangulator<V: IndexType, Vec2: Vector2D> {
    stack: Vec<usize>,
    d: ChainDirection,

    /// Edges that are constrained and should not be flipped.
    constrained_edges: HashSet<(usize, usize)>,

    /// Opposite vertex across an edge, assuming triangles are formed in a CCW order.
    /// Also the index of the triangle that contains the edge.
    opposite_vertex: HashMap<(usize, usize), (usize, usize)>,

    last_left: Option<usize>,
    last_right: Option<usize>,

    phantom: std::marker::PhantomData<(V, Vec2)>,
}

impl<V: IndexType, Vec2: Vector2D> std::fmt::Debug for DelaunayMonoTriangulator<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.d, self.stack)
    }
}

impl<V: IndexType, Vec2: Vector2D> DelaunayMonoTriangulator<V, Vec2> {
    fn direction(&self) -> ChainDirection {
        self.d
    }

    fn last(&self) -> usize {
        *self.stack.last().unwrap()
    }

    fn first(&self) -> usize {
        *self.stack.first().unwrap()
    }

    fn is_done(&self) -> bool {
        self.stack.len() <= 2
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn stack_edge(
        &self,
        seen: &mut HashSet<(usize, usize)>,
        stack: &mut Vec<(usize, usize)>,
        a: usize,
        b: usize,
    ) {
        assert!(a != b);
        let k = (a.min(b), a.max(b));
        if seen.contains(&k) {
            return;
        }
        if self.is_constrained_edge(a, b) {
            return; // Cannot flip a constrained edge
        }
        seen.insert(k);
        stack.push((a, b));
    }

    /// After inserting a triangle, we attempt to "legalize" newly formed edges
    /// according to the Delaunay criterion. This method:
    /// 1. Identifies the new edges introduced.
    /// 2. Checks if any of them need to be flipped.
    /// 3. Repeats until no more flips are needed.
    fn legalize_triangle(
        &mut self,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        new_triangle: [usize; 3],
    ) {
        // TODO: in theory we shouldn't need `seen`
        let mut seen = HashSet::new();
        let mut stack = Vec::new();

        // For each edge of the new triangle, try to legalize it:
        for i in 0..3 {
            let a = new_triangle[i];
            let b: usize = new_triangle[(i + 1) % 3];
            self.stack_edge(&mut seen, &mut stack, a, b);
        }

        loop {
            let Some((a, b)) = stack.pop() else {
                break;
            };
            if let Some((a, b, c, d)) = self.legalize_edge(indices, vec2s, a, b) {
                // TODO: Do wee need all five edges here?
                self.stack_edge(&mut seen, &mut stack, c, d);
                self.stack_edge(&mut seen, &mut stack, a, d);
                self.stack_edge(&mut seen, &mut stack, b, c);
                self.stack_edge(&mut seen, &mut stack, b, d);
                self.stack_edge(&mut seen, &mut stack, a, c);
            }
        }
    }

    /// Check if an edge is constrained.
    #[inline]
    fn is_constrained_edge(&self, a: usize, b: usize) -> bool {
        self.constrained_edges.contains(&(a.min(b), a.max(b)))
    }

    /// Mark an edge as constrained.
    #[inline]
    fn constrain_edge(&mut self, a: usize, b: usize) {
        self.constrained_edges.insert((a.min(b), a.max(b)));
    }

    /// Get the opposite vertex and adjacent triangle across an edge.
    fn opposite_vertex_of_edge(&self, a: usize, b: usize) -> Option<(usize, usize)> {
        self.opposite_vertex.get(&(a, b)).map(|&v| v)
    }

    /// Legalize an edge if possible. This involves:
    /// - Checking if the edge is internal and not constrained.
    /// - Finding the opposite vertex in the adjacent triangle.
    /// - Checking the circumcircle criterion.
    /// - Flipping the edge if it violates the Delaunay property.
    ///
    /// After flipping, the newly formed edges around the flipped edge must also be checked.
    fn legalize_edge(
        &mut self,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        a: usize,
        b: usize,
    ) -> Option<(usize, usize, usize, usize)> {
        //     c/p3
        //    /    \       triangle_ab
        // a/p1 --- b/p2
        //    \    /       triangle_ba
        //     d/p_test

        if let Some((c, triangle_ab)) = self.opposite_vertex_of_edge(a, b) {
            // Check Delaunay condition:
            let p1 = vec2s[a].vec;
            let p2 = vec2s[b].vec;
            let p3 = vec2s[c].vec;

            // The current triangle sharing edge (a,b) is already known from insertion.
            // We must find the vertex opposite in the "current" triangle formed.
            if let Some((d, triangle_ba)) = self.opposite_vertex_of_edge(b, a) {
                // Now we have a quadrilateral formed by (a,b,c,d).
                // Check if `d` lies inside the circumcircle of the triangle formed by (a,b,c).

                let p_test = vec2s[d].vec;

                // check whether the diagonal (p3, p_test) is valid
                if !p1.convex(p3, p_test) || !p2.convex(p_test, p3) {
                    return None;
                }

                if circumcircle_contains(&p1, &p2, &p3, &p_test) {
                    // Edge (a,b) is not Delaunay. Flip it.
                    // After flipping, we must re-legalize the edges that were affected.
                    assert!(indices
                        .flip_edge(vec2s[a].index, vec2s[b].index, triangle_ab, triangle_ba)
                        .is_ok());

                    // update the opposite edges
                    self.opposite_vertex.remove(&(a, b));
                    self.opposite_vertex.remove(&(b, a));
                    self.opposite_vertex.insert((a, d), (c, triangle_ab));
                    self.opposite_vertex.insert((d, c), (a, triangle_ab));
                    self.opposite_vertex.insert((c, a), (d, triangle_ab));
                    self.opposite_vertex.insert((b, c), (d, triangle_ba));
                    self.opposite_vertex.insert((c, d), (b, triangle_ba));
                    self.opposite_vertex.insert((d, b), (c, triangle_ba));

                    // The flip replaces edge (a,b) with (c, d).
                    // Now legalize the edges around the newly formed diagonals:
                    return Some((a, b, c, d));
                }
            }
        }
        return None;
    }

    /// Insert a triangle formed by three vertices and mark the opposite vertex across each edge.
    fn insert_triangle_local(
        &mut self,
        a: usize,
        b: usize,
        c: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        let index_offset = indices.next_pos();
        indices.insert_triangle_local(a, b, c, vec2s);
        debug_assert!(self.opposite_vertex.get(&(a, b)).is_none());
        debug_assert!(self.opposite_vertex.get(&(b, c)).is_none());
        debug_assert!(self.opposite_vertex.get(&(c, a)).is_none());
        self.opposite_vertex.insert((a, b), (c, index_offset));
        self.opposite_vertex.insert((b, c), (a, index_offset));
        self.opposite_vertex.insert((c, a), (b, index_offset));
    }

    /// Similar to the linear approach, but after forming a visible triangle, we check and fix edges.
    fn add_same_direction(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) {
        assert!(self.stack.len() >= 1);

        loop {
            let l = self.stack.len();
            if l <= 1 {
                break;
            }

            let angle = vec2s[value]
                .vec
                .angle_tri(vec2s[self.stack[l - 1]].vec, vec2s[self.stack[l - 2]].vec);

            if d == ChainDirection::Left {
                if angle > Vec2::S::ZERO {
                    break;
                }
                // Insert triangle: (stack[l-2], stack[l-1], value)
                self.insert_triangle_local(
                    self.stack[l - 1],
                    value,
                    self.stack[l - 2],
                    indices,
                    vec2s,
                );

                self.legalize_triangle(
                    indices,
                    vec2s,
                    [self.stack[l - 2], self.stack[l - 1], value],
                );
            } else {
                if angle < Vec2::S::ZERO {
                    break;
                }
                self.insert_triangle_local(
                    self.stack[l - 1],
                    self.stack[l - 2],
                    value,
                    indices,
                    vec2s,
                );

                self.legalize_triangle(
                    indices,
                    vec2s,
                    [self.stack[l - 2], self.stack[l - 1], value],
                );
            }

            self.stack.pop();
        }

        self.stack.push(value);
    }

    fn add_opposite_direction(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) {
        assert!(self.d != d);
        assert!(self.stack.len() >= 1);

        if self.stack.len() == 1 {
            self.stack.push(value);
            self.d = d;
        } else {
            // Triangulate the current chain completely:
            for i in 1..self.stack.len() {
                if d == ChainDirection::Left {
                    self.insert_triangle_local(
                        self.stack[i - 1],
                        value,
                        self.stack[i],
                        indices,
                        vec2s,
                    );
                    self.legalize_triangle(
                        indices,
                        vec2s,
                        [self.stack[i - 1], self.stack[i], value],
                    );
                } else {
                    self.insert_triangle_local(
                        self.stack[i - 1],
                        self.stack[i],
                        value,
                        indices,
                        vec2s,
                    );
                    self.legalize_triangle(
                        indices,
                        vec2s,
                        [self.stack[i - 1], self.stack[i], value],
                    );
                }
            }

            let last = self.stack.pop().unwrap();
            self.stack.clear();
            self.stack.push(last);
            self.stack.push(value);
            self.d = d;
        }
    }

    fn add(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) -> &Self {
        if self.d == ChainDirection::None {
            assert!(self.stack.len() == 1);
            self.stack.push(value);
            self.d = d;

            // we don't Delaunay outside of the monotone region
            self.constrain_edge(self.stack[0], value);
        } else if self.d == d {
            // we constrain the next edge on the same chain
            self.constrain_edge(self.last(), value);
            self.add_same_direction(value, indices, vec2s, d);
        } else {
            // we constrain the next edge on the opposite chain
            self.constrain_edge(self.first(), value);
            self.add_opposite_direction(value, indices, vec2s, d);
        }

        self
    }
}

impl<V: IndexType, Vec2: Vector2D> MonotoneTriangulator for DelaunayMonoTriangulator<V, Vec2> {
    type V = V;
    type Vec2 = Vec2;

    fn new(v: usize) -> Self {
        DelaunayMonoTriangulator {
            stack: vec![v],
            d: ChainDirection::None,
            phantom: std::marker::PhantomData,
            constrained_edges: HashSet::new(),
            opposite_vertex: HashMap::new(),
            last_left: Some(v),
            last_right: None,
        }
    }

    fn last_opposite(&self) -> usize {
        let res = if self.d == ChainDirection::None {
            assert!(self.last_right.is_none());
            self.last_left.unwrap()
        } else if self.d == ChainDirection::Right {
            self.last_left.unwrap()
        } else {
            self.last_right.unwrap()
        };

        assert!(res == self.first());
        res
    }

    fn is_right(&self) -> bool {
        self.direction() == ChainDirection::Right
    }

    fn sanity_check(&self, left_start: usize, right_start: usize, fixup: &Option<Self>) {
        match self.direction() {
            ChainDirection::None => {
                assert!(self.len() == 1);
                assert_eq!(left_start, self.first());
                assert_eq!(right_start, self.first());
            }
            ChainDirection::Left => {
                assert!(self.len() >= 2);
                if let Some(fixup) = fixup {
                    assert!(fixup.len() >= 2);
                    assert_eq!(right_start, self.first());
                    assert_eq!(left_start, fixup.first());
                } else {
                    assert_eq!(right_start, self.first());
                    assert_eq!(left_start, self.last());
                }
            }
            ChainDirection::Right => {
                assert!(self.len() >= 2);
                if let Some(fixup) = fixup {
                    assert!(fixup.len() >= 2);
                    assert_eq!(left_start, self.first());
                    assert_eq!(right_start, fixup.first());
                } else {
                    assert_eq!(left_start, self.first());
                    assert_eq!(right_start, self.last());
                }
            }
        };
    }

    fn right(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        self.last_right = Some(value);
        self.add(value, indices, vec2s, ChainDirection::Right);
    }

    fn left(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        self.last_left = Some(value);
        self.add(value, indices, vec2s, ChainDirection::Left);
    }

    fn finish(&mut self, _indices: &mut Triangulation<V>, _vec2s: &Vec<IndexedVertex2D<V, Vec2>>) {
        // By the time we're finishing, all triangles should be formed and edges legalized.
        // The Delaunay process was done incrementally as edges were formed.
        assert!(self.is_done());
    }
}
