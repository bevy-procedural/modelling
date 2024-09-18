//mod check;
mod mesh_type;
//mod normals;
mod payload;

pub use mesh_type::*;
pub use payload::*;

use crate::{
    math::{HasPosition, Transformable, Vector3D, VectorIteratorExt},
    tesselate::{triangulate_face, TesselationMeta, Triangulation, TriangulationAlgorithm},
};

use super::{Face3d, Vertex};

/// The `Mesh` trait doesn't assume any specific data structure or topology.
pub trait Mesh<T: MeshType<Mesh = Self>>: Default + std::fmt::Display + Clone {
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
        T: 'a;

    /// Returns an mutable iterator over all non-deleted vertices
    fn vertices_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T::Vertex>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted halfedge pairs without duplicates
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a T::Edge>
    where
        T: 'a;

    /// Returns an iterator over all non-deleted faces
    fn faces<'a>(&'a self) -> impl Iterator<Item = &'a T::Face>
    where
        T: 'a;

    /// Transforms all vertices in the mesh
    fn transform(&mut self, t: &T::Trans) -> &mut Self
    where
        T::VP: Transformable<Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.transform(t);
        }
        self
    }

    /// Translates all vertices in the mesh
    fn translate(&mut self, t: &T::Vec) -> &mut Self
    where
        T::VP: Transformable<Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.translate(t);
        }
        self
    }

    /// Rotates all vertices in the mesh
    fn rotate(&mut self, rotation: &T::Rot) -> &mut Self
    where
        T::VP: Transformable<Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.rotate(rotation);
        }
        self
    }

    /// Scales all vertices in the mesh
    fn scale(&mut self, scale: &T::Vec) -> &mut Self
    where
        T::VP: Transformable<Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.scale(scale);
        }
        self
    }

    /// Returns the mean of all vertex positions.
    fn centroid(&self) -> T::Vec
    where
        T::VP: HasPosition<T::Vec, S = T::S>,
    {
        self.vertices().map(|v| v.pos()).stable_mean()
    }

    /// Since the vertex payloads in the `Deletable` can be sparse,
    /// we need to compact the vertices when converting them to a dense vector.
    /// This function returns the cloned compact vertices and maps the indices to the new compact buffer.
    fn get_compact_vertices(&self, indices: &mut Vec<T::V>) -> Vec<T::VP>;

    /// convert the mesh to triangles and get all indices to do so.
    /// Compact the vertices and return the indices
    fn triangulate(
        &self,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<T::V>,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S>,
        T::Face: Face3d<T>,
    {
        let mut indices = Vec::new();
        for f in self.faces() {
            let mut tri = Triangulation::new(&mut indices);
            triangulate_face::<T>(f, self, &mut tri, algorithm, meta)

            // TODO debug_assert!(tri.verify_full());
        }

        let vs = self.get_compact_vertices(&mut indices);
        (indices, vs)
    }
}
