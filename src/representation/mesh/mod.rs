use super::{payload::Payload, HalfEdge, Face, IndexType, Vertex};
mod build;
mod check;
pub mod primitives;
mod tesselate;

#[cfg(feature = "bevy")]
mod bevy;

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
    vertices: Vec<Vertex<EdgeIndex, VertexIndex, PayloadType>>,
    edges: Vec<HalfEdge<EdgeIndex, VertexIndex, FaceIndex>>,
    faces: Vec<Face<EdgeIndex, FaceIndex>>,
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
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    /// Returns a reference to the requested vertex
    pub fn vertex(&self, index: V) -> &Vertex<E, V, P> {
        &self.vertices[index.index()]
    }

    /// Returns a reference to the requested edge
    pub fn edge(&self, index: E) -> &HalfEdge<E, V, F> {
        &self.edges[index.index()]
    }

    /// Returns the half edge from v to w
    pub fn edge_between(&self, v: V, w: V) -> Option<HalfEdge<E, V, F>> {
        self.vertex(v).edges(self).find(|e| e.target_id(self) == w)
    }

    /// Returns a reference to the requested face
    pub fn face(&self, index: F) -> &Face<E, F> {
        &self.faces[index.index()]
    }

    /// Returns a mutable reference to the requested vertex
    pub fn vertex_mut(&mut self, index: V) -> &mut Vertex<E, V, P> {
        &mut self.vertices[index.index()]
    }

    /// Returns a mutable reference to the requested edge
    pub fn edge_mut<'a>(&'a mut self, index: E) -> &'a mut HalfEdge<E, V, F> {
        &mut self.edges[index.index()]
    }

    /// Returns a mutable reference to the requested face
    pub fn face_mut(&mut self, index: F) -> &mut Face<E, F> {
        &mut self.faces[index.index()]
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

    /// Returns the number of edges in the mesh
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of faces in the mesh
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Returns an iterator over all vertices
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex<E, V, P>> {
        self.vertices.iter()
    }

    /// Returns an iterator over all edges
    pub fn edges(&self) -> impl Iterator<Item = &HalfEdge<E, V, F>> {
        self.edges.iter()
    }

    /// Returns an iterator over all faces
    pub fn faces(&self) -> impl Iterator<Item = &Face<E, F>> {
        self.faces.iter()
    }
}
