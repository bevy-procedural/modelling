use super::{basics::MeshBasics, MeshType};
use crate::{
    math::{HasNormal, HasPosition, Vector, Vector3D, VectorIteratorExt},
    mesh::{Face3d, FaceBasics, VertexBasics},
};
use std::collections::HashMap;

/// The algorithm to use for generating normals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenerateNormals {
    /// Do not generate normals. (no vertex duplication)
    None,

    /// Generate flat normals per face. (full vertex duplication)
    #[default]
    Flat,

    /// Generate only smooth normals. (no vertex duplication)
    Smooth,
    // Use face groups to decide whether to generate flat or smooth normals.
    //Groups,
}

/// Methods to work with normals in a mesh.
pub trait WithNormals<T: MeshType<Mesh = Self>>: MeshBasics<T> {
    /// Generates flat normals and safes them in the mesh.
    /// Requires all vertices in the mesh to be duplicated.
    /// TODO: Implement this function and also the duplication methods.
    fn generate_flat_normals(&mut self) -> &mut Self {
        todo!("generate_normals_flat is not implemented yet");
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
