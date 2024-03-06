use super::point::IndexedVertexPoint;
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeData<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub start: IndexedVertexPoint<V, Vec2, S>,
    pub end: IndexedVertexPoint<V, Vec2, S>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> EdgeData<V, Vec2, S> {
    pub fn new(start: IndexedVertexPoint<V, Vec2, S>, end: IndexedVertexPoint<V, Vec2, S>) -> Self {
        EdgeData { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalData<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub lowest: Option<IndexedVertexPoint<V, Vec2, S>>,
    pub left: EdgeData<V, Vec2, S>,
    pub right: EdgeData<V, Vec2, S>,
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
    map: BTreeMap<OrderedFloats<S>, IntervalData<V, Vec2, S>>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> SweepLineStatus<V, Vec2, S> {
    pub fn new() -> Self {
        SweepLineStatus {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: OrderedFloats<S>, value: IntervalData<V, Vec2, S>) {
        self.map.insert(key, value);
    }

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
}
