use super::point::IndexedVertexPoint;
use crate::{math::Scalar, math::Vector2D, representation::IndexType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepReflexChainDirection {
    /// The reflex chain is completely on the left
    Left,
    /// The reflex chain is completely on the right
    Right,
    /// The reflex chain consists of the first single item having no preference for a side or is empty
    None,
}

/// This structure stores the reflex chain of the untriangulated region above.
/// See https://www.cs.umd.edu/class/spring2020/cmsc754/Lects/lect05-triangulate.pdf
/// It preserves the following invariant:
/// for i>=2, let v_i be the triangle just processed by the algorithm. The untriangulated
/// region to the top of v_i consist of two y-monotone chains, a left and a right chain each containing
/// at least one edge. Only one of the two chains contains more than one edge. The chain with the single
/// edge has its bottom endpoint below the sweep line. Hence, we place the start vertex before the other
/// chain. The currently active chain is indicated by d.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepReflexChain<V: IndexType, Vec2: Vector2D> {
    stack: Vec<usize>,
    d: SweepReflexChainDirection,

    /// Bind the types to the chain. There is no need to mix the types and it simplifies the type signatures.
    phantom: std::marker::PhantomData<(V, Vec2)>,
}

impl<V: IndexType, Vec2: Vector2D> SweepReflexChain<V, Vec2> {
    /// Create an empty reflex chain
    pub fn new() -> Self {
        SweepReflexChain {
            stack: Vec::new(),
            d: SweepReflexChainDirection::None,
            phantom: std::marker::PhantomData,
        }
    }

    /// Get the direction of the chain
    pub fn direction(&self) -> SweepReflexChainDirection {
        self.d
    }

    /// Get the first element of the chain
    pub fn first(&self) -> usize {
        self.stack.first().unwrap().clone()
    }

    /// Create a new reflex chain with a single value
    pub fn single(v: usize) -> Self {
        SweepReflexChain {
            stack: vec![v],
            d: SweepReflexChainDirection::None,
            phantom: std::marker::PhantomData,
        }
    }

    fn add_same_direction(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<Vec2>>,
        d: SweepReflexChainDirection,
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
            if d == SweepReflexChainDirection::Left {
                if angle > Vec2::S::ZERO {
                    break;
                }
                indices.extend([
                    V::new(self.stack[l - 1]),
                    V::new(value),
                    V::new(self.stack[l - 2]),
                ]);
            } else {
                // right or no preference

                if angle < Vec2::S::ZERO {
                    break;
                }
                indices.extend([
                    V::new(self.stack[l - 1]),
                    V::new(self.stack[l - 2]),
                    V::new(value),
                ]);
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

    fn add_opposite_direction(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        d: SweepReflexChainDirection,
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
                if d == SweepReflexChainDirection::Left {
                    indices.extend([
                        V::new(self.stack[i - 1]),
                        V::new(value),
                        V::new(self.stack[i]),
                    ]);
                } else {
                    indices.extend([
                        V::new(self.stack[i - 1]),
                        V::new(self.stack[i]),
                        V::new(value),
                    ]);
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

    /// Add a new value to the left reflex chain
    pub fn add(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<Vec2>>,
        d: SweepReflexChainDirection,
    ) -> &Self {
        #[cfg(feature = "sweep_debug_print")]
        println!("left: {:?} {} {:?}", self.d, value, self.stack);
        if self.d == SweepReflexChainDirection::None {
            assert!(self.stack.len() <= 1);
            self.stack.push(value);
            self.d = d;
        } else if self.d == d {
            self.add_same_direction(value, indices, vec2s, d);
        } else {
            self.add_opposite_direction(value, indices, d);
        }

        self
    }

    /// Add a new value to the right reflex chain
    pub fn right(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<Vec2>>,
    ) -> &Self {
        self.add(value, indices, vec2s, SweepReflexChainDirection::Right)
    }

    /// Add a new value to the left reflex chain
    pub fn left(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<Vec2>>,
    ) -> &Self {
        self.add(value, indices, vec2s, SweepReflexChainDirection::Left)
    }

    pub fn is_done(&self) -> bool {
        self.stack.len() <= 2
    }
}
