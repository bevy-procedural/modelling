use super::{
    super::{HalfEdge, Face, IndexType, Mesh, Vertex},
    payload::Payload,
};

impl<E: IndexType, V: IndexType, P: Payload> Vertex<E, V, P> {
    /// Iterates all half-edges incident to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn edges<'a, F: IndexType>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> IncidentToVertexIterator<'a, E, V, F, P> {
        IncidentToVertexIterator::new(self.edge(mesh), mesh)
    }

    /// Iterates all vertices adjacent to the vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn vertices<'a, F: IndexType>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = Vertex<E, V, P>> + 'a {
        // TODO: slightly inefficient because of the clone and target being indirect
        self.edges(mesh).map(|e| e.target(mesh))
    }

    /// Iterates all faces adjacent to this vertex in the same manifold edge wheel (clockwise)
    #[inline(always)]
    pub fn faces<'a, F: IndexType>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> impl Iterator<Item = Face<E, F>> + 'a {
        // TODO: slightly inefficient because of the clone
        self.edges(mesh).filter_map(|e| e.face(mesh).clone())
    }

    /// Iterates the wheel of vertices (will have length one if the vertex is manifold)
    #[inline(always)]
    pub fn wheel<'a, F: IndexType>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> NonmanifoldVertexIterator<'a, E, V, F, P> {
        NonmanifoldVertexIterator::new(self.clone(), mesh)
    }
}

/// Iterator over all half-edges incident to the same vertex (clockwise)
pub struct IncidentToVertexIterator<'a, E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    is_first: bool,
    first: E,
    current: HalfEdge<E, V, F>,
    mesh: &'a Mesh<E, V, F, P>,
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload>
    IncidentToVertexIterator<'a, E, V, F, P>
{
    /// Creates a new iterator
    pub fn new(first: HalfEdge<E, V, F>, mesh: &'a Mesh<E, V, F, P>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload> Iterator
    for IncidentToVertexIterator<'a, E, V, F, P>
{
    type Item = HalfEdge<E, V, F>;

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

/// Iterator over all vertices in the same non-manifold vertex wheel
pub struct NonmanifoldVertexIterator<'a, E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    is_first: bool,
    first: V,
    current: Vertex<E, V, P>,
    mesh: &'a Mesh<E, V, F, P>,
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload>
    NonmanifoldVertexIterator<'a, E, V, F, P>
{
    /// Creates a new iterator
    pub fn new(first: Vertex<E, V, P>, mesh: &'a Mesh<E, V, F, P>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload> Iterator
    for NonmanifoldVertexIterator<'a, E, V, F, P>
{
    type Item = Vertex<E, V, P>;

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
