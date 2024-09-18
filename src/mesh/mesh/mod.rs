//mod check;
mod mesh_type;
mod normals;
mod payload;

use std::collections::HashMap;

pub use mesh_type::*;
pub use payload::*;
pub use normals::*;

use super::{Face, Face3d, Vertex};
use crate::{
    math::{HasNormal, HasPosition, IndexType, Transformable, Vector, Vector3D, VectorIteratorExt},
    tesselate::{triangulate_face, TesselationMeta, Triangulation, TriangulationAlgorithm},
};

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

    /// Generates flat normals and safes them in the mesh.
    /// Requires all vertices in the mesh to be duplicated.
    /// TODO: Implement this function and also the duplication methods.
    fn generate_flat_normals(&mut self) -> &mut Self {
        todo!("generate_normals_flat is not implemented yet");
    }

    /// Triangulates the mesh and duplicates the vertices for use with flat normals.
    /// This doesn't duplicate the halfedge mesh but only the exported vertex buffer.
    fn triangulate_and_generate_flat_normals_post(
        &self,
        algorithm: TriangulationAlgorithm,
        meta: &mut TesselationMeta<T::V>,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S> + HasNormal<T::Vec, S = T::S>,
        T::Face: Face3d<T>,
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for f in self.faces() {
            let mut tri = Triangulation::new(&mut indices);
            let face_normal = Face3d::normal(f, self).normalize();
            let mut id_map = HashMap::new();
            // generate a new list of vertices (full duplication)
            f.vertices(self).for_each(|v| {
                let mut p = v.payload().clone();
                id_map.insert(v.id(), IndexType::new(vertices.len()));
                p.set_normal(face_normal);
                vertices.push(p)
            });
            triangulate_face::<T>(f, self, &mut tri, algorithm, meta);
            tri.map_indices(&id_map);
        }

        (indices, vertices)
    }

    /// Generates smooth normals and safes them in the mesh.
    fn generate_smooth_normals(&mut self) -> &mut Self
    where
        T::Vec: Vector3D<S = T::S>,
        T::VP: HasPosition<T::Vec, S = T::S> + HasNormal<T::Vec, S = T::S>,
        T::Face: Face3d<T>,
    {
        // Smooth normals are calculated without vertex duplication.
        // Hence, we have to set the normals of the whole mesh.
        // we copy the vertices still to both compact the indices and set the normals without mutating the mesh
        let face_normals: HashMap<T::F, T::Vec> = self
            .faces()
            .map(|f| (f.id(), Face3d::normal(f, self).normalize()))
            .collect();

        let normals = self
            .vertices()
            .map(|v| {
                v.faces(self)
                    .map(|f| face_normals[&f.id()])
                    .stable_mean()
                    .normalize()
            })
            .collect::<Vec<_>>();

        self.vertices_mut().enumerate().for_each(|(i, v)| {
            // set the average of face normals for each vertex
            v.payload_mut().set_normal(normals[i]);
        });

        self
    }
}
