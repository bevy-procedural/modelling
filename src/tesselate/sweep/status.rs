use super::{interval::SweepLineInterval, monotone::MonotoneTriangulator};
use crate::{
    math::{IndexType, Scalar, Vector, Vector2D},
    mesh::IndexedVertex2D,
};
use std::collections::{BTreeSet, HashMap};

/// Sweep Line Interval Sorter
#[derive(Clone)]
struct SLISorter<Vec2: Vector2D> {
    /// The left end index of the interval
    left: usize,

    /// starting coordinate of the left interval boundary
    from: Vec2,

    /// ending coordinate of the left interval boundary
    to: Vec2,
}

impl<Vec2: Vector2D> PartialEq for SLISorter<Vec2> {
    fn eq(&self, other: &Self) -> bool {
        // the left index is unique for each interval
        self.left == other.left
    }
}

impl<Vec2: Vector2D> Eq for SLISorter<Vec2> {}

impl<Vec2: Vector2D> PartialOrd for SLISorter<Vec2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c: Vec2::S = other.from.y().min(self.from.y());

        // compare the horizontal positions at the current vertical position of the sweep line
        // Since the boundaries of the development of the sweep line segments
        // never cross during their stay in the tree, this should never break the ordering.
        other.x_at_y(c).partial_cmp(&self.x_at_y(c))
    }
}

impl<Vec2: Vector2D> Ord for SLISorter<Vec2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Ordering failed - are there NaN or inf values in your mesh?")
    }
}

impl<Vec2: Vector2D> std::fmt::Debug for SLISorter<Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntervalBoundary: {} ({:?} -> {:?})",
            self.left, self.from, self.to
        )
    }
}

impl<Vec2: Vector2D> SLISorter<Vec2> {
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
        SLISorter { left, from, to }
    }

    pub fn from_interval<V: IndexType, MT: MonotoneTriangulator<V = V, Vec2 = Vec2>>(
        interval: &SweepLineInterval<MT>,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> SLISorter<Vec2> {
        let from = vec2s[interval.left.start].vec;
        let to = vec2s[interval.left.end].vec;
        SLISorter::new(interval.left.end, from, to)
    }
}

/// The sweep line walks through the polygon and is segmented
/// into smaller intervals by the edges of the polygon.
/// The sweep line status keeps track of all sweep line intervals
/// that are currently inside the polygon.
pub struct SweepLineStatus<MT: MonotoneTriangulator> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, SweepLineInterval<MT>>,

    /// Maps right targets to left targets
    right: HashMap<usize, usize>,

    /// Use a b-tree to quickly find the correct interval
    tree: Option<BTreeSet<SLISorter<MT::Vec2>>>,
}

impl<MT: MonotoneTriangulator> SweepLineStatus<MT> {
    pub fn new() -> Self {
        SweepLineStatus {
            left: HashMap::new().hasher(),
            right: HashMap::new(),
            tree: None,
        }
    }

    pub fn insert(
        &mut self,
        value: SweepLineInterval<MT>,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) {
        // TODO: assert that the pos is in between the start and end
        debug_assert!(value.sanity_check());

        self.tree
            .as_mut()
            .map(|tree| assert!(tree.insert(SLISorter::from_interval(&value, vec2s))));

        self.right.insert(value.right.end, value.left.end);
        self.left.insert(value.left.end, value);
    }

    pub fn remove_left(
        &mut self,
        key: usize,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Option<SweepLineInterval<MT>> {
        if let Some(v) = self.left.remove(&key) {
            self.tree
                .as_mut()
                .map(|tree| assert!(tree.remove(&SLISorter::from_interval(&v, vec2s))));
            self.right.remove(&v.right.end);
            Some(v)
        } else {
            None
        }
    }

    pub fn peek_left(&self, key: usize) -> Option<&SweepLineInterval<MT>> {
        self.left.get(&key)
    }

    pub fn remove_right(
        &mut self,
        key: usize,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Option<SweepLineInterval<MT>> {
        if let Some(k) = self.right.remove(&key) {
            self.left.remove(&k).map(|v| {
                self.tree
                    .as_mut()
                    .map(|tree| tree.remove(&SLISorter::from_interval(&v, vec2s)));
                v
            })
        } else {
            None
        }
    }

    pub fn peek_right(&self, key: usize) -> Option<&SweepLineInterval<MT>> {
        self.right.get(&key).and_then(|k| self.peek_left(*k))
    }

    pub fn tree_sanity_check(&self, at: <MT::Vec2 as Vector2D>::S) -> bool {
        let mut last: Option<&SLISorter<MT::Vec2>> = None;
        for sorter in self.tree.as_ref().unwrap() {
            if let Some(l) = last {
                let last_at = SLISorter::new(l.left, MT::Vec2::new(l.x_at_y(at), at), l.to);
                assert!(
                    last_at <= *sorter,
                    "Tree is not sorted correctly at {} because {:?} <= {:?} does not hold.",
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
    fn find_linearly(
        &self,
        pos: &MT::Vec2,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Option<usize> {
        self.left
            .iter()
            .find(|(_, v)| v.contains(pos, vec2s))
            .map(|(k, _)| *k)
    }

    /// Find an interval by its coordinates on the sweep line using binary search.
    /// This runs in O(B * log n) time.
    fn find_btree(
        &self,
        pos: &MT::Vec2,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Option<usize> {
        let sorter = SLISorter::new(usize::MAX, *pos, *pos);

        debug_assert!(
            self.tree_sanity_check(pos.y()),
            "Tree is not sorted correctly. The sorting invariant is broken."
        );

        // Find the first interval that contains the position
        let x = self
            .tree
            .as_ref()
            .expect("The tree should be initialized.")
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

        if let Some(i) = x {
            if self.left[&i].contains(pos, vec2s) {
                return x;
            }
        }
        None
    }

    /// Delayed initialization of the b-tree
    fn init_btree(&mut self, vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>) {
        assert!(self.tree.is_none());
        let mut tree = BTreeSet::new();
        for (_, v) in &self.left {
            tree.insert(SLISorter::from_interval(v, vec2s));
        }
        self.tree = Some(tree);
    }

    /// This will find the left start index of interval that contains the given position or None if no interval contains the position.
    /// The algorithm will use a BTree if there are enough intervals to make it worthwhile.
    /// For a small number of intervals, a linear search will be used.
    /// The BTree will only be initialized and kept alive during the insert/remove operations once it is needed for the first time.
    pub fn find_by_position(
        &mut self,
        pos: &MT::Vec2,
        vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>,
    ) -> Option<usize> {
        const MIN_INTERVALS_FOR_BTREE: usize = 8;

        if self.left.len() > MIN_INTERVALS_FOR_BTREE || self.tree.is_some() {
            if self.tree.is_none() {
                self.init_btree(vec2s);
            }
            self.find_btree(pos, vec2s)
        } else {
            self.find_linearly(pos, vec2s)
        }
    }
}

impl<MT: MonotoneTriangulator> std::fmt::Debug for SweepLineStatus<MT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SweepLineStatus:\n")?;
        for (k, v) in &self.left {
            write!(f, "  {}: {:?}\n", k, v)?;
        }
        Ok(())
    }
}
