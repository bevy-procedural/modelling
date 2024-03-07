use super::vertex_type::VertexType;
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedVertexPoint<Vec2: Vector2D<S>, S: Scalar> {
    /// Position of the point
    pub vec: Vec2,
    /// Index in the local structure
    pub local: usize,
    phantom: std::marker::PhantomData<S>,
}

impl<Vec2: Vector2D<S>, S: Scalar> IndexedVertexPoint<Vec2, S> {
    pub fn new(vec: Vec2, local: usize) -> Self {
        IndexedVertexPoint {
            vec,
            local,
            phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventPoint<Vec2, S>
where
    Vec2: Vector2D<S>,
    S: Scalar,
{
    /// Current vertex in the face
    pub here: usize,
    pub prev: usize,
    pub next: usize,

    pub vec: Vec2,
    /// Precomputed vertex type
    pub vertex_type: VertexType,

    phantom: std::marker::PhantomData<S>,
}

impl<Vec2: Vector2D<S>, S: Scalar> EventPoint<Vec2, S> {
    pub fn new<V: IndexType>(here: usize, vec2s: &Vec<IndexedVertexPoint<Vec2, S>>) -> Self {
        let prev = (here + vec2s.len() - 1) % vec2s.len();
        let next = (here + 1) % vec2s.len();

        EventPoint {
            here,
            vec: vec2s[here].vec,
            prev,
            next,
            vertex_type: VertexType::new::<V, Vec2, S>(
                vec2s[prev].vec,
                vec2s[here].vec,
                vec2s[next].vec,
                S::EPS,
            ),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<Vec2: Vector2D<S>, S: Scalar> std::cmp::PartialEq for EventPoint<Vec2, S> {
    fn eq(&self, other: &Self) -> bool {
        self.vec.y() == other.vec.y()
    }
}

impl<Vec2: Vector2D<S>, S: Scalar> std::cmp::Eq for EventPoint<Vec2, S> {}

impl<Vec2: Vector2D<S>, S: Scalar> std::cmp::PartialOrd for EventPoint<Vec2, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(res) = self.vec.y().partial_cmp(&other.vec.y()) {
            if res == std::cmp::Ordering::Equal {
                other.vec.x().partial_cmp(&self.vec.x())
            } else {
                Some(res)
            }
        } else {
            None
        }
    }
}

impl<Vec2: Vector2D<S>, S: Scalar> std::cmp::Ord for EventPoint<Vec2, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO: Undefined behavior if float comparison is not defined
        if let Some(res) = self.vec.y().partial_cmp(&other.vec.y()) {
            if res == std::cmp::Ordering::Equal {
                other
                    .vec
                    .x()
                    .partial_cmp(&self.vec.x())
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                res
            }
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
