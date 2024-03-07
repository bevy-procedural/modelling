use super::point::IndexedVertexPoint;
use crate::{
    math::{Scalar, Vector2D},
    representation::{payload::Payload, IndexType, Mesh},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeData<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub start: IndexedVertexPoint<V, Vec2, S>,
    pub end: IndexedVertexPoint<V, Vec2, S>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> EdgeData<V, Vec2, S> {
    pub fn x_at_y(&self, y: S) -> S {
        let dx = self.end.vec.x() - self.start.vec.x();
        let dy = self.end.vec.y() - self.start.vec.y();
        self.start.vec.x() + dx * (y - self.start.vec.y()) / dy
    }
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> EdgeData<V, Vec2, S> {
    pub fn new(start: IndexedVertexPoint<V, Vec2, S>, end: IndexedVertexPoint<V, Vec2, S>) -> Self {
        EdgeData { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SweepReflexChainDirection {
    /// The reflex chain is completely on the left
    Left,
    /// The reflex chain is completely on the right
    Right,
    /// We have a right and a left element
    RightLeft,
    /// We have a left and a right element
    LeftRight,
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
            SweepReflexChainDirection::RightLeft => {
                // Consume the triangle immediately
                assert!(self.stack.len() == 2);
                indices.extend([self.stack[0], self.stack[1], value]);
                println!("create lrl: {:?}", [self.stack[0], self.stack[1], value]);
                // self.stack[0] = self.stack[0];
                self.stack[1] = value;
                self.d = SweepReflexChainDirection::Left;
            }
            SweepReflexChainDirection::LeftRight => {
                // Consume the triangle immediately
                assert!(self.stack.len() == 2);
                indices.extend([self.stack[0], self.stack[1], value]);
                println!("create llr: {:?}", [self.stack[0], self.stack[1], value]);
                self.stack[0] = self.stack[1];
                self.stack[1] = value;
                self.d = SweepReflexChainDirection::RightLeft;
            }
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
                    self.d = SweepReflexChainDirection::RightLeft;
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
                    self.d = SweepReflexChainDirection::RightLeft;
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
            SweepReflexChainDirection::RightLeft => {
                // Consume the triangle immediately
                assert!(self.stack.len() == 2);
                indices.extend([self.stack[0], self.stack[1], value]);
                println!("create rrl: {:?}", [self.stack[0], self.stack[1], value]);
                self.stack[0] = self.stack[1];
                self.stack[1] = value;
                self.d = SweepReflexChainDirection::LeftRight;
            }
            SweepReflexChainDirection::LeftRight => {
                // Consume the triangle immediately
                assert!(self.stack.len() == 2);
                indices.extend([self.stack[1], self.stack[0], value]);
                println!("create rlr: {:?}", [self.stack[1], self.stack[0], value]);
                //self.stack[0] = self.stack[0];
                self.stack[1] = value;
                self.d = SweepReflexChainDirection::Right;
            }
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
                    self.d = SweepReflexChainDirection::LeftRight;
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
                    self.d = SweepReflexChainDirection::LeftRight;
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

    /* pub fn close(&self, value: V, indices: &mut Vec<V>) {
        assert!(self.stack.len() >= 2);
        for i in 1..self.stack.len() {
            indices.extend([self.stack[i - 1], self.stack[i], value]);
            println!(
                "create mul: {:?}",
                [self.stack[i], self.stack[i - 1], value]
            );
        }
    }*/
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalData<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub helper: IndexedVertexPoint<V, Vec2, S>,
    pub left: EdgeData<V, Vec2, S>,
    pub right: EdgeData<V, Vec2, S>,
    pub stacks: SweepReflexChain<V>,
    pub fixup: Option<SweepReflexChain<V>>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> IntervalData<V, Vec2, S> {
    pub fn contains(&self, pos: &Vec2) -> bool {
        assert!(self.left.x_at_y(pos.y()) <= self.right.x_at_y(pos.y()));
        self.left.x_at_y(pos.y()) <= pos.x() && pos.x() <= self.right.x_at_y(pos.y())
    }
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::fmt::Display for IntervalData<V, Vec2, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lowest: {} ", self.helper.index)?;
        write!(
            f,
            "left: {}->{} ",
            self.left.start.index, self.left.end.index
        )?;
        write!(
            f,
            "right: {}->{} ",
            self.right.start.index, self.right.end.index
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OrderedFloats<S: Scalar> {
    value: S,
}

impl<S: Scalar> OrderedFloats<S> {
    pub fn new(value: S) -> Self {
        OrderedFloats { value }
    }
}

impl<S: Scalar> std::cmp::Eq for OrderedFloats<S> {}

impl<S: Scalar> std::cmp::Ord for OrderedFloats<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .partial_cmp(&other.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

pub struct SweepLineStatus<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<V, IntervalData<V, Vec2, S>>,
    /// Maps right targets to left targets
    right: HashMap<V, V>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> SweepLineStatus<V, Vec2, S> {
    pub fn new() -> Self {
        SweepLineStatus {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: IntervalData<V, Vec2, S>) {
        // TODO: assert that the pos is inbetween the start and end
        self.right
            .insert(value.right.end.index, value.left.end.index);
        self.left.insert(value.left.end.index, value);
    }

    pub fn get_left(&self, key: &V) -> Option<&IntervalData<V, Vec2, S>> {
        self.left.get(key)
    }

    pub fn get_right(&self, key: &V) -> Option<&IntervalData<V, Vec2, S>> {
        self.right.get(key).and_then(|key| self.left.get(key))
    }

    pub fn remove_left(&mut self, key: &V) -> Option<IntervalData<V, Vec2, S>> {
        if let Some(v) = self.left.remove(key) {
            self.right.remove(&v.right.end.index);
            Some(v)
        } else {
            None
        }
    }

    pub fn remove_right(&mut self, key: &V) -> Option<IntervalData<V, Vec2, S>> {
        if let Some(k) = self.right.remove(key) {
            self.left.remove(&k)
        } else {
            None
        }
    }

    pub fn find_by_position(&self, pos: &Vec2) -> Option<(&V, &IntervalData<V, Vec2, S>)> {
        // TODO: faster search using a BTreeMap
        self.left.iter().find(|(_, v)| v.contains(pos))
    }

    /*
    pub fn remove(&mut self, key: &OrderedFloats<S>) -> Option<IntervalData<V, Vec2, S>> {
        self.map.remove(key)
    }

    pub fn next(&self, value: S) -> Option<(&OrderedFloats<S>, &IntervalData<V, Vec2, S>)> {
        self.map
            .range((Included(&OrderedFloats::new(value)), Unbounded))
            .nth(1)
    }

    pub fn prev(&self, value: S) -> Option<(&OrderedFloats<S>, &IntervalData<V, Vec2, S>)> {
        self.map
            .range((Unbounded, Included(&OrderedFloats::new(value))))
            .next_back()
    }
    */
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::fmt::Display for SweepLineStatus<V, Vec2, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SweepLineStatus:\n")?;
        for (k, v) in &self.left {
            write!(f, "  {}: {}\n", k, v)?;
        }
        Ok(())
    }
}
