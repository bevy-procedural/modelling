use crate::{
    math::{HasZero, Vector2D},
    mesh::{
        tesselate::{IndexedVertex2D, Triangulation},
        IndexType,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflexChainDirection {
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
pub struct ReflexChain<V: IndexType, Vec2: Vector2D> {
    stack: Vec<usize>,
    d: ReflexChainDirection,

    /// Bind the types to the chain. There is no need to mix the types and it simplifies the type signatures.
    phantom: std::marker::PhantomData<(V, Vec2)>,
}

impl<V: IndexType, Vec2: Vector2D> std::fmt::Display for ReflexChain<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.d, self.stack)
    }
}

impl<V: IndexType, Vec2: Vector2D> ReflexChain<V, Vec2> {
    /// Create an empty reflex chain
    pub fn new() -> Self {
        ReflexChain {
            stack: Vec::new(),
            d: ReflexChainDirection::None,
            phantom: std::marker::PhantomData,
        }
    }

    /// Get the direction of the chain
    pub fn direction(&self) -> ReflexChainDirection {
        self.d
    }

    /// Get the first element of the chain
    pub fn first(&self) -> usize {
        self.stack.first().unwrap().clone()
    }

    /// Get the last element of the chain
    pub fn last(&self) -> usize {
        self.stack.last().unwrap().clone()
    }

    /// Create a new reflex chain with a single value
    pub fn single(v: usize) -> Self {
        ReflexChain {
            stack: vec![v],
            d: ReflexChainDirection::None,
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    fn add_same_direction(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ReflexChainDirection,
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
            if d == ReflexChainDirection::Left {
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
        d: ReflexChainDirection,
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
                if d == ReflexChainDirection::Left {
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
    pub fn add(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
        d: ReflexChainDirection,
    ) -> &Self {
        #[cfg(feature = "sweep_debug_print")]
        println!("chain add: {:?} {} {:?}", self.d, value, self.stack);
        if self.d == ReflexChainDirection::None {
            assert!(self.stack.len() <= 1);
            self.stack.push(value);
            self.d = d;
        } else if self.d == d {
            self.add_same_direction(value, indices, vec2s, d);
        } else {
            self.add_opposite_direction(value, indices, vec2s, d);
        }

        self
    }

    /// Add a new value to the right reflex chain
    #[inline]
    pub fn right(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) -> &Self {
        self.add(value, indices, vec2s, ReflexChainDirection::Right)
    }

    /// Add a new value to the left reflex chain
    #[inline]
    pub fn left(
        &mut self,
        value: usize,
        indices: &mut Triangulation<V>,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) -> &Self {
        self.add(value, indices, vec2s, ReflexChainDirection::Left)
    }

    pub fn is_done(&self) -> bool {
        self.stack.len() <= 2
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
