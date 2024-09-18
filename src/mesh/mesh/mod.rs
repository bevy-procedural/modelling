//mod check;
//mod geometry;
mod mesh_type;
//mod normals;
mod payload;
//mod tesselate;
//mod topology;
//mod transform;

pub use mesh_type::MeshType;

/// A mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
/// The geometry doesn't have to be Euclidean (TODO: But what do we require?).
///
/// TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
pub trait Mesh<T: MeshType>: Default + std::fmt::Display + Clone {
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

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T::Vertex: 'a;

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T::Vertex: 'a;

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T::Edge: 'a;

    /// Returns an iterator over all non-deleted faces
    fn faces<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T::Face: 'a;
}
