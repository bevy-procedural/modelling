use super::{ForwardEdgeIterator, HalfEdgeImplMeshType};
use crate::{
    math::IndexType,
    mesh::{
        cursor::*, DefaultFacePayload, EdgeBasics, EdgeRef2TargetRefAdapter, Face, Face3d,
        FaceBasics, FacePayload, HalfEdge, HasIslands, IslandCircularLinkedList, MeshBasics,
        MeshType3D, Vertex2ValidVertexCursorAdapter,
    },
    util::{CreateEmptyIterator, Deletable},
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

    // The next island in the circular linked list of islands.
    // TODO: make this optional
    next_island: T::F,
}

impl<T: HalfEdgeImplMeshType + MeshType3D> Face3d<T> for HalfEdgeFaceImpl<T> {}

impl<T: HalfEdgeImplMeshType> IslandCircularLinkedList<T> for HalfEdgeFaceImpl<T> {
    #[inline]
    fn next_island(&self) -> T::F {
        self.next_island
    }

    #[inline]
    fn set_next_island(&mut self, island: T::F) {
        self.next_island = island;
    }
}

impl<T: HalfEdgeImplMeshType> FaceBasics<T> for HalfEdgeFaceImpl<T> {
    #[inline]
    fn next_island_helper(&self) -> Option<T::F> {
        assert!(self.next_island != IndexType::max());
        Some(self.next_island)
    }

    #[inline]
    fn edge<'a>(&'a self, mesh: &'a T::Mesh) -> &'a T::Edge {
        mesh.edge_ref(self.edge)
    }

    #[inline]
    fn edge_id(&self) -> T::E {
        self.edge
    }

    #[inline]
    fn is_flat(&self) -> bool {
        true
        // TODO
    }

    #[inline]
    fn set_edge(&mut self, edge: T::E) {
        self.edge = edge;
    }

    #[inline]
    fn id(&self) -> T::F {
        self.id
    }

    #[inline]
    fn num_edges(&self, mesh: &T::Mesh) -> usize {
        let (min, max) = self.edge_refs(mesh).size_hint();
        assert!(min == max.unwrap());
        min
    }

    #[inline]
    fn payload(&self) -> &T::FP {
        &self.payload
    }

    #[inline]
    fn payload_mut(&mut self) -> &mut T::FP {
        &mut self.payload
    }

    /// Iterates references to all vertices adjacent to the face
    fn vertex_refs<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = &'a T::Vertex> + CreateEmptyIterator
    where
        T: 'a,
    {
        EdgeRef2TargetRefAdapter::<'a, T, _>::new(mesh, self.edge_refs(mesh))
    }

    #[inline]
    fn vertices<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = ValidVertexCursor<'a, T>> + CreateEmptyIterator
    where
        T: 'a,
    {
        Vertex2ValidVertexCursorAdapter::new(mesh, self.vertex_refs(mesh))
    }

    #[inline]
    fn vertex_ids<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = T::V> + CreateEmptyIterator
    where
        T: 'a,
    {
        let mapper: fn(ValidVertexCursor<'a, T>) -> T::V = |c| c.id();
        self.vertices(mesh).map(mapper)
    }

    #[inline]
    fn edge_refs<'a>(
        &'a self,
        mesh: &'a T::Mesh,
    ) -> impl Iterator<Item = &'a T::Edge> + CreateEmptyIterator
    where
        T: 'a,
    {
        ForwardEdgeIterator::new(self.edge(mesh), mesh)
    }

    #[inline]
    fn edge_ids<'a>(&'a self, mesh: &'a T::Mesh) -> impl Iterator<Item = T::E> + CreateEmptyIterator
    where
        T: 'a,
    {
        let mapper: fn(&'a T::Edge) -> T::E = |e| e.id();
        self.edge_refs(mesh).map(mapper)
    }

    #[inline]
    fn add_quasi_island(&self, mesh: &mut T::Mesh, island: T::E) -> Option<T::E> {
        self.add_island(mesh, island)
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
    pub fn new(edge: T::E, payload: T::FP, next_island: T::F) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            next_island,
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
        self.next_island = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: T::F) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
        if self.next_island == IndexType::max() {
            self.next_island = id;
        }
    }

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            next_island: IndexType::max(),
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
            next_island: IndexType::max(),
            edge: IndexType::max(),
            payload: T::FP::default(),
        }
    }
}
