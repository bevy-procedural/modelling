use super::point::IndexedVertexPoint;
use crate::{
    math::{Scalar, Vector2D},
    representation::{payload::Payload, IndexType, Mesh},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SweepReflexChainDirection {
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
pub struct SweepReflexChain<V: IndexType> {
    stack: Vec<V>,
    d: SweepReflexChainDirection,
}

impl<V: IndexType> SweepReflexChain<V> {
    pub fn new() -> Self {
        SweepReflexChain {
            stack: Vec::new(),
            d: SweepReflexChainDirection::None,
        }
    }

    pub fn first(v: V) -> Self {
        SweepReflexChain {
            stack: vec![v],
            d: SweepReflexChainDirection::None,
        }
    }

    /// Add a new value to the left reflex chain
    pub fn left<P: Payload>(
        &mut self,
        value: V,
        indices: &mut Vec<V>,
        vec2s: &HashMap<V, IndexedVertexPoint<V, P::Vec2, P::S>>,
    ) -> Self {
        println!("left: {:?} {} {:?}", self.d, value, self.stack);
        match self.d {
            SweepReflexChainDirection::None => {
                assert!(self.stack.len() <= 1);
                self.stack.push(value);
                self.d = SweepReflexChainDirection::Left;
            }
            SweepReflexChainDirection::Left => {
                assert!(self.stack.len() >= 1);

                // draw triangles while they are visible
                loop {
                    let l = self.stack.len();
                    if l <= 1 {
                        break;
                    }
                    let angle = vec2s[&value]
                        .vec
                        .angle(vec2s[&self.stack[l - 1]].vec, vec2s[&self.stack[l - 2]].vec);
                    if angle > P::S::ZERO {
                        break;
                    }
                    println!(
                        "create vis l: {:?}",
                        [self.stack[l - 1], self.stack[l - 2], value]
                    );
                    indices.extend([self.stack[l - 1], value, self.stack[l - 2]]);
                    self.stack.pop();
                }

                // remember on more for the same direction
                self.stack.push(value);
            }
            SweepReflexChainDirection::Right => {
                assert!(self.stack.len() >= 1);
                // place the next triangle!
                if self.stack.len() == 1 {
                    self.stack.push(value);
                    self.d = SweepReflexChainDirection::Left;
                } else {
                    // there is enough on the stack to consume
                    for i in 1..self.stack.len() {
                        indices.extend([self.stack[i - 1], value, self.stack[i]]);
                        println!(
                            "create mul l: {:?}",
                            [self.stack[i - 1], self.stack[i], value]
                        );
                    }
                    let last = self.stack.pop().unwrap();
                    self.stack.clear();
                    self.stack.push(last);
                    self.stack.push(value);
                    self.d = SweepReflexChainDirection::Left;
                }
            }
        }
        self.clone()
    }

    /// Add a new value to the right reflex chain
    pub fn right<P: Payload>(
        &mut self,
        value: V,
        indices: &mut Vec<V>,
        vec2s: &HashMap<V, IndexedVertexPoint<V, P::Vec2, P::S>>,
    ) -> Self {
        println!("right: {:?} {} {:?}", self.d, value, self.stack);
        match self.d {
            SweepReflexChainDirection::None => {
                assert!(self.stack.len() <= 1);
                self.stack.push(value);
                self.d = SweepReflexChainDirection::Right;
            }
            SweepReflexChainDirection::Right => {
                assert!(self.stack.len() >= 1);

                // draw triangles while they are visible
                loop {
                    let l = self.stack.len();
                    if l <= 1 {
                        break;
                    }
                    let angle = vec2s[&value]
                        .vec
                        .angle(vec2s[&self.stack[l - 1]].vec, vec2s[&self.stack[l - 2]].vec);
                    if angle < P::S::ZERO {
                        break;
                    }
                    println!(
                        "create vis r: {:?}",
                        [self.stack[l - 1], self.stack[l - 2], value]
                    );
                    indices.extend([self.stack[l - 1], self.stack[l - 2], value]);
                    self.stack.pop();
                }

                // remember on more for the same direction
                self.stack.push(value);
            }
            SweepReflexChainDirection::Left => {
                assert!(self.stack.len() >= 1);
                // place the next triangle!
                if self.stack.len() == 1 {
                    self.stack.push(value);
                    self.d = SweepReflexChainDirection::Right;
                } else {
                    // there is enough on the stack to consume
                    for i in 1..self.stack.len() {
                        indices.extend([self.stack[i - 1], self.stack[i], value]);
                        println!(
                            "create mul r: {:?}",
                            [self.stack[i - 1], self.stack[i], value]
                        );
                    }
                    let last = self.stack.pop().unwrap();
                    self.stack.clear();
                    self.stack.push(last);
                    self.stack.push(value);
                    self.d = SweepReflexChainDirection::Right;
                }
            }
        }
        self.clone()
    }

    pub fn is_done(&self) -> bool {
        self.stack.len() <= 2
        //&& self.d != SweepReflexChainDirection::Left
        // && self.d != SweepReflexChainDirection::Right
    }
}
