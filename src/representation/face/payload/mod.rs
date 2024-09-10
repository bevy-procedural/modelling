use std::hash::Hash;

/// A trait that defines what data you can store in a face.
pub trait FacePayload: Clone + Copy + PartialEq + Eq + Hash + std::fmt::Debug {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;
}

/// A FacePayload that is safe to be constructed with defaults.
/// For example, when extruding, it is ok for all new faces to have the same default payload.
pub trait DefaultFacePayload: FacePayload + Default {}

/// An empty face payload if you don't need any additional information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EmptyFacePayload;

impl FacePayload for EmptyFacePayload {
    fn allocate() -> Self {
        Self
    }
}

impl DefaultFacePayload for EmptyFacePayload {}
