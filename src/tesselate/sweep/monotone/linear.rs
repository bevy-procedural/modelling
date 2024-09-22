use super::{ChainDirection, MonotoneTriangulator};
use crate::{
    math::{HasZero, IndexType, Vector2D},
    mesh::{IndexedVertex2D, Triangulation},
};

/// This structure stores the reflex chain of the untriangulated region above.
/// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf
/// It preserves the following invariant:
/// for i>=2, let v_i be the triangle just processed by the algorithm. The untriangulated
/// region to the top of v_i consist of two y-monotone chains, a left and a right chain each containing
/// at least one edge. Only one of the two chains contains more than one edge. The chain with the single
/// edge has its bottom endpoint below the sweep line. Hence, we place the start vertex before the other
/// chain. The currently active chain is indicated by d.
#[derive(Clone)]
pub struct LinearMonoTriangulator<V: IndexType, Vec2: Vector2D> {
    stack: Vec<usize>,
    d: ChainDirection,

    /// Bind the types to the chain. There is no need to mix the types and it simplifies the type signatures.
    phantom: std::marker::PhantomData<(V, Vec2)>,
}

impl<V: IndexType, Vec2: Vector2D> std::fmt::Debug for LinearMonoTriangulator<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.d, self.stack)
    }
}

impl<V: IndexType, Vec2: Vector2D> LinearMonoTriangulator<V, Vec2> {
    /// Get the direction of the chain
    fn direction(&self) -> ChainDirection {
        self.d
    }

    /// Get the last element of the chain
    fn last(&self) -> usize {
        self.stack.last().unwrap().clone()
    }

    #[inline]
    fn add_same_direction(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) {
        assert!(self.stack.len() >= 1);
        // TODO: assert for direction not none?

        // draw triangles while they are visible
        loop {
            let l = self.stack.len();
            if l <= 1 {
                break;
            }
            let angle = vec2s[value]
                .vec
                .angle(vec2s[self.stack[l - 1]].vec, vec2s[self.stack[l - 2]].vec);
            if d == ChainDirection::Left {
                if angle > Vec2::S::ZERO {
                    break;
                }
                indices.insert_triangle_local(self.stack[l - 1], value, self.stack[l - 2], vec2s);
            } else {
                // right or no preference

                if angle < Vec2::S::ZERO {
                    break;
                }
                indices.insert_triangle_local(self.stack[l - 1], self.stack[l - 2], value, vec2s);
            }

            #[cfg(feature = "sweep_debug_print")]
            println!(
                "create vis: {:?}",
                [self.stack[l - 1], self.stack[l - 2], value]
            );

            self.stack.pop();
        }

        // remember on more for the same direction
        self.stack.push(value);
    }

    #[inline]
    fn add_opposite_direction(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) {
        assert!(self.d != d);
        // TODO: assert for direction not none?
        assert!(self.stack.len() >= 1);
        // place the next triangle!
        if self.stack.len() == 1 {
            self.stack.push(value);
            self.d = d;
        } else {
            // there is enough on the stack to consume
            for i in 1..self.stack.len() {
                if d == ChainDirection::Left {
                    indices.insert_triangle_local(self.stack[i - 1], value, self.stack[i], vec2s);
                } else {
                    indices.insert_triangle_local(self.stack[i - 1], self.stack[i], value, vec2s);
                }

                #[cfg(feature = "sweep_debug_print")]
                println!(
                    "create mul l: {:?}",
                    [self.stack[i - 1], self.stack[i], value]
                );
            }
            let last = self.stack.pop().unwrap();
            self.stack.clear();
            self.stack.push(last);
            self.stack.push(value);
            self.d = d;
        }
    }

    /// Add a new value to the reflex chain
    #[inline]
    fn add(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ChainDirection,
    ) -> &Self {
        #[cfg(feature = "sweep_debug_print")]
        println!("chain add: {:?} {} {:?}", self.d, value, self.stack);
        if self.d == ChainDirection::None {
            assert!(self.stack.len() <= 1);
            self.stack.push(value);
            self.d = d;
        } else if self.d == d {
            self.add_same_direction(value, indices, vec2s, d);
        } else {
            self.add_opposite_direction(value, indices, vec2s, d);
        }

        assert!(self.d == d);
        self
    }

    /// Get the length of the reflex chain
    fn len(&self) -> usize {
        self.stack.len()
    }

    /// Check whether the reflex chain is done, i.e., everything is already triangulated
    fn is_done(&self) -> bool {
        self.stack.len() <= 2
    }
}

impl<V: IndexType, Vec2: Vector2D> MonotoneTriangulator for LinearMonoTriangulator<V, Vec2> {
    type V = V;
    type Vec2 = Vec2;

    /// Create a new reflex chain with a single value
    fn new(v: usize) -> Self {
        LinearMonoTriangulator {
            stack: vec![v],
            d: ChainDirection::None,
            phantom: std::marker::PhantomData,
        }
    }

    /// Get the first element of the chain (the last inserted vertex)
    fn first(&self) -> usize {
        self.stack.first().unwrap().clone()
    }

    /// Whether the chain is oriented to the right
    fn is_right(&self) -> bool {
        self.direction() == ChainDirection::Right
    }

    /// Validate the reflex chain
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

    /// Add a new value to the right reflex chain
    #[inline]
    fn right(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        self.add(value, indices, vec2s, ChainDirection::Right);
    }

    /// Add a new value to the left reflex chain
    #[inline]
    fn left(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) {
        self.add(value, indices, vec2s, ChainDirection::Left);
    }

    /// Finish triangulating the reflex chain
    fn finish(&mut self, _indices: &mut Triangulation<V>, _vec2s: &Vec<IndexedVertex2D<V, Vec2>>) {
        // the linear triangulator does not need to finish the triangulation
        // - all work must be finished when adding the vertices
        assert!(self.is_done());
    }
}
