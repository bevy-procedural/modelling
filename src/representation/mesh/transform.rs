use crate::representation::HalfEdge;

use super::{Mesh, MeshType};

impl<T: MeshType> Mesh<T> {
    /// Transforms all vertices in the mesh
    pub fn transform(&mut self, t: &T::Trans) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.transform(t);
        }
        self
    }

    /// Translates all vertices in the mesh
    pub fn translate(&mut self, t: &T::Vec) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.translate(t);
        }
        self
    }

    /// Rotates all vertices in the mesh
    pub fn rotate(&mut self, rotation: &T::Quat) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.rotate(rotation);
        }
        self
    }

    /// Scales all vertices in the mesh
    pub fn scale(&mut self, scale: &T::Vec) -> &mut Self {
        for v in self.vertices.iter_mut() {
            v.scale(scale);
        }
        self
    }

    /// Flips the edge, i.e., swaps the origin and target vertices.
    pub fn flip_edge(&mut self, e: T::E) -> &mut Self {
        HalfEdge::flip(e, self);
        self
    }

    /// Flip all edges (and faces) turning the mesh inside out.
    pub fn flip(&mut self) -> &mut Self {
        // TODO: this is an unnecessary clone
        let ids: Vec<T::E> = self.edges().map(|(e, _)| e.id()).collect();
        ids.iter().for_each(|&e| {
            self.flip_edge(e);
        });
        self
    }
}
