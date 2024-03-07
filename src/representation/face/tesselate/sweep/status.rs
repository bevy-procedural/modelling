use super::{chain::SweepReflexChain, point::IndexedVertexPoint};
use crate::{
    math::{Scalar, Vector, Vector2D},
    representation::{payload::Payload, IndexType},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeData {
    pub start: usize,
    pub end: usize,
}

impl EdgeData {
    pub fn x_at_y<V: IndexType, P: Payload>(
        &self,
        y: P::S,
        vec2s: &Vec<IndexedVertexPoint<V, P::Vec2, P::S>>,
    ) -> P::S {
        let e = vec2s[self.end].vec;
        let s = vec2s[self.start].vec;
        let dx = e.x() - s.x();
        let dy = e.y() - s.y();
        s.x() + dx * (y - s.y()) / dy
    }
}

impl EdgeData {
    pub fn new(start: usize, end: usize) -> Self {
        EdgeData { start, end }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalData {
    pub helper: usize,
    pub left: EdgeData,
    pub right: EdgeData,
    pub stacks: SweepReflexChain,
    pub fixup: Option<SweepReflexChain>,
}

impl IntervalData {
    pub fn contains<V: IndexType, P: Payload>(
        &self,
        pos: &P::Vec2,
        vec2s: &Vec<IndexedVertexPoint<V, P::Vec2, P::S>>,
    ) -> bool {
        let p1 = self.left.x_at_y::<V, P>(pos.y(), vec2s);
        let p2 = self.right.x_at_y::<V, P>(pos.y(), vec2s);
        assert!(p1 <= p2);
        p1 <= pos.x() && pos.x() <= p2
    }
}

// TODO: local indices
/*
impl std::fmt::Display for IntervalData {
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
}*/

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

pub struct SweepLineStatus {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, IntervalData>,
    /// Maps right targets to left targets
    right: HashMap<usize, usize>,
}

impl SweepLineStatus {
    pub fn new() -> Self {
        SweepLineStatus {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: IntervalData) {
        // TODO: assert that the pos is in between the start and end
        self.right.insert(value.right.end, value.left.end);
        self.left.insert(value.left.end, value);
    }

    pub fn get_left(&self, key: usize) -> Option<&IntervalData> {
        self.left.get(&key)
    }

    pub fn get_right(&self, key: usize) -> Option<&IntervalData> {
        self.right.get(&key).and_then(|key| self.left.get(key))
    }

    pub fn remove_left(&mut self, key: usize) -> Option<IntervalData> {
        if let Some(v) = self.left.remove(&key) {
            self.right.remove(&v.right.end);
            Some(v)
        } else {
            None
        }
    }

    pub fn remove_right(&mut self, key: usize) -> Option<IntervalData> {
        if let Some(k) = self.right.remove(&key) {
            self.left.remove(&k)
        } else {
            None
        }
    }

    pub fn find_by_position<V: IndexType, P: Payload>(
        &self,
        pos: &P::Vec2,
        vec2s: &Vec<IndexedVertexPoint<V, P::Vec2, P::S>>,
    ) -> Option<(&usize, &IntervalData)> {
        // TODO: faster search using a BTreeMap
        self.left
            .iter()
            .find(|(_, v)| v.contains::<V, P>(pos, vec2s))
    }
}

/*
impl std::fmt::Display for SweepLineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SweepLineStatus:\n")?;
        for (k, v) in &self.left {
            write!(f, "  {}: {}\n", k, v)?;
        }
        Ok(())
    }
}*/
