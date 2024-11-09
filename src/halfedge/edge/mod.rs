mod basics;
mod halfedge;
mod iterator;

pub use iterator::*;
pub use basics::*;
pub use halfedge::*;

use super::HalfEdgeMeshType;
use crate::{
    math::IndexType,
    mesh::{DefaultEdgePayload, Edge, EdgeBasics, EdgePayload, Halfedge, MeshBasics},
    util::Deletable,
};

// TODO: Memory alignment?
// TODO: include a way to explicitly access faces around vertex/face? https://en.wikipedia.org/wiki/Polygon_mesh

/// Half-edge inspired data structure
#[derive(Clone, Copy, PartialEq)]
pub struct HalfEdgeImpl<T: HalfEdgeMeshType> {
    /// the index of the half-edge
    id: T::E,

    /// next half-edge incident to the same face
    /// (first edge encountered when traversing around the target vertex in clockwise order).
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    next: T::E,

    /// The other, opposite half-edge.
    /// This will always exist.
    twin: T::E,

    /// The previous half-edge incident to the same face.
    /// This will always exist. If the edge is a boundary, it will wrap around the boundary.
    prev: T::E,

    /// The source vertex of the half-edge.
    /// This will always exist.
    origin_id: T::V,

    /// The face the half-edge is incident to.
    /// The face lies to the left of the half-edge.
    /// Half-edges traverse the boundary of the face in counter-clockwise order.
    /// This index will be FaceIndex.max() if it doesn't exist, i.e., if the edge is a boundary.
    face: T::F,

    /// Some user-defined payload
    payload: T::EP,
}

impl<T: HalfEdgeMeshType> Edge for HalfEdgeImpl<T> {
    type T = T;
}

impl<T: HalfEdgeMeshType> std::fmt::Debug for HalfEdgeImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --{}--> ; twin: {}, face: {} [{}] {}",
            self.origin_id.index(),
            self.id().index(),
            self.twin.index(),
            self.prev.index(),
            if self.face == IndexType::max() {
                "none".to_string()
            } else {
                self.face.index().to_string()
            },
            self.next.index(),
        )?;
        if !self.payload.is_empty() {
            write!(f, ", payload: {:?}", self.payload)?;
        }
        Ok(())
    }
}

impl<T: HalfEdgeMeshType> Deletable<T::E> for HalfEdgeImpl<T> {
    fn delete(&mut self) {
        assert!(self.id != IndexType::max());
        self.id = IndexType::max();
    }

    fn is_deleted(&self) -> bool {
        self.id == IndexType::max()
    }

    fn set_id(&mut self, id: T::E) {
        assert!(self.id == IndexType::max());
        assert!(id != IndexType::max());
        assert!(self.next != id);
        assert!(self.prev != id);
        self.id = id;
    }

    fn allocate() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin_id: IndexType::max(),
            face: IndexType::max(),
            payload: T::EP::allocate(),
        }
    }
}

impl<T: HalfEdgeMeshType> Default for HalfEdgeImpl<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Creates a deleted edge
    fn default() -> Self {
        Self {
            id: IndexType::max(),
            next: IndexType::max(),
            twin: IndexType::max(),
            prev: IndexType::max(),
            origin_id: IndexType::max(),
            face: IndexType::max(),
            payload: T::EP::default(),
        }
    }
}
