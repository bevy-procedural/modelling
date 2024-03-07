use super::{chain::SweepReflexChain, point::IndexedVertexPoint};
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
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
pub struct IntervalData<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub helper: IndexedVertexPoint<V, Vec2, S>,
    pub left: EdgeData<V, Vec2, S>,
    pub right: EdgeData<V, Vec2, S>,
    pub stacks: SweepReflexChain,
    pub fixup: Option<SweepReflexChain>,
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
