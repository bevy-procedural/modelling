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
pub struct EventPoint<V: IndexType, Vec2: Vector2D<S>, S: Scalar> {
    pub prev: IndexedVertexPoint<V, Vec2, S>,
    pub here: IndexedVertexPoint<V, Vec2, S>,
    pub next: IndexedVertexPoint<V, Vec2, S>,
    pub vertex_type: VertexType,
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::cmp::PartialEq for EventPoint<V, Vec2, S> {
    fn eq(&self, other: &Self) -> bool {
        self.here.vec.y() == other.here.vec.y()
    }
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::cmp::Eq for EventPoint<V, Vec2, S> {}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::cmp::PartialOrd for EventPoint<V, Vec2, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(res) = self.here.vec.y().partial_cmp(&other.here.vec.y()) {
            if res == std::cmp::Ordering::Equal {
                other.here.vec.x().partial_cmp(&self.here.vec.x())
            } else {
                Some(res)
            }
        } else {
            None
        }
    }
}

impl<V: IndexType, Vec2: Vector2D<S>, S: Scalar> std::cmp::Ord for EventPoint<V, Vec2, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO: Undefined behavior if float comparison is not defined
        if let Some(res) = self.here.vec.y().partial_cmp(&other.here.vec.y()) {
            if res == std::cmp::Ordering::Equal {
                other
                    .here
                    .vec
                    .x()
                    .partial_cmp(&self.here.vec.x())
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                res
            }
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
