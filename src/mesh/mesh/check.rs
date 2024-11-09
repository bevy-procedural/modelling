use super::{MeshBasics, MeshType};

/// A trait for checking the consistency of a mesh.
pub trait MeshChecker<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Checks the mesh for consistency
    fn check(&self) -> Result<(), String>;
}
