use super::{EuclideanMeshType, MeshType};
use crate::math::Transformable;

/// A trait representing a payload for a mesh.
///
/// This could be used to associate the mesh with additional global data,
/// such as a spatial data structure for finding vertices etc.
pub trait MeshPayload<T: MeshType>: Default + Clone + PartialEq + std::fmt::Debug {}

/// An empty mesh payload that can be used when no additional data is needed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyMeshPayload<T: MeshType> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: MeshType> MeshPayload<T> for EmptyMeshPayload<T> {}

impl<T: MeshType> std::fmt::Display for EmptyMeshPayload<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmptyMeshPayload")
    }
}

impl<const D: usize, T: EuclideanMeshType<D>> Transformable<D> for EmptyMeshPayload<T> {
    type Rot = T::Rot;
    type S = T::S;
    type Trans = T::Trans;
    type Vec = T::Vec;

    fn transform(&mut self, _: &Self::Trans) -> &mut Self {
        self
    }

    fn lerp(&mut self, _: &Self, _: Self::S) -> &mut Self {
        self
    }
}
