use crate::{
    math::{Scalar, Vector2D},
    representation::IndexType,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
/// The type of a vertex in a sweep line reflex chain
pub enum VertexType {
    #[default]
    /// Undefined vertex type.
    Undefined,

    /// Start a new sweep line here
    Start,

    /// End the sweep line here
    End,

    /// Split the sweep line in two parts at this scan reflex vertex
    Split,

    /// Merge two parts of the sweep line at this scan reflex vertex
    Merge,

    /// Polygon is monotone at this vertex 
    Regular,
}

impl VertexType {
    // TODO: When there are two vertices with the same y-coordinate, the vertex type is not well defined. i.e., the first one should be Start and all others should be regular
    /// Calculate the vertex type based on the previous, current and next vertices.
    pub fn new<V: IndexType, Vec2: Vector2D>(
        prev: Vec2,
        here: Vec2,
        next: Vec2,
        tol: Vec2::S,
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
