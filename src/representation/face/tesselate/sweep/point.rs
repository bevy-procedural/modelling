use super::vertex_type::VertexType;
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexedVertexPoint<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    /// Position of the point
    pub vec: Vec2,
    /// Index in the local structure
    pub local: usize,
    /// Index in the mesh
    pub index: V,
    phantom: std::marker::PhantomData<S>,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> IndexedVertexPoint<V, Vec2, S> {
    pub fn new(vec: Vec2, local: usize, index: V) -> Self {
        IndexedVertexPoint {
            vec,
            local,
            index,
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
    /// Previous vertex in the face
    pub prev: usize,
    /// Current vertex in the face
    pub here: usize,
    /// Next vertex in the face
    pub next: usize,

    pub vec: Vec2,
    /// Precomputed vertex type
    pub vertex_type: VertexType,

    phantom: std::marker::PhantomData<S>,
}

impl<Vec2: Vector2D<S>, S: Scalar> EventPoint<Vec2, S> {
    pub fn new(
        prev: usize,
        here: usize,
        next: usize,
        vertex_type: VertexType,
        vec: Vec2,
    ) -> Self {
        EventPoint {
            prev,
            here,
            next,
            vec,
            vertex_type,
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
