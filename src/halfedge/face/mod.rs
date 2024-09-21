mod iterator;

use super::HalfEdgeMeshType;
use crate::{
    math::{HasPosition, IndexType, Vector3D},
    mesh::{DefaultFacePayload, EdgeBasics, Face, Face3d, FaceBasics, FacePayload, MeshBasics},
    util::Deletable,
};

/// A face in a mesh.
///
/// If you want to handle a non-orientable mesh, you have to use double covering.
///
/// Also, if you have inner components, you have to use multiple faces!
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct HalfEdgeFace<T: HalfEdgeMeshType> {
    /// the index of the face
    id: T::F,

    /// a half-edge incident to the face (outer component)
    edge: T::E,

    /// whether the face is curved, i.e., not planar
    curved: bool,

    /// Some user-defined payload
    payload: T::FP,
}

impl<T: HalfEdgeMeshType> Face3d<T> for HalfEdgeFace<T>
where
    T::Vec: Vector3D<S = T::S>,
    T::VP: HasPosition<T::Vec, S = T::S>,
{
}

impl<T: HalfEdgeMeshType> FaceBasics<T> for HalfEdgeFace<T> {
    #[inline(always)]
    fn edge(&self, mesh: &T::Mesh) -> T::Edge {
        *mesh.edge(self.edge)
    }

    #[inline(always)]
    fn id(&self) -> T::F {
        self.id
    }

    fn may_be_curved(&self) -> bool {
        self.curved
    }

    fn num_edges(&self, mesh: &T::Mesh) -> usize {
        let (min, max) = self.edges(mesh).size_hint();
        assert!(min == max.unwrap());
        min
    }

    fn num_vertices(&self, mesh: &T::Mesh) -> usize {
        FaceBasics::num_edges(self, mesh)
    }

    fn num_triangles(&self, mesh: &T::Mesh) -> usize {
        (FaceBasics::num_vertices(self, mesh) - 2) * 3
    }

    fn payload(&self) -> &T::FP {
        &self.payload
    }

    fn payload_mut(&mut self) -> &mut T::FP {
        &mut self.payload
    }

    #[inline(always)]
    fn vertices<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::Vertex> + 'a + Clone + ExactSizeIterator {
        self.edges(mesh).map(|e| e.target(mesh).clone())
    }
}

impl<T: HalfEdgeMeshType> Face for HalfEdgeFace<T> {
    type T = T;

    fn triangle_touches_boundary(
        &self,
        mesh: &T::Mesh,
        v0: T::V,
        v1: T::V,
        v2: T::V,
    ) -> Option<bool> {
        if let Some(e) = mesh.shared_edge(v0, v1) {
            // it has a common halfedge with another face. That means, it cannot be part of *this* face.
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }
        if let Some(e) = mesh.shared_edge(v1, v2) {
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }
        if let Some(e) = mesh.shared_edge(v2, v0) {
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }

        return None;
    }
}

impl<T: HalfEdgeMeshType> HalfEdgeFace<T> {
    /// Returns the id of a half-edge incident to the face.
    #[inline(always)]
    pub fn edge_id(&self) -> T::E {
        self.edge
    }

    /// Creates a new face.
    pub fn new(edge: T::E, curved: bool, payload: T::FP) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            curved,
            payload,
        }
    }
}

impl<T: HalfEdgeMeshType> std::fmt::Debug for HalfEdgeFace<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{: >w$}) {}",
            self.id().index(),
            self.edge.index(),
            w = 2,
        )
    }
}

impl<T: HalfEdgeMeshType> Deletable<T::F> for HalfEdgeFace<T> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max(), "Face is already deleted");
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: T::F) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
    }

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            curved: false,
            payload: T::FP::allocate(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeFace<T>
where
    T::FP: DefaultFacePayload,
{
    /// Creates a deleted face
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            curved: false,
            payload: T::FP::default(),
        }
    }
}
