use super::{basics::MeshBasics, EuclideanMeshType};
use crate::{
    math::Transformable,
    mesh::{EdgeBasics, FaceBasics, VertexBasics},
};

/// Methods for transforming meshes.
pub trait TransformableMesh<const D: usize, T: EuclideanMeshType<D, Mesh = Self>>:
    MeshBasics<T>
{
    /// Transforms all vertices in the mesh
    fn transform(&mut self, t: &T::Trans) -> &mut Self
    where
        T::VP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::EP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::FP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::MP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.payload_mut().transform(t);
        }
        for e in self.edges_mut() {
            e.payload_mut().transform(t);
        }
        for f in self.faces_mut() {
            f.payload_mut().transform(t);
        }
        self.payload_mut().transform(t);
        self
    }

    /// Translates all vertices in the mesh
    fn translate(&mut self, t: &T::Vec) -> &mut Self
    where
        T::VP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::EP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::FP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::MP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.payload_mut().translate(t);
        }
        for e in self.edges_mut() {
            e.payload_mut().translate(t);
        }
        for f in self.faces_mut() {
            f.payload_mut().translate(t);
        }
        self.payload_mut().translate(t);
        self
    }

    /// Rotates all vertices in the mesh
    fn rotate(&mut self, rotation: &T::Rot) -> &mut Self
    where
        T::VP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::EP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::FP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::MP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.payload_mut().rotate(rotation);
        }
        for e in self.edges_mut() {
            e.payload_mut().rotate(rotation);
        }
        for f in self.faces_mut() {
            f.payload_mut().rotate(rotation);
        }
        self.payload_mut().rotate(rotation);
        self
    }

    /// Scales all vertices in the mesh
    fn scale(&mut self, scale: &T::Vec) -> &mut Self
    where
        T::VP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::EP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::FP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
        T::MP: Transformable<D, Rot = T::Rot, Vec = T::Vec, Trans = T::Trans, S = T::S>,
    {
        for v in self.vertices_mut() {
            v.payload_mut().scale(scale);
        }
        for e in self.edges_mut() {
            e.payload_mut().scale(scale);
        }
        for f in self.faces_mut() {
            f.payload_mut().scale(scale);
        }
        self.payload_mut().scale(scale);
        self
    }
}
