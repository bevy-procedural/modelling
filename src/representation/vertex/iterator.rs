use super::{
    super::{Face, HalfEdge, IndexType, Mesh, Vertex},
    payload::VertexPayload,
};
use crate::representation::MeshType;

impl<E: IndexType, V: IndexType, VP: VertexPayload> Vertex<E, V, VP> {
    /// Iterates all outgoing half-edges incident to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn edges_out<'a, T: MeshType<E = E, V = V, VP = VP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = HalfEdge<E, V, T::F, T::EP>> + 'a {
        IncidentToVertexIterator::new(self.edge(mesh), mesh)
    }

    /// Iterates all ingoing half-edges incident to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn edges_in<'a, T: MeshType<E = E, V = V, VP = VP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = HalfEdge<E, V, T::F, T::EP>> + 'a {
        IncidentToVertexIterator::new(self.edge(mesh), mesh).map(|e| e.twin(mesh))
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn vertices<'a, T: MeshType<E = E, V = V, VP = VP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = Vertex<E, V, T::VP>> + 'a {
        // TODO: slightly inefficient because of the clone and target being indirect
        self.edges_out(mesh).map(|e| e.target(mesh))
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn faces<'a, T: MeshType<E = E, V = V, VP = VP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = Face<E, T::F, T::FP>> + 'a {
        // TODO: slightly inefficient because of the clone
        self.edges_out(mesh)
            .filter_map(|e| e.face(mesh).clone())
    }

    /*
    /// Iterates the wheel of vertices (will have length one if the vertex is manifold)
    #[inline(always)]
    pub fn wheel<'a, T: MeshType<E = E, V = V, VP = VP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> impl Iterator<Item = Vertex<E, V, VP>> + 'a {
        NonmanifoldVertexIterator::new(self.clone(), mesh)
    }*/
}

/// Iterator over all half-edges incident to the same vertex (clockwise)
pub struct IncidentToVertexIterator<'a, T: MeshType> {
    is_first: bool,
    first: T::E,
    current: HalfEdge<T::E, T::V, T::F, T::EP>,
    mesh: &'a Mesh<T>,
}

impl<'a, T: MeshType> IncidentToVertexIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: HalfEdge<T::E, T::V, T::F, T::EP>, mesh: &'a Mesh<T>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: MeshType> Iterator for IncidentToVertexIterator<'a, T> {
    type Item = HalfEdge<T::E, T::V, T::F, T::EP>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current);
        }
        let next = self.current.twin(self.mesh).next(self.mesh);
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
pub struct NonmanifoldVertexIterator<'a, T: MeshType> {
    is_first: bool,
    first: T::V,
    current: Vertex<T::E, T::V, T::VP>,
    mesh: &'a Mesh<T>,
}

impl<'a, T: MeshType> NonmanifoldVertexIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: Vertex<T::E, T::V, T::VP>, mesh: &'a Mesh<T>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: MeshType> Iterator for NonmanifoldVertexIterator<'a, T> {
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