use super::{interval::SweepLineInterval, point::LocallyIndexedVertex};
use crate::{math::Vector2D, representation::IndexType};
use std::collections::HashMap;

/// The sweep line walks through the polygon and is segmented into smaller intervals by the edges of the polygon.
/// The sweep line status keeps track of all sweep line intervals that are currently inside the polygon.
pub struct SweepLineStatus<V: IndexType, Vec2: Vector2D> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, SweepLineInterval<V, Vec2>>,
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

    pub fn insert(&mut self, value: SweepLineInterval<V, Vec2>) {
        // TODO: assert that the pos is in between the start and end
        debug_assert!(value.sanity_check());
        self.right.insert(value.right.end, value.left.end);
        self.left.insert(value.left.end, value);
    }

    pub fn get_left(&self, key: usize) -> Option<&SweepLineInterval<V, Vec2>> {
        self.left.get(&key)
    }

    pub fn get_right(&self, key: usize) -> Option<&SweepLineInterval<V, Vec2>> {
        self.right.get(&key).and_then(|key| self.left.get(key))
    }

    pub fn remove_left(&mut self, key: usize) -> Option<SweepLineInterval<V, Vec2>> {
        if let Some(v) = self.left.remove(&key) {
            self.right.remove(&v.right.end);
            Some(v)
        } else {
            None
        }
    }

    pub fn remove_right(&mut self, key: usize) -> Option<SweepLineInterval<V, Vec2>> {
        if let Some(k) = self.right.remove(&key) {
            self.left.remove(&k)
        } else {
            None
        }
    }

    /// Find an interval by its coordinates on the sweep line
    /// 
    /// This should be done in O(log n) time, but currently uses a linear search.
    /// This is quite important for the algorithm since it could cause a worst case of O(n^2) time complexity.
    pub fn find_by_position(
        &self,
        pos: &Vec2,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Option<(&usize, &SweepLineInterval<V, Vec2>)> {
        // TODO: faster search using a BTreeMap. Or maybe binary search is enough?
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
