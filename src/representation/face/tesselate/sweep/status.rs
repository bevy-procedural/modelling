use super::{interval::SweepLineInterval, point::LocallyIndexedVertex};
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};
use std::collections::{BTreeSet, HashMap};

// PERF: Insert / remove is much more frequent than find_by_position. It could be a good idea to not build the heap until we have a find_by_position call with a large number of intervals.

#[derive(Debug, Clone)]
struct SweepLineIntervalSorter<Vec2: Vector2D> {
    /// The left end index of the interval
    left: usize,

    /// starting coordinate of the left interval boundary
    from: Vec2,

    /// ending coordinate of the left interval boundary
    to: Vec2,
}

impl<Vec2: Vector2D> PartialEq for SweepLineIntervalSorter<Vec2> {
    fn eq(&self, other: &Self) -> bool {
        // the left index is unique for each interval
        self.left == other.left
    }
}

impl<Vec2: Vector2D> Eq for SweepLineIntervalSorter<Vec2> {}

impl<Vec2: Vector2D> PartialOrd for SweepLineIntervalSorter<Vec2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c: Vec2::S = Vec2::S::min(other.from.y(), self.from.y());

        // compare the horizontal positions at the current vertical position of the sweep line
        // Since the boundaries of the development of the sweep line segments
        // never cross during their stay in the tree, this should never break the ordering.
        other.x_at_y(c).partial_cmp(&self.x_at_y(c))
    }
}

impl<Vec2: Vector2D> Ord for SweepLineIntervalSorter<Vec2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Ordering failed - are there NaN or inf values in your mesh?")
    }
}

impl<Vec2: Vector2D> std::fmt::Display for SweepLineIntervalSorter<Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntervalBoundary: {} ({:?} -> {:?})",
            self.left, self.from, self.to
        )
    }
}

impl<Vec2: Vector2D> SweepLineIntervalSorter<Vec2> {
    fn x_at_y(&self, y: Vec2::S) -> Vec2::S {
        let s = self.from;
        let e = self.to;
        let dy = e.y() - s.y();
        if dy == Vec2::S::ZERO {
            // when parallel to the sweep line, we can just use the x-coordinate of the end vertex
            e.x()
        } else {
            let dx = e.x() - s.x();
            s.x() + dx * (y - s.y()) / dy
        }
    }

    pub fn new(left: usize, from: Vec2, to: Vec2) -> Self {
        assert!(from.y() >= to.y());
        SweepLineIntervalSorter { left, from, to }
    }

    pub fn from_interval<V: IndexType>(
        interval: &SweepLineInterval<V, Vec2>,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> SweepLineIntervalSorter<Vec2> {
        let from = vec2s[interval.left.start].vec;
        let to = vec2s[interval.left.end].vec;
        SweepLineIntervalSorter::new(interval.left.end, from, to)
    }
}

/// The sweep line walks through the polygon and is segmented
/// into smaller intervals by the edges of the polygon.
/// The sweep line status keeps track of all sweep line intervals
/// that are currently inside the polygon.
pub struct SweepLineStatus<V: IndexType, Vec2: Vector2D> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, (SweepLineInterval<V, Vec2>, SweepLineIntervalSorter<Vec2>)>,

    /// Maps right targets to left targets
    right: HashMap<usize, usize>,

    /// Use a b-tree to quickly find the correct interval
    tree: BTreeSet<SweepLineIntervalSorter<Vec2>>,
}

impl<V: IndexType, Vec2: Vector2D> SweepLineStatus<V, Vec2> {
    pub fn new() -> Self {
        SweepLineStatus {
            left: HashMap::new(),
            right: HashMap::new(),
            tree: BTreeSet::new(),
        }
    }

    pub fn insert(
        &mut self,
        value: SweepLineInterval<V, Vec2>,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) {
        // TODO: assert that the pos is in between the start and end
        debug_assert!(value.sanity_check());

        let sis = SweepLineIntervalSorter::from_interval(&value, vec2s);

        // PERF: is it necessary to store the sis twice or can we remove them with another method?
        assert!(self.tree.insert(sis.clone()));
        self.right.insert(value.right.end, value.left.end);
        self.left.insert(value.left.end, (value, sis));
    }

    pub fn get_left(&self, key: usize) -> Option<&SweepLineInterval<V, Vec2>> {
        self.left.get(&key).map(|(v, _)| v)
    }

    pub fn get_right(&self, key: usize) -> Option<&SweepLineInterval<V, Vec2>> {
        self.right.get(&key).and_then(|key| self.get_left(*key))
    }

    pub fn remove_left(&mut self, key: usize) -> Option<SweepLineInterval<V, Vec2>> {
        if let Some((v, sis)) = self.left.remove(&key) {
            self.right.remove(&v.right.end);
            assert!(self.tree.remove(&sis));
            Some(v)
        } else {
            None
        }
    }

    pub fn remove_right(&mut self, key: usize) -> Option<SweepLineInterval<V, Vec2>> {
        if let Some(k) = self.right.remove(&key) {
            self.left.remove(&k).map(|(v, sis)| {
                assert!(self.tree.remove(&sis));
                v
            })
        } else {
            None
        }
    }

    pub fn tree_sanity_check(&self, at: Vec2::S) -> bool {
        let mut last: Option<&SweepLineIntervalSorter<Vec2>> = None;
        for sorter in &self.tree {
            if let Some(l) = last {
                let last_at =
                    SweepLineIntervalSorter::new(l.left, Vec2::from_xy(l.x_at_y(at), at), l.to);
                assert!(
                    last_at <= *sorter,
                    "Tree is not sorted correctly at {} because {} <= {} does not hold.",
                    at,
                    last_at,
                    *sorter
                );
            }
            last = Some(sorter);
        }
        return true;
    }

    /// Find an interval by its coordinates on the sweep line using linear search.
    /// This runs in O(n) time.
    fn find_linearly(&self, pos: &Vec2, vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> Option<usize> {
        self.left
            .iter()
            .find(|(_, v)| v.0.contains(pos, vec2s))
            .map(|(k, _)| *k)
    }

    /// Find an interval by its coordinates on the sweep line using binary search.
    /// This runs in O(B * log n) time.
    fn find_btree(&self, pos: &Vec2, vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> Option<usize> {
        let sorter = SweepLineIntervalSorter::new(usize::MAX, *pos, *pos);

        debug_assert!(
            self.tree_sanity_check(pos.y()),
            "Tree is not sorted correctly. The sorting invariant is broken."
        );

        // Find the first interval that contains the position
        let x = self
            .tree
            .range(sorter.clone()..)
            .next()
            .map(|sorter| sorter.left);

        debug_assert!(
            x == self.find_linearly(pos, vec2s),
            "The binary search did not return the same result as the linear search. {:?} != {:?}
            pos = {:?}
            {:?}
            ",
            x,
            self.find_linearly(pos, vec2s),
            pos,
            self.tree,
        );

        x
    }

    pub fn find_by_position(
        &self,
        pos: &Vec2,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Option<usize> {
        // TODO: use linear search for small numbers of intervals
        self.find_btree(pos, vec2s)
        //self.find_linearly(pos, vec2s)
    }
}

impl<V: IndexType, Vec2: Vector2D> std::fmt::Display for SweepLineStatus<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SweepLineStatus:\n")?;
        for (k, v) in &self.left {
            write!(f, "  {}: {}\n", k, v.0)?;
        }
        Ok(())
    }
}
