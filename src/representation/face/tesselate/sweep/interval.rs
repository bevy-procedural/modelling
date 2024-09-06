use super::{chain::ReflexChain, point::LocallyIndexedVertex};
use crate::{
    math::{IndexType, Vector2D},
    representation::tesselate::sweep::chain::ReflexChainDirection,
};

/// This represents a single edge constraining a sweep line interval.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        vec2s: &Vec<LocallyIndexedVertex<Vec2>>,
    ) -> Vec2::S {
        let e = vec2s[self.end].vec;
        let s = vec2s[self.start].vec;
        let dx = e.x() - s.x();
        let dy = e.y() - s.y();
        // TODO: handle division by zero
        s.x() + dx * (y - s.y()) / dy
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepLineInterval<V: IndexType, Vec2: Vector2D> {
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
    pub chain: ReflexChain<V, Vec2>,

    /// Whether there was a merge that needs to be fixed up.
    pub fixup: Option<ReflexChain<V, Vec2>>,
}

impl<V: IndexType, Vec2: Vector2D> SweepLineInterval<V, Vec2> {
    /// Check whether the interval contains a position
    pub fn contains(&self, pos: &Vec2, vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> bool {
        let p1 = self.left.x_at_y::<V, Vec2>(pos.y(), vec2s);
        let p2 = self.right.x_at_y::<V, Vec2>(pos.y(), vec2s);
        assert!(p1 <= p2);
        p1 <= pos.x() && pos.x() <= p2
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
        match self.chain.direction() {
            ReflexChainDirection::None => {
                assert!(self.chain.len() == 1);
                assert_eq!(self.left.start, self.chain.first());
                assert_eq!(self.right.start, self.chain.first());
            }
            ReflexChainDirection::Left => {
                assert!(self.chain.len() >= 2);
                if let Some(fixup) = &self.fixup {
                    assert!(fixup.len() >= 2);
                    assert_eq!(self.right.start, self.chain.first());
                    assert_eq!(self.left.start, fixup.first());
                } else {
                    assert_eq!(self.right.start, self.chain.first());
                    assert_eq!(self.left.start, self.chain.last());
                }
            }
            ReflexChainDirection::Right => {
                assert!(self.chain.len() >= 2);
                if let Some(fixup) = &self.fixup {
                    assert!(fixup.len() >= 2);
                    assert_eq!(self.left.start, self.chain.first());
                    assert_eq!(self.right.start, fixup.first());
                } else {
                    assert_eq!(self.left.start, self.chain.first());
                    assert_eq!(self.right.start, self.chain.last());
                }
            }
        };
        return true;
    }
}

// TODO: local indices
impl<V: IndexType, Vec2: Vector2D> std::fmt::Display for SweepLineInterval<V, Vec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lowest: {} ", self.helper)?;
        write!(f, "left: {}->{} ", self.left.start, self.left.end)?;
        write!(f, "right: {}->{} ", self.right.start, self.right.end)?;
        write!(f, "stacks: {} ", self.chain)?;
        if let Some(fixup) = &self.fixup {
            write!(f, "fixup: {}", fixup)?;
        }
        Ok(())
    }
}
