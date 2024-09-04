use super::{chain::SweepReflexChain, point::LocallyIndexedVertex};
use crate::{math::Vector2D, representation::IndexType};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeData {
    pub start: usize,
    pub end: usize,
}

impl EdgeData {
    pub fn x_at_y<V: IndexType, Vec2: Vector2D>(
        &self,
        y: Vec2::S,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Vec2::S {
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
pub struct IntervalData<V: IndexType, Vec2: Vector2D> {
    pub helper: usize,
    pub left: EdgeData,
    pub right: EdgeData,
    pub stacks: SweepReflexChain<V, Vec2>,
    pub fixup: Option<SweepReflexChain<V, Vec2>>,
}

impl<V: IndexType, Vec2: Vector2D> IntervalData<V, Vec2> {
    pub fn contains(&self, pos: &Vec2, vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> bool {
        let p1 = self.left.x_at_y::<V, Vec2>(pos.y(), vec2s);
        let p2 = self.right.x_at_y::<V, Vec2>(pos.y(), vec2s);
        assert!(p1 <= p2);
        p1 <= pos.x() && pos.x() <= p2
    }

    pub fn is_circular(&self) -> bool {
        (self.left.start == self.right.end && self.right.start == self.left.end)
            || (self.left.start == self.right.start && self.left.end == self.right.end)
    }

    pub fn is_end(&self) -> bool {
        self.left.end == self.right.end
    }
}

// TODO: local indices
impl<V: IndexType, Vec2: Vector2D> std::fmt::Display for IntervalData<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lowest: {} ", self.helper)?;
        write!(f, "left: {}->{} ", self.left.start, self.left.end)?;
        write!(f, "right: {}->{} ", self.right.start, self.right.end)?;
        Ok(())
    }
}

pub struct SweepLineStatus<V: IndexType, Vec2: Vector2D> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, IntervalData<V, Vec2>>,
    /// Maps right targets to left targets
    right: HashMap<usize, usize>,
}

impl<V: IndexType, Vec2: Vector2D> SweepLineStatus<V, Vec2> {
    pub fn new() -> Self {
        SweepLineStatus {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: IntervalData<V, Vec2>) {
        // TODO: assert that the pos is in between the start and end
        assert!(!value.is_circular());
        self.right.insert(value.right.end, value.left.end);
        self.left.insert(value.left.end, value);
    }

    pub fn get_left(&self, key: usize) -> Option<&IntervalData<V, Vec2>> {
        self.left.get(&key)
    }

    pub fn get_right(&self, key: usize) -> Option<&IntervalData<V, Vec2>> {
        self.right.get(&key).and_then(|key| self.left.get(key))
    }

    pub fn remove_left(&mut self, key: usize) -> Option<IntervalData<V, Vec2>> {
        if let Some(v) = self.left.remove(&key) {
            self.right.remove(&v.right.end);
            Some(v)
        } else {
            None
        }
    }

    pub fn remove_right(&mut self, key: usize) -> Option<IntervalData<V, Vec2>> {
        if let Some(k) = self.right.remove(&key) {
            self.left.remove(&k)
        } else {
            None
        }
    }

    pub fn find_by_position(
        &self,
        pos: &Vec2,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Option<(&usize, &IntervalData<V, Vec2>)> {
        // TODO: faster search using a BTreeMap
        self.left.iter().find(|(_, v)| v.contains(pos, vec2s))
    }
}

impl<V: IndexType, Vec2: Vector2D> std::fmt::Display for SweepLineStatus<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SweepLineStatus:\n")?;
        for (k, v) in &self.left {
            write!(f, "  {}: {}\n", k, *v)?;
        }
        Ok(())
    }
}
