mod iterator;
mod topology;

use super::HalfEdgeMeshType;
use crate::{
    mesh::Mesh,
    util::{Deletable, DeletableVector},
};

/// A halfedge-inspired mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are currently not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
///
/// Currently only euclidean geometry is supported.
#[derive(Clone)]
pub struct HalfEdgeMesh<T: HalfEdgeMeshType> {
    // TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
    // TODO: non-euclidean geometry
    vertices: DeletableVector<T::Vertex, T::V>,
    halfedges: DeletableVector<T::Edge, T::E>,
    faces: DeletableVector<T::Face, T::F>,
    payload: T::MP,
}

impl<T: HalfEdgeMeshType> HalfEdgeMesh<T> {
    /// Creates a new empty halfedge mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
            payload: T::MP::default(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeMesh<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HalfEdgeMeshType> Mesh<T> for HalfEdgeMesh<T> {
    fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    fn vertex(&self, index: T::V) -> &T::Vertex {
        self.vertices.get(index)
    }

    fn edge(&self, index: T::E) -> &T::Edge {
        self.halfedges.get(index)
    }

    fn face(&self, index: T::F) -> &T::Face {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    fn vertex_mut(&mut self, index: T::V) -> &mut T::Vertex {
        self.vertices.get_mut(index)
    }

    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut T::Edge {
        self.halfedges.get_mut(index)
    }

    fn face_mut(&mut self, index: T::F) -> &mut T::Face {
        self.faces.get_mut(index)
    }

    fn is_open(&self) -> bool {
        self.halfedges.iter().any(|e| e.is_boundary_self())
    }

    fn max_vertex_index(&self) -> usize {
        self.vertices.capacity()
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn num_edges(&self) -> usize {
        self.halfedges.len()
    }

    fn num_faces(&self) -> usize {
        self.faces.len()
    }

    fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.halfedges.clear();
        self.faces.clear();
        self
    }

    fn payload(&self) -> &T::MP {
        &self.payload
    }

    fn payload_mut(&mut self) -> &mut T::MP {
        &mut self.payload
    }

    /// Returns an iterator over all non-deleted vertices
    fn vertices<'a>(&'a self) -> impl Iterator<Item = &'a T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter()
    }

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T::Vertex: 'a,
    {
        self.vertices.iter_mut()
    }
}

impl<T: HalfEdgeMeshType> std::fmt::Display for HalfEdgeMesh<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*write!(
            f,
            "Mesh:\nvertices:\n{}\n edge --><-- twin   |  face: edge/twin \n{}\n faces: \n{}\n{} ",
            self.vertices()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pair_edges()
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.faces()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            if let Err(msg) = self.check() {
                format!(
                    "⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ERROR ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️ ⚠️\n{}",
                    msg
                )
            } else {
                "".to_string()
            }
        )*/
        todo!("Display not implemented yet");
    }
}
