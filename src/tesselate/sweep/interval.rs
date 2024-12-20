use super::monotone::MonotoneTriangulator;
use crate::{
    math::{IndexType, Scalar, Vector, Vector2D},
    mesh::IndexedVertex2D,
};

/// This represents a single edge constraining a sweep line interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalBoundaryEdge {
    pub start: usize,
    pub end: usize,
}

impl IntervalBoundaryEdge {
    /// Calculates the x-coordinate of the edge at a given y-coordinate.
    ///
    /// This method uses linear interpolation to find the x-coordinate on the edge
    /// defined by the start and end vertices at the specified y-coordinate.
    pub fn x_at_y<V: IndexType, Vec2: Vector2D>(
        &self,
        y: Vec2::S,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) -> Vec2::S {
        let e = vec2s[self.end].vec;
        let s = vec2s[self.start].vec;
        let dx = e.x() - s.x();
        let dy = e.y() - s.y();
        if dy == Vec2::S::ZERO {
            // when parallel to the sweep line, we can just use the x-coordinate of the end vertex
            e.x()
        } else {
            s.x() + dx * (y - s.y()) / dy
        }
    }

    /// Calculate the parameters of the beam f(y) = a*y + b where y >= c
    pub fn beam<V: IndexType, Vec2: Vector2D>(
        &self,
        vec2s: &Vec<IndexedVertex2D<V, Vec2>>,
    ) -> Option<(Vec2::S, Vec2::S, Vec2::S)> {
        let e = vec2s[self.end].vec;
        let s = vec2s[self.start].vec;
        let dx = e.x() - s.x();
        let dy = e.y() - s.y();
        if dy == Vec2::S::ZERO {
            return None;
        }
        let a = dx / dy;
        let b = s.x() - a * s.y();
        let c = s.y();
        Some((a, b, c))
    }
}

impl IntervalBoundaryEdge {
    pub fn new(start: usize, end: usize) -> Self {
        IntervalBoundaryEdge { start, end }
    }
}

/// This represents a single interval of the sweep line.
/// Each interval stores edges that are still work in progress
/// and information in how to connect them to the rest of the mesh.
#[derive(Clone, PartialEq, Eq)]
pub struct SweepLineInterval<MT: MonotoneTriangulator> {
    /// The lowest vertex index of the interval.
    /// Things can be connected to this vertex when needed.
    pub helper: usize,

    /// The edge to the left of the interval
    pub left: IntervalBoundaryEdge,

    /// The edge to the right of the interval
    pub right: IntervalBoundaryEdge,

    /// There might be a longer chain of edges that connect the left
    /// and right edge and are not yet included in generated triangles.
    /// Those are stored in the reflex chain and are either leading to the
    /// left or to the right edge.
    pub chain: MT,

    /// Whether there was a merge that needs to be fixed up.
    pub fixup: Option<MT>,
}

impl<MT: MonotoneTriangulator> SweepLineInterval<MT> {
    /// Check whether the interval contains a position
    pub fn contains(&self, pos: &MT::Vec2, vec2s: &Vec<IndexedVertex2D<MT::V, MT::Vec2>>) -> bool {
        let p1 = self.left.x_at_y::<MT::V, MT::Vec2>(pos.y(), vec2s);
        // return `false` early to speed things up
        if p1 > pos.x() {
            return false;
        }
        let p2 = self.right.x_at_y::<MT::V, MT::Vec2>(pos.y(), vec2s);
        assert!(p1 <= p2);
        return pos.x() <= p2;
    }

    /// Check if the interval is circular
    fn is_circular(&self) -> bool {
        (self.left.start == self.right.end && self.right.start == self.left.end)
            || (self.left.start == self.right.start && self.left.end == self.right.end)
    }

    /// When the intervals are connected, the next vertex must be the end.
    pub fn is_end(&self) -> bool {
        self.left.end == self.right.end
    }

    pub fn sanity_check(&self) -> bool {
        assert!(!self.is_circular());
        self.chain
            .sanity_check(self.left.start, self.right.start, &self.fixup);
        return true;
    }
}

impl<MT: MonotoneTriangulator> std::fmt::Debug for SweepLineInterval<MT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lowest: {} ", self.helper)?;
        write!(f, "left: {}->{} ", self.left.start, self.left.end)?;
        write!(f, "right: {}->{} ", self.right.start, self.right.end)?;
        write!(f, "stacks: {:?} ", self.chain)?;
        if let Some(fixup) = &self.fixup {
            write!(f, "fixup: {:?}", fixup)?;
        }
        Ok(())
    }
}
