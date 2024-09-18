use super::MeshType;

/// A trait representing a payload for a mesh.
///
/// This could be used to associate the mesh with additional global data,
/// such as a spatial data structure for finding vertices etc.
pub trait MeshPayload<T: MeshType>: Default + Clone + PartialEq + std::fmt::Debug {}

/// An empty mesh payload that can be used when no additional data is needed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyMeshPayload;

impl<T: MeshType> MeshPayload<T> for EmptyMeshPayload {}
