use super::{payload::Payload, HalfEdge, IndexType, Mesh};
mod iterator;
mod tesselate;

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
    // No! We don't have i
    // a half-edge incident to each inner component of the face
    // inner_components: Vec<EdgeIndex>,
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
    pub fn new(id: F, edge: E) -> Self {
        assert!(id != IndexType::max());
        assert!(edge != IndexType::max());
        Self { id, edge }
    }

    /// Creates a new deleted face.
    pub fn deleted() -> Self {
        Self {
            id: IndexType::max(),
            edge: IndexType::max(),
        }
    }

    /// Returns true if the face is deleted.
    pub fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }
}

impl<E: IndexType, F: IndexType> std::fmt::Display for Face<E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.id().index(), self.edge.index(),)
    }
}
