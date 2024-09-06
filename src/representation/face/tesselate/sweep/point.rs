use super::vertex_type::VertexType;
use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocallyIndexedVertex<Vec2: Vector2D> {
    /// Position of the point
    pub vec: Vec2,
    /// Index in the local structure
    pub local: usize,
}

impl<Vec2: Vector2D> LocallyIndexedVertex<Vec2> {
    pub fn new(vec: Vec2, local: usize) -> Self {
        LocallyIndexedVertex {
            vec,
            local,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventPoint<Vec2>
where
    Vec2: Vector2D
{
    /// Current vertex in the face
    pub here: usize,
    pub prev: usize,
    pub next: usize,

    pub vec: Vec2,
    
    /// Precomputed vertex type
    pub vertex_type: VertexType,
}

impl<Vec2: Vector2D> EventPoint<Vec2> {
    pub fn classify<V: IndexType>(here: usize, vec2s: &Vec<LocallyIndexedVertex<Vec2>>) -> Self {
        let prev = (here + vec2s.len() - 1) % vec2s.len();
        let next = (here + 1) % vec2s.len();

        EventPoint {
            here,
            vec: vec2s[here].vec,
            prev,
            next,
            vertex_type: VertexType::detect::<V, Vec2>(
                vec2s[prev].vec,
                vec2s[here].vec,
                vec2s[next].vec,
                Vec2::S::EPS * Vec2::S::from(1000.0),
            )
        }
    }
}

impl<Vec2: Vector2D> std::cmp::PartialEq for EventPoint<Vec2> {
    fn eq(&self, other: &Self) -> bool {
        self.vec.y() == other.vec.y()
    }
}

impl<Vec2: Vector2D> std::cmp::Eq for EventPoint<Vec2> {}

impl<Vec2: Vector2D> std::cmp::PartialOrd for EventPoint<Vec2> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(res) = (-self.vec.y()).partial_cmp(&(-other.vec.y())) {
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

impl<Vec2: Vector2D> std::cmp::Ord for EventPoint<Vec2> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO: Undefined behavior if float comparison is not defined
        if let Some(res) = (-self.vec.y()).partial_cmp(&(-other.vec.y())) {
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
