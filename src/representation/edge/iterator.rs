use crate::representation::payload::Payload;

use super::{super::IndexType, super::Mesh, Edge};

impl<E: IndexType, V: IndexType, F: IndexType> Edge<E, V, F> {
    /// Iterates all half-edges incident to the same face (counter-clockwise)
    #[inline(always)]
    pub fn edges_face<'a, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> IncidentToFaceIterator<'a, E, V, F, P> {
        IncidentToFaceIterator::new(*self, mesh)
    }

    /// Iterates all half-edges incident to the same face (clockwise)
    #[inline(always)]
    pub fn edges_face_back<'a, P: Payload>(
        &'a self,
        mesh: &'a Mesh<E, V, F, P>,
    ) -> IncidentToFaceBackIterator<'a, E, V, F, P> {
        IncidentToFaceBackIterator::new(*self, mesh)
    }

    /// Iterates all half-edges incident to the same face
    /// WARNING: This method is unsafe because it allows mutable access to the mesh! Be careful!
    #[inline(always)]
    pub fn edges_face_mut<'a, P: Payload>(
        &'a self,
        mesh: &'a mut Mesh<E, V, F, P>,
    ) -> IncidentToFaceIteratorMut<'a, E, V, F, P> {
        IncidentToFaceIteratorMut::new(self.id(), mesh)
    }
}

/// Iterator over all half-edges incident to the same face (counter-clockwise)
pub struct IncidentToFaceIterator<'a, E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    is_first: bool,
    first: E,
    current: Edge<E, V, F>,
    mesh: &'a Mesh<E, V, F, P>,
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload>
    IncidentToFaceIterator<'a, E, V, F, P>
{
    /// Creates a new iterator
    pub fn new(first: Edge<E, V, F>, mesh: &'a Mesh<E, V, F, P>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload> Iterator
    for IncidentToFaceIterator<'a, E, V, F, P>
{
    type Item = Edge<E, V, F>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current);
        }
        let next = self.current.next(self.mesh);
        if next.id() == self.first {
            return None;
        } else {
            self.current = next;
            return Some(next);
        }
    }
}

/// Iterator over all half-edges incident to the same face (counter-clockwise)
pub struct IncidentToFaceBackIterator<'a, E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    is_first: bool,
    first: E,
    current: Edge<E, V, F>,
    mesh: &'a Mesh<E, V, F, P>,
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload>
    IncidentToFaceBackIterator<'a, E, V, F, P>
{
    /// Creates a new iterator
    pub fn new(first: Edge<E, V, F>, mesh: &'a Mesh<E, V, F, P>) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload> Iterator
    for IncidentToFaceBackIterator<'a, E, V, F, P>
{
    type Item = Edge<E, V, F>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current);
        }
        let prev = self.current.prev(self.mesh);
        if prev.id() == self.first {
            return None;
        } else {
            self.current = prev;
            return Some(prev);
        }
    }
}

/// Iterator over all half-edges incident to the same face (counter-clockwise)
pub struct IncidentToFaceIteratorMut<'a, E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    is_first: bool,
    first: E,
    current: E,
    mesh: &'a mut Mesh<E, V, F, P>,
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload>
    IncidentToFaceIteratorMut<'a, E, V, F, P>
{
    /// Creates a new iterator
    pub fn new(first: E, mesh: &'a mut Mesh<E, V, F, P>) -> Self {
        Self {
            first,
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, E: IndexType, V: IndexType, F: IndexType, P: Payload> Iterator
    for IncidentToFaceIteratorMut<'a, E, V, F, P>
{
    type Item = &'a mut Edge<E, V, F>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: This unsafe block assumes exclusive access to `self.mesh`
        // throughout the lifetime of the iterator. It is the caller's responsibility
        // to ensure that no other mutable references to `self.mesh` exist during
        // iteration to avoid undefined behavior.
        // TODO: use a different pattern to avoid unsafe
        unsafe {
            if self.is_first {
                self.is_first = false;
                let edge_ptr = self.mesh.edge_mut(self.current) as *mut Edge<E, V, F>;
                return Some(&mut *edge_ptr);
            }
            let next = self.mesh.edge(self.current).next(self.mesh);
            if next.id() == self.first {
                return None;
            } else {
                self.current = next.id();
                let edge_ptr = self.mesh.edge_mut(next.id()) as *mut Edge<E, V, F>;
                return Some(&mut *edge_ptr);
            }
        }
    }
}
