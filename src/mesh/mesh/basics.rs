use crate::mesh::{EdgeBasics, FaceBasics, MeshType, VertexBasics};

/// Some basic operations to retrieve information about the mesh.
pub trait MeshBasics<T: MeshType<Mesh = Self>>: Default + std::fmt::Debug + Clone {
    /// Returns whether the vertex exists and is not deleted
    fn has_vertex(&self, index: T::V) -> bool;

    /// Returns a reference to the requested vertex
    fn vertex(&self, index: T::V) -> &T::Vertex;

    /// Returns a reference to the requested edge
    fn edge(&self, index: T::E) -> &T::Edge;

    /// Returns a reference to the requested face
    fn face(&self, index: T::F) -> &T::Face;

    /// Returns a mutable reference to the requested vertex
    fn vertex_mut(&mut self, index: T::V) -> &mut T::Vertex;

    /// Returns a mutable reference to the requested edge
    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge;

    /// Returns a mutable reference to the requested face
    fn face_mut(&mut self, index: T::F) -> &mut T::Face;

    /// Whether the mesh is open, i.e., has boundary edges
    fn is_open(&self) -> bool;

    /// Returns the maximum vertex index in the mesh
    fn max_vertex_index(&self) -> usize;

    /// Returns the number of vertices in the mesh
    fn num_vertices(&self) -> usize;

    /// Returns the number of edges in the mesh
    fn num_edges(&self) -> usize;

    /// Returns the number of faces in the mesh
    fn num_faces(&self) -> usize;

    /// Clears the mesh (deletes all vertices, edges, and faces)
    fn clear(&mut self) -> &mut Self;

    /// Get the payload of the mesh
    fn payload(&self) -> &T::MP;

    /// Get a mutable reference to the payload of the mesh
    fn payload_mut(&mut self) -> &mut T::MP;

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn get_compact_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP>;

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted vertice's ids
    fn vertex_ids<'a>(&'a self) -> impl Iterator<Item = T::V>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.vertices().map(|v| v.id())
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted edge's ids
    fn edge_ids<'a>(&'a self) -> impl Iterator<Item = T::E>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.edges().map(|e| e.id())
    }

    /// Returns an iterator over all non-deleted faces
    fn faces<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted face's ids
    fn face_ids<'a>(&'a self) -> impl Iterator<Item = T::F>
    where
        T: 'a,
        T::Face: 'a,
    {
        self.faces().map(|f| f.id())
    }

    /// Returns the id of the (half)edge from `v` to `w` or `None` if they are not neighbors.
    fn shared_edge(&self, v: T::V, w: T::V) -> Option<T::Edge>;

    /// Returns the (half)edge id from v to w. Panics if the edge does not exist.
    fn shared_edge_id(&self, v: T::V, w: T::V) -> Option<T::E>;

    /// Returns the face shared by the two vertices or `None`.
    /// TODO: Currently cannot distinguish between holes and "the outside"
    fn shared_face(&self, v0: T::V, v1: T::V) -> Option<T::F>;
}
