

impl<E: IndexType, V: IndexType, F: IndexType, EP: EdgePayload> HalfEdge<E, V, F, EP> {
    /// Iterates all half-edges incident to the same face (counter-clockwise)
    #[inline(always)]
    pub fn edges_face<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> IncidentToFaceIterator<'a, T> {
        IncidentToFaceIterator::new(*self, mesh)
    }

    /// Iterates all half-edges incident to the same face (clockwise)
    #[inline(always)]
    pub fn edges_face_back<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a Mesh<T>,
    ) -> IncidentToFaceBackIterator<'a, T> {
        IncidentToFaceBackIterator::new(*self, mesh)
    }

    /// Iterates all half-edges incident to the same face
    /// WARNING: This method is unsafe because it allows mutable access to the mesh! Be careful!
    #[inline(always)]
    pub fn edges_face_mut<'a, T: MeshType<E = E, V = V, F = F, EP = EP>>(
        &'a self,
        mesh: &'a mut Mesh<T>,
    ) -> IncidentToFaceIteratorMut<'a, T> {
        IncidentToFaceIteratorMut::new(self.id(), mesh)
    }
}

/// Iterator over all half-edges incident to the same face (counter-clockwise)
#[derive(Clone, Copy)]
pub struct IncidentToFaceIterator<'a, T: MeshType> {
    is_first: bool,
    first: T::E,
    current: HalfEdge<T::E, T::V, T::F, T::EP>,
    mesh: &'a Mesh<T>,
}

impl<'a, T: MeshType> IncidentToFaceIterator<'a, T> {
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

impl<'a, T: MeshType> Iterator for IncidentToFaceIterator<'a, T> {
    type Item = HalfEdge<T::E, T::V, T::F, T::EP>;

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

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut curr = self.current;
        let mut len = 1;
        while curr.next(self.mesh).id() != self.first {
            len += 1;
            curr = curr.next(self.mesh);
        }
        (len, Some(len))
    }
}

impl<'a, T: MeshType> ExactSizeIterator for IncidentToFaceIterator<'a, T> {}

/// Iterator over all half-edges incident to the same face (clockwise)
pub struct IncidentToFaceBackIterator<'a, T: MeshType> {
    is_first: bool,
    first: T::E,
    current: HalfEdge<T::E, T::V, T::F, T::EP>,
    mesh: &'a Mesh<T>,
}

impl<'a, T: MeshType> IncidentToFaceBackIterator<'a, T> {
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

impl<'a, T: MeshType> Iterator for IncidentToFaceBackIterator<'a, T> {
    type Item = HalfEdge<T::E, T::V, T::F, T::EP>;

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
pub struct IncidentToFaceIteratorMut<'a, T: MeshType> {
    is_first: bool,
    first: T::E,
    current: T::E,
    mesh: &'a mut Mesh<T>,
}

impl<'a, T: MeshType> IncidentToFaceIteratorMut<'a, T> {
    /// Creates a new iterator
    pub fn new(first: T::E, mesh: &'a mut Mesh<T>) -> Self {
        Self {
            first,
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: MeshType> Iterator for IncidentToFaceIteratorMut<'a, T> {
    type Item = &'a mut HalfEdge<T::E, T::V, T::F, T::EP>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: This unsafe block assumes exclusive access to `self.mesh`
        // throughout the lifetime of the iterator. It is the caller's responsibility
        // to ensure that no other mutable references to `self.mesh` exist during
        // iteration to avoid undefined behavior.
        // TODO: use a different pattern to avoid unsafe
        unsafe {
            if self.is_first {
                self.is_first = false;
                let edge_ptr =
                    self.mesh.edge_mut(self.current) as *mut HalfEdge<T::E, T::V, T::F, T::EP>;
                return Some(&mut *edge_ptr);
            }
            let next = self.mesh.edge(self.current).next(self.mesh);
            if next.id() == self.first {
                return None;
            } else {
                self.current = next.id();
                let edge_ptr =
                    self.mesh.edge_mut(next.id()) as *mut HalfEdge<T::E, T::V, T::F, T::EP>;
                return Some(&mut *edge_ptr);
            }
        }
    }
}
