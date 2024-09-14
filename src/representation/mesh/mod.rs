pub mod builder;
mod check;
mod iterator;
mod mesh_type;
pub mod primitives;
mod tesselate;
mod normals;
mod topology;
mod transform;

#[cfg(feature = "bevy")]
pub mod bevy;

use super::{Deletable, DeletableVector, Face, HalfEdge, Vertex};
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
#[derive(Debug, Clone)]
pub struct Mesh<T: MeshType> {
    vertices: DeletableVector<Vertex<T::E, T::V, T::VP>, T::V>,
    halfedges: DeletableVector<HalfEdge<T::E, T::V, T::F, T::EP>, T::E>,
    faces: DeletableVector<Face<T::E, T::F, T::FP>, T::F>,
}

impl<T: MeshType> Mesh<T> {
    /// Creates a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
        }
    }

    /// Returns whether the vertex exists and is not deleted
    pub fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    /// Returns a reference to the requested vertex
    pub fn vertex(&self, index: T::V) -> &Vertex<T::E, T::V, T::VP> {
        &self.vertices.get(index)
    }

    /// Returns a reference to the requested edge
    pub fn edge(&self, index: T::E) -> &HalfEdge<T::E, T::V, T::F, T::EP> {
        &self.halfedges.get(index)
    }

    /// Returns a reference to the requested face
    pub fn face(&self, index: T::F) -> &Face<T::E, T::F, T::FP> {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    /// Returns a mutable reference to the requested vertex
    pub fn vertex_mut(&mut self, index: T::V) -> &mut Vertex<T::E, T::V, T::VP> {
        self.vertices.get_mut(index)
    }

    /// Returns a mutable reference to the requested edge
    pub fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut HalfEdge<T::E, T::V, T::F, T::EP> {
        self.halfedges.get_mut(index)
    }

    /// Returns a mutable reference to the requested face
    pub fn face_mut(&mut self, index: T::F) -> &mut Face<T::E, T::F, T::FP> {
        self.faces.get_mut(index)
    }

    /// Whether the mesh is open, i.e., has boundary edges
    pub fn is_open(&self) -> bool {
        self.halfedges.iter().any(|e| e.is_boundary_self())
    }

    /// Returns the maximum vertex index in the mesh
    pub fn max_vertex_index(&self) -> usize {
        self.vertices.capacity()
    }

    /// Returns the number of vertices in the mesh
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges in the mesh
    pub fn num_edges(&self) -> usize {
        self.halfedges.len()
    }

    /// Returns the number of faces in the mesh
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Clears the mesh (deletes all vertices, edges, and faces)
    pub fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.halfedges.clear();
        self.faces.clear();
        self
    }
}
