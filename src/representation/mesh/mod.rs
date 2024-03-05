use super::{payload::Payload, Deletable, DeletableVector, Face, HalfEdge, IndexType, Vertex};
pub mod builder;
mod check;
pub mod primitives;
mod tesselate;

#[cfg(feature = "bevy")]
pub mod bevy;

/// A mesh data structure for (open) manifold meshes.
///
/// Since coordinates are a variable payload, you can use this mesh for any dimension >= 2.
///
/// Non-manifold edges (multiple faces per edge) are not supported
/// -- use multiple meshes or a "tufted cover".
/// Non-manifold vertices are supported!
///
/// Non-orientable surfaces have to be covered by multiple faces (so they become oriented).
/// The geometry doesn't have to be Euclidean (TODO: But what do we require?).
///
/// TODO: to import non-manifold edges, we could use the "tufted cover" https://www.cs.cmu.edu/~kmcrane/Projects/NonmanifoldLaplace/index.html
#[derive(Debug, Clone)]
pub struct Mesh<EdgeIndex, VertexIndex, FaceIndex, PayloadType>
where
    EdgeIndex: IndexType,
    VertexIndex: IndexType,
    FaceIndex: IndexType,
    PayloadType: Payload,
{
    vertices: DeletableVector<Vertex<EdgeIndex, VertexIndex, PayloadType>, VertexIndex>,
    edges: DeletableVector<HalfEdge<EdgeIndex, VertexIndex, FaceIndex>, EdgeIndex>,
    faces: DeletableVector<Face<EdgeIndex, FaceIndex>, FaceIndex>,
}

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Creates a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            edges: DeletableVector::new(),
            faces: DeletableVector::new(),
        }
    }

    /// Returns a reference to the requested vertex
    pub fn vertex(&self, index: V) -> &Vertex<E, V, P> {
        &self.vertices.get(index)
    }

    /// Returns a reference to the requested edge
    pub fn edge(&self, index: E) -> &HalfEdge<E, V, F> {
        &self.edges.get(index)
    }

    /// Returns the half edge from v to w
    pub fn edge_between(&self, v: V, w: V) -> Option<HalfEdge<E, V, F>> {
        let v = self.vertex(v).edges(self).find(|e| e.target_id(self) == w);
        if let Some(vv) = v {
            if vv.is_deleted() {
                None
            } else {
                Some(vv)
            }
        } else {
            None
        }
    }

    /// Returns the half edge id from v to w. Panics if the edge does not exist.
    pub fn edge_id_between(&self, v: V, w: V) -> E {
        self.edge_between(v, w).unwrap().id()
    }

    /// Returns a reference to the requested face
    pub fn face(&self, index: F) -> &Face<E, F> {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    /// Returns a mutable reference to the requested vertex
    pub fn vertex_mut(&mut self, index: V) -> &mut Vertex<E, V, P> {
        self.vertices.get_mut(index)
    }

    /// Returns a mutable reference to the requested edge
    pub fn edge_mut<'a>(&'a mut self, index: E) -> &'a mut HalfEdge<E, V, F> {
        self.edges.get_mut(index)
    }

    /// Returns a mutable reference to the requested face
    pub fn face_mut(&mut self, index: F) -> &mut Face<E, F> {
        self.faces.get_mut(index)
    }

    /// Whether the mesh is open, i.e., has boundary edges
    pub fn is_open(&self) -> bool {
        self.edges.iter().any(|e| e.is_boundary_self())
    }

    /// Whether the mesh has non-manifold vertices
    pub fn has_nonmanifold_vertices(&self) -> bool {
        self.vertices.iter().any(|v| !v.is_manifold())
    }

    /// Whether the mesh is manifold, i.e., has no boundary edges and no non-manifold vertices
    pub fn is_manifold(&self) -> bool {
        !self.is_open() && !self.has_nonmanifold_vertices()
    }

    /// Returns the number of vertices in the mesh
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the maximum vertex index in the mesh
    pub fn max_vertex_index(&self) -> usize {
        self.vertices.max_ind()
    }

    /// Returns the number of edges in the mesh
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of faces in the mesh
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns an iterator over all non-deleted vertices
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<E, V, P>> {
        self.vertices.iter()
    }

    /// Returns an iterator over all non-deleted edges
    pub fn edges(&self) -> impl Iterator<Item = &HalfEdge<E, V, F>> {
        self.edges.iter()
    }

    /// Returns an iterator over all non-deleted faces
    pub fn faces(&self) -> impl Iterator<Item = &Face<E, F>> {
        self.faces.iter()
    }

    /// Transforms all vertices in the mesh
    pub fn transform(&mut self, t: &P::Trans) {
        for v in self.vertices.iter_mut() {
            v.transform(t);
        }
    }
}
