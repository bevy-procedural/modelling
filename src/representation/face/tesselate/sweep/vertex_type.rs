use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VertexType {
    Start,
    End,
    Split,
    Merge,
    Regular,
}

impl VertexType {
    // TODO: When there are two vertices with the same y-coordinate, the vertex type is not well defined. i.e., the first one should be Start and all others should be regular
    pub fn new<V: IndexType, Vec2: Vector2D<S>, S: Scalar>(
        prev: Vec2,
        here: Vec2,
        next: Vec2,
        tol: S,
    ) -> Self {
        let cross = (here - prev).cross2d(&(next - here));
        let is_above_prev = here.y() - prev.y() > tol;
        let is_above_next = here.y() - next.y() > tol;
        let is_below_prev = prev.y() - here.y() > tol;
        let is_below_next = next.y() - here.y() > tol;

        if (is_above_prev && is_above_next)
            || (is_below_prev && is_below_next && cross.abs() <= tol)
        {
            if cross > tol {
                VertexType::Start
            } else if cross < -tol {
                VertexType::Split
            } else {
                // This handles the edge case where the cross product is within the tolerance
                // TODO: What is this?
                println!("WARN: Cross product is within tolerance");
                VertexType::Regular
            }
        } else if is_below_prev && is_below_next {
            if cross > tol {
                VertexType::End
            } else if cross < -tol {
                VertexType::Merge
            } else {
                // Similar handling for when the cross product is within the tolerance
                // TODO: What is this?
                println!("WARN: Cross product is within tolerance");
                VertexType::Regular
            }
        } else {
            VertexType::Regular
        }
    }
}
