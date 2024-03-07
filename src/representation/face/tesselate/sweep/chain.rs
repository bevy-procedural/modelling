use super::point::IndexedVertexPoint;
use crate::{
    math::{Scalar, Vector2D},
    representation::{payload::Payload, IndexType},
};

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
pub struct SweepReflexChain {
    stack: Vec<usize>,
    d: SweepReflexChainDirection,
}

impl SweepReflexChain {
    pub fn new() -> Self {
        SweepReflexChain {
            stack: Vec::new(),
            d: SweepReflexChainDirection::None,
        }
    }

    pub fn direction(&self) -> SweepReflexChainDirection {
        self.d
    }

    pub fn first(&self) -> usize {
        self.stack.first().unwrap().clone()
    }

    pub fn single(v: usize) -> Self {
        SweepReflexChain {
            stack: vec![v],
            d: SweepReflexChainDirection::None,
        }
    }

    /// Add a new value to the left reflex chain
    pub fn add<V: IndexType, P: Payload>(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<P::Vec2, P::S>>,
        d: SweepReflexChainDirection,
    ) -> &Self {
        #[cfg(feature = "sweep_debug_print")]
        println!("left: {:?} {} {:?}", self.d, value, self.stack);
        if self.d == SweepReflexChainDirection::None {
            assert!(self.stack.len() <= 1);
            self.stack.push(value);
            self.d = d;
        } else if self.d == d {
            assert!(self.stack.len() >= 1);

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
                    if angle > P::S::ZERO {
                        break;
                    }
                    indices.extend([
                        V::new(self.stack[l - 1]),
                        V::new(value),
                        V::new(self.stack[l - 2]),
                    ]);
                } else {
                    if angle < P::S::ZERO {
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
        } else {
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

        self
    }

    /// Add a new value to the right reflex chain
    pub fn right<V: IndexType, P: Payload>(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<P::Vec2, P::S>>,
    ) -> &Self {
        self.add::<V, P>(value, indices, vec2s, SweepReflexChainDirection::Right)
    }

    /// Add a new value to the left reflex chain
    pub fn left<V: IndexType, P: Payload>(
        &mut self,
        value: usize,
        indices: &mut Vec<V>,
        vec2s: &Vec<IndexedVertexPoint<P::Vec2, P::S>>,
    ) -> &Self {
        self.add::<V, P>(value, indices, vec2s, SweepReflexChainDirection::Left)
    }

    pub fn is_done(&self) -> bool {
        self.stack.len() <= 2
    }
}
