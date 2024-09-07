use super::{interval::SweepLineInterval, point::LocallyIndexedVertex};
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};
use std::collections::{BTreeSet, HashMap};

// PERF: Insert / remove is much more frequent than find_by_position. It could be a good idea to not build the heap until we have a find_by_position call with a large number of intervals.

#[derive(Debug, Clone)]
struct SweepLineIntervalSorter<S: Scalar> {
    /// The left index of the interval
    left: usize,

    /// The left boundary is given as a beam f(y) = a*y + b where y >= c
    a: S,

    /// The left boundary is given as a beam f(y) = a*y + b where y >= c
    b: S,

    /// The left boundary is given as a beam f(y) = a*y + b where y >= c
    c: S,
}

impl<S: Scalar> PartialEq for SweepLineIntervalSorter<S> {
    fn eq(&self, other: &Self) -> bool {
        // the left index is unique for each interval
        self.left == other.left
    }
}

impl<S: Scalar> Eq for SweepLineIntervalSorter<S> {}

impl<S: Scalar> PartialOrd for SweepLineIntervalSorter<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = S::max(other.c, self.c);

        // compare the vertical positions at the current position of the sweep line
        // Since the boundaries of the development of the sweep line segments
        // never cross during their stay in the tree, this should never break the ordering.
        let other_y = other.a * c + other.b;
        let my_y = self.a * c + self.b;
        my_y.partial_cmp(&other_y)
    }
}

impl<S: Scalar> Ord for SweepLineIntervalSorter<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Ordering failed - are there NaN or inf values in your mesh?")
    }
}

impl<S: Scalar> std::fmt::Display for SweepLineIntervalSorter<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntervalBoundary: {} ({}*y + {}, y >= {})",
            self.left, self.a, self.b, self.c
        )
    }
}

impl<S: Scalar> SweepLineIntervalSorter<S> {
    pub fn new(left: usize, a: S, b: S, c: S) -> Self {
        assert!(
            a.is_finite()
                && !a.is_nan()
                && b.is_finite()
                && !b.is_nan()
                && c.is_finite()
                && !c.is_nan(),
            " Cannot construct SweepLineIntervalSorter({}, {}, {}) because the values are invalid.",
            a,
            b,
            c
        );

        SweepLineIntervalSorter { left, a, b, c }
    }

    pub fn from_interval<V: IndexType, Vec2: Vector2D>(
        interval: &SweepLineInterval<V, Vec2>,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> SweepLineIntervalSorter<Vec2::S> {
        let (a, b, c) = interval.left.beam::<V, Vec2>(vec2s);
        SweepLineIntervalSorter::new(interval.left.start, a, b, c)
    }
}

/// The sweep line walks through the polygon and is segmented
/// into smaller intervals by the edges of the polygon.
/// The sweep line status keeps track of all sweep line intervals
/// that are currently inside the polygon.
pub struct SweepLineStatus<V: IndexType, Vec2: Vector2D> {
    /// The sweep lines, ordered by the target vertex index of the left edge
    left: HashMap<usize, (SweepLineInterval<V, Vec2>, SweepLineIntervalSorter<Vec2::S>)>,

    /// Maps right targets to left targets
    right: HashMap<usize, usize>,

    /// Use a b-tree to quickly find the correct interval
    tree: BTreeSet<SweepLineIntervalSorter<Vec2::S>>,
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

        let sis = SweepLineIntervalSorter::<Vec2::S>::from_interval(&value, vec2s);

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
        let mut last = None;
        for sorter in &self.tree {
            let last_at = last.map(|l: &SweepLineIntervalSorter<Vec2::S>| {
                SweepLineIntervalSorter::<Vec2::S>::new(l.left, l.a, l.b, at)
            });

            if let Some(l) = last_at {
                assert!(
                    l <= *sorter,
                    "Tree is not sorted correctly at {} because {} <= {} does not hold.",
                    at,
                    l,
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
    pub fn find_by_position(
        &self,
        pos: &Vec2,
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Option<usize> {
        let sorter = SweepLineIntervalSorter::new(usize::MAX, Vec2::S::ZERO, pos.x(), pos.y());

        debug_assert!(
            self.tree_sanity_check(pos.y()),
            "Tree is not sorted correctly. The sorting invariant is broken."
        );

        // Find the first interval that contains the position
        let x = self.tree.range(sorter..).next().map(|sorter| sorter.left);

        debug_assert!(
            x == self.find_linearly(pos, vec2s),
            "The binary search did not return the same result as the linear search. {:?} {:?}",
            x,
            self.find_linearly(pos, vec2s)
        );

        x
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
