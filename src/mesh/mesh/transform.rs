use super::{basics::MeshBasics, MeshType};
use crate::math::Transformable;

/// Methods for transforming meshes.
pub trait MeshTransforms<T: MeshType<Mesh = Self>>: MeshBasics<T> {
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
}
