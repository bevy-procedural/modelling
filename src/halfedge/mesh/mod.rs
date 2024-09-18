mod iterator;

use crate::mesh::MeshType;

pub struct HalfEdgeMesh<T: MeshType> {
    vertices: DeletableVector<Vertex>,
    halfedges: DeletableVector<HalfEdge>,
    faces: DeletableVector<Face>,
    payload: MeshPayload,
}

impl<T: MeshType> HalfEdgeMesh<T> {
    pub fn new() -> Self {
        Self {
            vertices: DeletableVector::new(),
            halfedges: DeletableVector::new(),
            faces: DeletableVector::new(),
            payload: MeshPayload::default(),
        }
    }
}

impl<T: MeshType> Default for HalfEdgeMesh<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: MeshType> Mesh<T> for HalfEdgeMesh<T> {
    fn has_vertex(&self, index: T::V) -> bool {
        self.vertices.has(index)
    }

    fn vertex(&self, index: T::V) -> &Vertex<T::E, T::V, T::VP> {
        &self.vertices.get(index)
    }

    fn edge(&self, index: T::E) -> &HalfEdge<T::E, T::V, T::F, T::EP> {
        &self.halfedges.get(index)
    }

    fn face(&self, index: T::F) -> &Face<T::E, T::F, T::FP> {
        let f = &self.faces.get(index);
        assert!(!f.is_deleted());
        f
    }

    fn vertex_mut(&mut self, index: T::V) -> &mut Vertex<T::E, T::V, T::VP> {
        self.vertices.get_mut(index)
    }

    fn edge_mut<'a>(&'a mut self, index: T::E) -> &'a mut HalfEdge<T::E, T::V, T::F, T::EP> {
        self.halfedges.get_mut(index)
    }

    fn face_mut(&mut self, index: T::F) -> &mut Face<T::E, T::F, T::FP> {
        self.faces.get_mut(index)
    }

    fn is_open(&self) -> bool {
        self.halfedges.iter().any(|e| e.is_boundary_self())
    }

    fn max_vertex_index(&self) -> usize {
        self.vertices.capacity()
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn num_edges(&self) -> usize {
        self.halfedges.len()
    }

    fn num_faces(&self) -> usize {
        self.faces.len()
    }

    fn clear(&mut self) -> &mut Self {
        self.vertices.clear();
        self.halfedges.clear();
        self.faces.clear();
        self
    }

    fn payload(&self) -> &T::MP {
        &self.payload
    }

    fn payload_mut(&mut self) -> &mut T::MP {
        &mut self.payload
    }
}
