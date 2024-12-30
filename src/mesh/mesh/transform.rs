use crate::{
    math::Transformable,
    mesh::{EuclideanMeshType, FaceBasics, MeshBasics, VertexBasics},
};
use itertools::Itertools;

/// Methods for transforming meshes.
pub trait TransformableMesh<const D: usize, T: EuclideanMeshType<D, Mesh = Self>>:
    MeshBasics<T>
where
    T::VP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    T::EP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    T::FP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    T::MP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
{
    /// Transforms all vertices in the mesh
    fn transform(&mut self, t: &T::Trans) -> &mut Self {
        for v in self.vertices_mut() {
            v.payload_mut().transform(t);
        }
        for e in self.edges().cloned().collect_vec() {
            self.edge_payload_mut(&e).transform(t);
        }
        for f in self.faces_mut() {
            f.payload_mut().transform(t);
        }
        self.payload_mut().transform(t);
        self
    }

    /// Returns a transformed clone of the mesh
    #[must_use]
    fn transformed(&self, t: &T::Trans) -> Self {
        let mut mesh = self.clone();
        mesh.transform(t);
        mesh
    }

    /// Translates all vertices in the mesh
    fn translate(&mut self, t: &T::Vec) -> &mut Self {
        for v in self.vertices_mut() {
            v.payload_mut().translate(t);
        }
        for e in self.edges().cloned().collect_vec() {
            self.edge_payload_mut(&e).translate(t);
        }
        for f in self.faces_mut() {
            f.payload_mut().translate(t);
        }
        self.payload_mut().translate(t);
        self
    }

    /// Returns a translated clone of the mesh
    #[must_use]
    fn translated(&self, t: &T::Vec) -> Self {
        let mut mesh = self.clone();
        mesh.translate(t);
        mesh
    }

    /// Rotates all vertices in the mesh
    fn rotate(&mut self, rotation: &T::Rot) -> &mut Self {
        for v in self.vertices_mut() {
            v.payload_mut().rotate(rotation);
        }
        for e in self.edges().cloned().collect_vec() {
            self.edge_payload_mut(&e).rotate(rotation);
        }
        for f in self.faces_mut() {
            f.payload_mut().rotate(rotation);
        }
        self.payload_mut().rotate(rotation);
        self
    }

    /// Returns a rotated clone of the mesh
    #[must_use]
    fn rotated(&self, rotation: &T::Rot) -> Self {
        let mut mesh = self.clone();
        mesh.rotate(rotation);
        mesh
    }

    /// Scales all vertices in the mesh
    fn scale(&mut self, scale: &T::Vec) -> &mut Self {
        for v in self.vertices_mut() {
            v.payload_mut().scale(scale);
        }
        for e in self.edges().cloned().collect_vec() {
            self.edge_payload_mut(&e).scale(scale);
        }
        for f in self.faces_mut() {
            f.payload_mut().scale(scale);
        }
        self.payload_mut().scale(scale);
        self
    }

    /// Returns a scaled clone of the mesh
    #[must_use]
    fn scaled(&self, scale: &T::Vec) -> Self {
        let mut mesh = self.clone();
        mesh.scale(scale);
        mesh
    }
}
