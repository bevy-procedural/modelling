use super::{payload::Payload, Deletable, HalfEdge, IndexType, Mesh};
mod geometry;
mod iterator;
pub mod tesselate;

/// A face in a mesh.
///
/// If you want to handle a non-orientable mesh, you have to use double covering.
///
/// Also, if you have inner components, you have to use multiple faces!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face<EdgeIndex, FaceIndex>
where
    EdgeIndex: IndexType,
    FaceIndex: IndexType,
{
    /// the index of the face
    id: FaceIndex,

    /// a half-edge incident to the face (outer component)
    edge: EdgeIndex,

    /// whether the face is curved, i.e., not planar
    curved: bool,
}

impl<E: IndexType, F: IndexType> Face<E, F> {
    /// Returns the index of the face.
    #[inline(always)]
    pub fn id(&self) -> F {
        self.id
    }

    /// Returns a half-edge incident to the face.
    #[inline(always)]
    pub fn edge<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> HalfEdge<E, V, F> {
        *mesh.edge(self.edge)
    }

    /// Returns the id of a half-edge incident to the face.
    #[inline(always)]
    pub fn edge_id(&self) -> E {
        self.edge
    }

    /// Creates a new face.
    pub fn new(edge: E, curved: bool) -> Self {
        assert!(edge != IndexType::max());
        Self {
            id: IndexType::max(),
            edge,
            curved,
        }
    }

    /// Whether the face is allowed to be curved.
    pub fn may_be_curved(&self) -> bool {
        self.curved
    }

    /// Get the number of edges of the face.
    pub fn num_edges<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> usize {
        let (min, max) = self.edges(mesh).size_hint();
        assert!(min == max.unwrap());
        min
    }

    /// Get the number of vertices of the face.
    pub fn num_vertices<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> usize {
        self.num_edges(mesh)
    }

    /// Get the number of triangles of the face. (n-2)*3
    pub fn num_triangles<V: IndexType, P: Payload>(&self, mesh: &Mesh<E, V, F, P>) -> usize {
        (self.num_vertices(mesh) - 2) * 3
    }

    /// Whether a triangle shares a halfedge with the face.
    ///
    /// If there is no evidence that the triangle is touching the face, return None.
    /// Given that all vertices are part of this face, this implies that the triangle is part of the face.
    pub fn triangle_touches_boundary<V: IndexType, P: Payload>(
        &self,
        mesh: &Mesh<E, V, F, P>,
        v0: V,
        v1: V,
        v2: V,
    ) -> Option<bool> {
        if let Some(e) = mesh.edge_between(v0, v1) {
            // it has a common halfedge with another face. That means, it cannot be part of *this* face.
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }
        if let Some(e) = mesh.edge_between(v1, v2) {
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }
        if let Some(e) = mesh.edge_between(v2, v0) {
            if e.face_id() != self.id() {
                return Some(false);
            }
            return Some(!e.is_boundary_self());
        }

        return None;
    }
}

impl<E: IndexType, F: IndexType> std::fmt::Display for Face<E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.id().index(), self.edge.index(),)
    }
}

impl<E: IndexType, F: IndexType> Deletable<F> for Face<E, F> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max(), "Face is already deleted");
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: F) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        self.id = id;
    }
}

impl<E: IndexType, F: IndexType> Default for Face<E, F> {
    /// Creates a deleted face
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
            curved: false,
        }
    }
}
