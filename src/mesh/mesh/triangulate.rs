use std::collections::HashMap;

use super::{MeshBasics, MeshType, MeshType3D};
use crate::{
    math::{HasNormal, IndexType, Vector},
    mesh::{Face3d, FaceBasics, Triangulation, VertexBasics},
    tesselate::{triangulate_face, TriangulationAlgorithm},
};

/// Methods for transforming meshes.
pub trait Triangulateable<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// convert the mesh to triangles and get all indices to do so.
    /// Compact the vertices and return the indices
    fn triangulate(&self, algorithm: TriangulationAlgorithm) -> (Vec<T::V>, Vec<T::VP>)
    where
        T: MeshType3D,
    {
        let mut indices = Vec::new();
        for f in self.face_refs() {
            let mut tri = Triangulation::new(&mut indices);
            triangulate_face::<T>(f, self, &mut tri, algorithm)

            // TODO debug_assert!(tri.verify_full());
        }

        let vs = self.dense_vertices(&mut indices);
        (indices, vs)
    }

    /// Triangulates the mesh and duplicates the vertices for use with flat normals.
    /// This doesn't duplicate the halfedge mesh but only the exported vertex buffer.
    fn triangulate_and_generate_flat_normals_post(
        &self,
        algorithm: TriangulationAlgorithm,
    ) -> (Vec<T::V>, Vec<T::VP>)
    where
        T: MeshType3D,
        T::VP: HasNormal<3, T::Vec, S = T::S>,
    {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for f in self.face_refs() {
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
            triangulate_face::<T>(f, self, &mut tri, algorithm);
            tri.map_indices(&id_map);
        }

        (indices, vertices)
    }
}
