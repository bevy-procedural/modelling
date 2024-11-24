use crate::{
    math::Transformable,
    mesh::MeshType,
};

/// A trait that defines what data you can store in a face.
pub trait FacePayload: Clone + Copy + PartialEq + Eq + std::fmt::Debug {
    /// Returns a new default instance without any meaningful data.
    fn allocate() -> Self;
}

/// A FacePayload that is safe to be constructed with defaults.
/// For example, when extruding, it is ok for all new faces to have the same default payload.
pub trait DefaultFacePayload: FacePayload + Default {}

/// An empty face payload if you don't need any additional information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EmptyFacePayload<T: MeshType> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: MeshType> FacePayload for EmptyFacePayload<T> {
    fn allocate() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: MeshType> DefaultFacePayload for EmptyFacePayload<T> {}

impl<T: MeshType> Transformable for EmptyFacePayload<T> {
    type Rot = T::Rot;
    type S = T::S;
    type Trans = T::Trans;
    type Vec = T::Vec;

    fn transform(&mut self, _: &T::Trans) -> &mut Self {
        self
    }

    fn lerp(&mut self, _: &Self, _: Self::S) -> &mut Self {
        self
    }
}
