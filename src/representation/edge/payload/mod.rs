/// A trait that defines how the payload of an edge should behave.
pub trait EdgePayload: Clone + Copy + std::fmt::Debug + PartialEq {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;

    /// Returns true if the payload is empty.
    fn is_empty(&self) -> bool;
}

/// The default edge payload can be safely constructed with a default constructor.
/// For example, when extruding, it is ok for all new edges to have the same default payload.
pub trait DefaultEdgePayload: EdgePayload + Default {}

/// An empty edge payload if you don't need any additional information.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EmptyEdgePayload;

impl EdgePayload for EmptyEdgePayload {
    fn allocate() -> Self {
        Self
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl DefaultEdgePayload for EmptyEdgePayload {}
