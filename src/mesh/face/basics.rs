use crate::{
    mesh::{cursor::ValidVertexCursor, MeshType},
    util::CreateEmptyIterator,
};

/// A face in a mesh.
///
/// Isn't necessarily planar or triangular.
pub trait FaceBasics<T: MeshType<Face = Self>>: std::fmt::Debug + Clone + Copy {
    /// Returns the index of the face.
    #[must_use]
    fn id(&self) -> T::F;

    /// Returns an edge incident to the face.
    #[must_use]
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Edge;

    /// Sets the representative edge incident to the face.
    fn set_edge(&mut self, edge: T::E);

    /// Returns the id of a half-edge incident to the face.
    #[must_use]
    fn edge_id(&self) -> T::E;

    /// Whether the face is flat.
    /// There are additional traits for non-planar faces planned, e.g. `HasTSpline`.
    /// Once the API stabilizes, this should be replaced with a typecheck for the respective traits.
    #[must_use]
    fn is_flat(&self) -> bool;

    /// Get the number of edges of the face.
    #[must_use]
    fn num_edges(&self, mesh: &T::Mesh) -> usize;

    /// Get the number of vertices of the face.
    #[must_use]
    #[inline]
    fn num_vertices(&self, mesh: &T::Mesh) -> usize {
        FaceBasics::num_edges(self, mesh)
    }

    /// Get the number of triangles of the face. (n-2)*3
    #[must_use]
    #[inline]
    fn num_triangles(&self, mesh: &T::Mesh) -> usize {
        (FaceBasics::num_vertices(self, mesh) - 2) * 3
    }

    /// Returns the face payload.
    #[must_use]
    fn payload(&self) -> &T::FP;

    /// Returns a mutable reference to the face payload.
    #[must_use]
    fn payload_mut(&mut self) -> &mut T::FP;

    /// Iterates references to all vertices adjacent to the face.
    /// Ignores islands and holes.
    #[must_use]
    fn vertex_refs<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = &'a T::Vertex> + CreateEmptyIterator
    where
        T: 'a;

    /// Iterates all vertices adjacent to the face
    /// Ignores islands and holes.
    #[must_use]
    fn vertices<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = ValidVertexCursor<'a, T>> + CreateEmptyIterator
    where
        T: 'a;

    /// Iterates all vertex ids adjacent to the face
    /// Ignores islands and holes.
    #[must_use]
    fn vertex_ids<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::V> + CreateEmptyIterator
    where
        T: 'a;

    /// Whether the face has islands/holes.
    ///
    /// If the face doesn't implement `HasIslands`, this will always return false.
    /// Otherwise, it returns true if this particular face has at least one island.
    #[must_use]
    fn has_islands(&self) -> bool {
        let Some(next) = self.next_island_helper() else {
            return false;
        };
        next != self.id()
    }

    /// Returns the next island of the face.
    /// If the face doesn't implement `HasIslands`, this will always return `None`.
    /// If the face has no islands but supports them, it will return its own id.
    /// 
    /// TODO: This should be moved to payloads once the payload API is stable.
    #[must_use]
    fn next_island_helper(&self) -> Option<T::F>;

    /// Iterates all half-edges incident to the face
    /// Ignores islands and holes.
    #[must_use]
    fn edge_refs<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = &'a T::Edge> + CreateEmptyIterator
    where
        T: 'a;

    /// Iterates all half-edge ids incident to the face
    /// Ignores islands and holes.
    #[must_use]
    fn edge_ids<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::E> + CreateEmptyIterator
    where
        T: 'a;

    /// If the face supports islands, add an island.
    ///
    /// If the face doesn't support islands, join the outer edge chain with the island.
    /// Use the shortest diagonal if vertex positions are defined. Validity of the diagonal is not gauranteed.
    #[must_use]
    fn add_quasi_island(&self, mesh: &mut T::Mesh, island: T::E) -> Option<T::E>;
}
