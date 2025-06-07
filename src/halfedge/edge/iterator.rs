use super::HalfEdgeImpl;
use crate::{
    halfedge::HalfEdgeImplMeshType,
    math::IndexType,
    mesh::{cursor::*, EdgeBasics, HalfEdge, MeshBasics},
    util::CreateEmptyIterator,
};

impl<T: HalfEdgeImplMeshType> HalfEdgeImpl<T> {
    /// Iterates all half-edges incident to the same face
    /// WARNING: This method is unsafe because it allows mutable access to the mesh! Be careful!
    #[inline]
    pub fn chain_mut<'a>(&'a self, mesh: &'a mut T::Mesh) -> ForwardEdgeIteratorMut<'a, T> {
        ForwardEdgeIteratorMut::new(self.id(), mesh)
    }
}

/// Follows a chain of half-edges forwards (counter-clockwise) until reaching the start again
#[derive(Clone)]
pub struct ForwardEdgeIterator<'a, T: HalfEdgeImplMeshType>
where
    T::Edge: 'a,
{
    is_first: bool,
    first: T::E,
    current: Option<&'a HalfEdgeImpl<T>>,
    mesh: Option<&'a T::Mesh>,
}

impl<'a, T: HalfEdgeImplMeshType> CreateEmptyIterator for ForwardEdgeIterator<'a, T> {
    fn create_empty() -> Self {
        Self {
            is_first: false,
            first: IndexType::max(),
            current: None,
            mesh: None,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> ForwardEdgeIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: &'a HalfEdgeImpl<T>, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: Some(first),
            mesh: Some(mesh),
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for ForwardEdgeIterator<'a, T>
where
    T::Edge: 'a,
{
    type Item = &'a T::Edge;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return self.current;
        }
        let next = self.current?.next(self.mesh?);
        if next.id() == self.first {
            return None;
        } else {
            self.current = Some(next);
            return Some(next);
        }
    }

    /// Estimates the number of edges in the loop by traversing it.
    /// This is an O(n) operation where n is the number of edges in the face.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(mut curr) = self.current else {
            return (0, Some(0));
        };
        let Some(mesh) = self.mesh else {
            return (0, Some(0));
        };
        let mut len = 1;
        while curr.next(mesh).id() != self.first {
            len += 1;
            curr = curr.next(mesh);
        }
        (len, Some(len))
    }
}

impl<'a, T: HalfEdgeImplMeshType> ExactSizeIterator for ForwardEdgeIterator<'a, T> {}

/// Follows a chain of half-edges backwards (clockwise) until reaching the start again
pub struct BackwardEdgeIterator<'a, T: HalfEdgeImplMeshType + 'a> {
    is_first: bool,
    first: T::E,
    current: Option<&'a HalfEdgeImpl<T>>,
    mesh: Option<&'a T::Mesh>,
}

impl<'a, T: HalfEdgeImplMeshType> CreateEmptyIterator for BackwardEdgeIterator<'a, T> {
    fn create_empty() -> Self {
        Self {
            is_first: false,
            first: IndexType::max(),
            current: None,
            mesh: None,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> BackwardEdgeIterator<'a, T> {
    /// Creates a new iterator
    pub fn new(first: &'a HalfEdgeImpl<T>, mesh: &'a T::Mesh) -> Self {
        Self {
            first: first.id(),
            current: Some(first),
            mesh: Some(mesh),
            is_first: true,
        }
    }
}

impl<'a, T: HalfEdgeImplMeshType> Iterator for BackwardEdgeIterator<'a, T> {
    type Item = &'a HalfEdgeImpl<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return self.current;
        }
        let prev = self.current?.prev(self.mesh?);
        if prev.id() == self.first {
            return None;
        } else {
            self.current = Some(prev);
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

impl<'a, T: HalfEdgeImplMeshType + 'a> Iterator for ForwardEdgeIteratorMut<'a, T> {
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
                let edge_ptr = self.mesh.edge_ref_mut(self.current) as *mut HalfEdgeImpl<T>;
                return Some(&mut *edge_ptr);
            }

            let next = self.mesh.edge(self.current).next();
            let Some(id) = next.id() else {
                return None;
            };
            if id == self.first {
                return None;
            } else {
                self.current = id;
                let edge_ptr = self.mesh.edge_ref_mut(id) as *mut HalfEdgeImpl<T>;
                return Some(&mut *edge_ptr);
            }
        }
    }
}
