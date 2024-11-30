use crate::math::{IndexType, Scalar, Vector2D};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
/// The type of a vertex in a sweep line reflex chain
pub enum VertexType {
    #[default]
    /// Undefined vertex type.
    Undefined,

    /// Start a new sweep line here.
    /// Both edges lie to the right of v, but the interior angle is smaller than π.
    Start,

    /// A start vertex that is detected late.
    StartLate,

    /// Split the sweep line in two parts at this scan reflex vertex.
    Split,

    /// Split vertex that is detected late.
    SplitLate,

    /// End the sweep line here.
    /// Both edges lie to the left of v, but the interior angle is larger than π.
    End,

    /// An end vertex that is detected late.
    EndLate,

    /// Merge two parts of the sweep line at this scan reflex vertex.
    Merge,

    /// Merge vertex that is detected late.
    MergeLate,

    /// Polygon is monotone at this vertex.
    /// Can be a hidden Start or End vertex that will be discovered during the sweep.
    /// One edge is to the left, and one to the right, and the polygon interior is above or below.
    /// PERF: Distinguish upper- and lower-chain regular vertices at this point already
    Regular,

    /// If two vertices are parallel to the sweep line we cannot say whether
    /// the vertex is a regular, start, split, end or merge vertex.
    /// However, the algorithm can usually later classify this based on the sweep line status
    /// and assign StartLate, SplitLate, EndLate or MergeLate.
    Undecisive,
}

impl VertexType {
    /// Calculate the vertex type based on the previous, current and next vertices.
    /// This is not exact, since we cannot detect Starts and Ends when the y-coordinate is the same.
    /// In those cases, they will be detected as regular vertices and the sweep line will fix this later.
    pub fn classify<V: IndexType, Vec2: Vector2D>(
        prev: Vec2,
        here: Vec2,
        next: Vec2,
        tol: Vec2::S,
    ) -> Self {
        let cross = (here - prev).perp_dot(&(next - here));

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
                // you might assume that this can only be a start or split vertex, but
                // "numerical_hell_6" is an example where this is in fact a merge vertex
                VertexType::Undecisive
            }
        } else if is_below_prev && is_below_next {
            if cross > tol {
                VertexType::End
            } else if cross < -tol {
                VertexType::Merge
            } else {
                VertexType::Undecisive
            }
        } else {
            if (!is_above_next && !is_below_next)
                || (!is_above_prev && !is_below_prev)
                || cross.abs() <= tol * 2.0.into()
            {
                VertexType::Undecisive
            } else {
                VertexType::Regular
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "bevy")]
mod tests {
    use bevy::math::Vec2;

    use super::*;

    #[test]
    fn detect_vertex_type_start() {
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(-1.0, 0.0),
                f32::EPS
            ),
            VertexType::Start
        );
    }

    fn detect_vertex_type_merge() {
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, -1.0),
                Vec2::new(-1.0, 0.0),
                f32::EPS
            ),
            VertexType::Merge
        );
    }

    fn detect_vertex_type_split() {
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(-1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 0.0),
                f32::EPS
            ),
            VertexType::Split
        );
    }

    fn detect_vertex_type_end() {
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(-1.0, 0.0),
                Vec2::new(0.0, -1.0),
                Vec2::new(1.0, 0.0),
                f32::EPS
            ),
            VertexType::End
        );
    }

    fn detect_vertex_type_regular() {
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(-1.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                f32::EPS
            ),
            VertexType::Regular
        );
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(-1.0, 0.0),
                f32::EPS
            ),
            VertexType::Regular
        );
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, -1.0),
                f32::EPS
            ),
            VertexType::Regular
        );
        assert_eq!(
            VertexType::classify::<usize, Vec2>(
                Vec2::new(0.0, -1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                f32::EPS
            ),
            VertexType::Regular
        );
    }
}
