use super::HalfEdgeImpl;
use crate::{
    halfedge::HalfEdgeImplMeshType,
    mesh::{EdgeBasics, HalfEdge, MeshBasics},
};

impl<T: HalfEdgeImplMeshType> HalfEdgeImpl<T> {
    /// Iterates all half-edges incident to the same face
    /// WARNING: This method is unsafe because it allows mutable access to the mesh! Be careful!
    #[inline(always)]
    pub fn edges_face_mut<'a>(&'a self, mesh: &'a mut T::Mesh) -> ForwardEdgeIteratorMut<'a, T> {
        ForwardEdgeIteratorMut::new(self.id(), mesh)
    }
}

/// Follows a chain of half-edges forwards (counter-clockwise) until reaching the start again
#[derive(Clone)]
pub struct ForwardEdgeIterator<'a, T: HalfEdgeImplMeshType + 'a> {
    is_first: bool,
    first: T::E,
    current: HalfEdgeImpl<T>,
    mesh: &'a T::Mesh,
}

impl<'a, T: HalfEdgeImplMeshType> ForwardEdgeIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: HalfEdgeImpl<T>, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for ForwardEdgeIterator<'a, T> {
    type Item = T::Edge;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current.clone());
        }
        let next = self.current.next(self.mesh);
        if next.id() == self.first {
            return None;
        } else {
            self.current = next.clone();
            return Some(next);
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut curr = self.current.clone();
        let mut len = 1;
        while curr.next(self.mesh).id() != self.first {
            len += 1;
            curr = curr.next(self.mesh);
        }
        (len, Some(len))
    }
}

impl<'a, T: HalfEdgeImplMeshType> ExactSizeIterator for ForwardEdgeIterator<'a, T> {}

/// Follows a chain of half-edges backwards (clockwise) until reaching the start again
pub struct BackwardEdgeIterator<'a, T: HalfEdgeImplMeshType + 'a> {
    is_first: bool,
    first: T::E,
    current: HalfEdgeImpl<T>,
    mesh: &'a T::Mesh,
}

impl<'a, T: HalfEdgeImplMeshType> BackwardEdgeIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: HalfEdgeImpl<T>, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for BackwardEdgeIterator<'a, T> {
    type Item = HalfEdgeImpl<T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.current.clone());
        }
        let prev = self.current.prev(self.mesh);
        if prev.id() == self.first {
            return None;
        } else {
            self.current = prev.clone();
            return Some(prev);
        }
    }
}

/// Follows a chain of half-edges forwards (counter-clockwise) until reaching the start again
pub struct ForwardEdgeIteratorMut<'a, T: HalfEdgeImplMeshType + 'a> {
    is_first: bool,
    first: T::E,
    current: T::E,
    mesh: &'a mut T::Mesh,
}

impl<'a, T: HalfEdgeImplMeshType> ForwardEdgeIteratorMut<'a, T> {
    /// Creates a new iterator
    pub fn new(first: T::E, mesh: &'a mut T::Mesh) -> Self {
        Self {
            first,
            current: first,
            mesh,
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for ForwardEdgeIteratorMut<'a, T> {
    type Item = &'a mut HalfEdgeImpl<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: This unsafe block assumes exclusive access to `self.mesh`
        // throughout the lifetime of the iterator. It is the caller's responsibility
        // to ensure that no other mutable references to `self.mesh` exist during
        // iteration to avoid undefined behavior.
        // TODO: use a different pattern to avoid unsafe
        unsafe {
            if self.is_first {
                self.is_first = false;
                let edge_ptr = self.mesh.edge_mut(self.current) as *mut HalfEdgeImpl<T>;
                return Some(&mut *edge_ptr);
            }
            let next = self.mesh.edge(self.current).next(self.mesh);
            if next.id() == self.first {
                return None;
            } else {
                self.current = next.id();
                let edge_ptr = self.mesh.edge_mut(next.id()) as *mut HalfEdgeImpl<T>;
                return Some(&mut *edge_ptr);
            }
        }
    }
}
