use super::{ForwardEdgeIterator, HalfEdgeImplMeshType};
use crate::{
    math::IndexType,
    mesh::{
        DefaultFacePayload, EdgeBasics, Face, Face3d, FaceBasics, FacePayload, HalfEdge,
        MeshBasics, MeshType3D,
    },
    util::Deletable,
};

/// A face in a mesh.
///
/// If you want to handle a non-orientable mesh, you have to use double covering.
///
/// Also, if you have inner components, you have to use multiple faces!
#[derive(Clone, Copy, Hash)]
pub struct HalfEdgeFaceImpl<T: HalfEdgeImplMeshType> {
    /// the index of the face
    id: T::F,

    /// a half-edge incident to the face (outer component)
    edge: T::E,

    /// Some user-defined payload
    payload: T::FP,
}

impl<T: HalfEdgeImplMeshType + MeshType3D> Face3d<T> for HalfEdgeFaceImpl<T> {}

impl<T: HalfEdgeImplMeshType> FaceBasics<T> for HalfEdgeFaceImpl<T> {
    #[inline(always)]
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Edge {
        mesh.edge(self.edge)
    }

    #[inline(always)]
    fn edge_id(&self) -> T::E {
        self.edge
    }

    fn may_be_curved(&self) -> bool {
        false
        // TODO
    }

    #[inline(always)]
    fn set_edge(&mut self, edge: T::E) {
        self.edge = edge;
    }

    #[inline(always)]
    fn id(&self) -> T::F {
        self.id
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

    #[inline(always)]
    #[allow(refining_impl_trait)]
    fn edges<'a>(&'a self, mesh: &'a T::Mesh) -> ForwardEdgeIterator<'a, T>
    where
        T: 'a,
    {
        ForwardEdgeIterator::new(self.edge(mesh), mesh)
    }
}

impl<T: HalfEdgeImplMeshType> Face for HalfEdgeFaceImpl<T> {
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

impl<T: HalfEdgeImplMeshType> HalfEdgeFaceImpl<T> {
    /// Creates a new face.
    pub fn new(edge: T::E, payload: T::FP) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            payload,
        }
    }
}

impl<T: HalfEdgeImplMeshType> std::fmt::Debug for HalfEdgeFaceImpl<T> {
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

impl<T: HalfEdgeImplMeshType> Deletable<T::F> for HalfEdgeFaceImpl<T> {
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
            payload: T::FP::allocate(),
        }
    }
}

impl<T: HalfEdgeImplMeshType> Default for HalfEdgeFaceImpl<T>
where
    T::FP: DefaultFacePayload,
{
    /// Creates a deleted face
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            payload: T::FP::default(),
        }
    }
}
