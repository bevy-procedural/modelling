use crate::{
    halfedge::HalfEdgeImplMeshType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics},
};

/// Iterator over all half-edges incident to the same vertex (clockwise)
pub struct IncidentToVertexIterator<'a, T: HalfEdgeImplMeshType + 'a> {
    is_first: bool,
    first: T::E,
    current: T::Edge,
    mesh: &'a T::Mesh,
}

impl<'a, T: HalfEdgeImplMeshType> IncidentToVertexIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: T::Edge, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for IncidentToVertexIterator<'a, T> {
    type Item = T::Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current);
        }
        let next = self.current.twin(self.mesh).next(self.mesh);
        debug_assert!(
            next.origin_id() == self.mesh.edge(self.first).origin_id(),
            "The edge wheel around vertex {} is not closed. The mesh is invalid.",
            next.origin_id()
        );
        if next.id() == self.first {
            return None;
        } else {
            self.current = next;
            return Some(next);
        }
    }
}

/*
/// Iterator over all vertices in the same non-manifold vertex wheel
pub struct NonmanifoldVertexIterator<'a, T: HalfEdgeMeshType> {
    is_first: bool,
    first: T::V,
    current: Vertex<T::E, T::V, T::VP>,
    mesh: &'a T::Mesh,
}

impl<'a, T: HalfEdgeMeshType> NonmanifoldVertexIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: Vertex<T::E, T::V, T::VP>, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeMeshType> Iterator for NonmanifoldVertexIterator<'a, T> {
    type Item = Vertex<T::E, T::V, T::VP>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            Some(self.current.clone())
        } else {
            if self.current.next == self.first {
                return None;
            }
            // TODO: avoid clone?
            self.current = self.mesh.vertex(self.current.next).clone();
            Some(self.current.clone())
        }
    }
}
*/
