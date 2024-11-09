//! Payloads for vertices in n-dimensional space.

// TODO: remove the `Default` similar to the `DefaultEdgePayload`
/// A trait that defines how the payload of a vertex should behave.
pub trait VertexPayload: Clone + PartialEq + std::fmt::Debug {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;
}

/// The default vertex payload can be safely constructed with a default constructor.
/// For vertex payloads this is usually not the case when meaningful positions are required.
pub trait DefaultVertexPayload: VertexPayload + Default {}

// TODO: use this whenever it is required for the position to be euclidean.
//pub trait IsEuclidean {}

/// An empty vertex payload if you don't need any vertex information.
/// Notice that your mesh will behave more like a graph without any payload.
// TODO: implement this. Requires the VertexPayload to be weaker and use a separate, stronger trait (e.g., `EuclideanVertexPayload`) for the full payload.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct EmptyVertexPayload;
