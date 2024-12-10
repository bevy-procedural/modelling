use crate::mesh::{EdgeBasics, VertexBasics};

use super::MeshType;

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait FaceBasics<T: MeshType<Face = Self>>: std::fmt::Debug + Clone + Copy {
    /// Returns the index of the face.
    fn id(&self) -> T::F;

    /// Returns an edge incident to the face.
    fn edge(&self, mesh: &T::Mesh) -> T::Edge;

    /// Sets the representative edge incident to the face.
    fn set_edge(&mut self, edge: T::E);

    /// Returns the id of a half-edge incident to the face.
    fn edge_id(&self) -> T::E;

    /// Whether the face is allowed to be curved.
    fn may_be_curved(&self) -> bool;

    /// Get the number of edges of the face.
    fn num_edges(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of vertices of the face.
    fn num_vertices(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of triangles of the face. (n-2)*3
    fn num_triangles(&self, mesh: &T::Mesh) -> usize;

    /// Returns the face payload.
    fn payload(&self) -> &T::FP;

    /// Returns a mutable reference to the face payload.
    fn payload_mut(&mut self) -> &mut T::FP;

    /// Iterates all vertices adjacent to the face
    fn vertices<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::Vertex> + 'a + Clone + ExactSizeIterator;

    /// Iterates all vertex ids adjacent to the face
    fn vertex_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::V> + 'a
    where
        T: 'a,
    {
        self.vertices(mesh).map(|v| v.id())
    }

    /// Whether the face has holes.
    /// The data structure (currently!) cannot represent holes, so this is always false.
    fn has_holes(&self) -> bool {
        return false;
    }

    /// Iterates all half-edges incident to the face
    fn edges<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::Edge>;

    /// Iterates all half-edge ids incident to the face
    fn edge_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::E> + 'a
    where
        T: 'a,
    {
        self.edges(mesh).map(|e| e.id())
    }
}
