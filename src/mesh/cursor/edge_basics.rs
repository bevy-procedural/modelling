use super::{CursorData, FaceCursorData, VertexCursorData};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, EuclideanMeshType, HalfEdge, MeshType},
};

/// This trait defines the basic functionality for accessing the data fields of an edge cursor.
pub trait EdgeCursorData<'a, T: MeshType + 'a>: CursorData<T = T, I = T::E, S = T::Edge> {
    /// The associated vertex cursor type
    type VC: VertexCursorData<'a, T>;

    /// The associated face cursor type
    type FC: FaceCursorData<'a, T>;

    /// Derives a new vertex cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new face cursor pointing to the given face id.
    #[must_use]
    fn move_to_face(self, id: T::F) -> Self::FC;
}

/// This trait implements some basic functionality for edge cursors that works with any type of mesh and both mutable and immutable cursors.
pub trait EdgeCursorBasics<'a, T: MeshType + 'a>: EdgeCursorData<'a, T> {
    /// Moves the cursor to the origin vertex of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn origin(self) -> Self::VC {
        let id = self.map_or(IndexType::max(), |e| e.origin_id(self.mesh()));
        self.move_to_vertex(id)
    }

    /// Moves the cursor to the target vertex of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn target(self) -> Self::VC {
        let id = self.map_or(IndexType::max(), |e| e.target_id(self.mesh()));
        self.move_to_vertex(id)
    }

    /// Returns the id of the origin vertex of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn origin_id(&self) -> T::V {
        self.unwrap().origin_id(self.mesh())
    }

    /// Returns the id of the target vertex of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn target_id(&self) -> T::V {
        self.unwrap().target_id(self.mesh())
    }

    /// Returns the ids of all faces adjacent to the edge
    /// (including the twin for halfedges and parallel edges' faces if the edge is non-manifold).
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn face_ids<'b>(&'b self) -> impl Iterator<Item = T::F> + 'b
    where
        T::Edge: 'b,
        'a: 'b,
    {
        self.unwrap().face_ids(self.mesh())
    }

    /// Whether the edge (or its halfedgetwin) is boundary.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn is_boundary(&self) -> bool {
        self.unwrap().is_boundary(self.mesh())
    }

    /// Whether the edge is manifold.
    /// See [EdgeBasics::is_manifold] for more information.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn is_manifold(&self) -> bool {
        self.unwrap().is_manifold(self.mesh())
    }

    /// Returns the centroid of the edge, i.e., the average of the origin and target vertices.
    #[inline]
    #[must_use]
    fn centroid<const D: usize>(&self) -> T::Vec
    where
        T: EuclideanMeshType<D>,
    {
        self.unwrap().centroid(self.mesh())
    }
}

/// This trait implements some basic functionality for edge cursors that works with halfedge meshes and both mutable and immutable cursors.
pub trait EdgeCursorHalfedgeBasics<'a, T: MeshType + 'a>: EdgeCursorData<'a, T>
where
    T::Edge: HalfEdge<T>,
{
    /// Moves the cursor to the next halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn next(self) -> Self {
        self.try_move(|e| e.next_id())
    }

    /// Moves the cursor by calling next `n` times.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn skip(self, n: usize) -> Self {
        let mut cursor = self;
        for _ in 0..n {
            cursor = cursor.next();
        }
        cursor
    }

    /// Moves the cursor to the previous halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn prev(self) -> Self {
        self.try_move(|e| e.prev_id())
    }

    /// Moves the cursor by calling prev `n` times.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn skip_back(self, n: usize) -> Self {
        let mut cursor = self;
        for _ in 0..n {
            cursor = cursor.prev();
        }
        cursor
    }

    /// Moves the cursor to the twin halfedge of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn twin(self) -> Self {
        self.try_move(|e| e.twin_id())
    }

    /// Returns the id of the next halfedge of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn next_id(&self) -> T::E {
        self.unwrap().next_id()
    }

    /// Returns the id of the previous halfedge of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn prev_id(&self) -> T::E {
        self.unwrap().prev_id()
    }

    /// Returns the id of the twin halfedge of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn twin_id(&self) -> T::E {
        self.unwrap().twin_id()
    }

    /// Moves the cursor to the sibling of the edge, i.e., the twin's next edge.
    /// Calling this repeatedly will return all outgoing halfedges with the same origin.
    /// If the origin is non-manifold, this might not reach all outgoing halfedges but only those in the same wheel.
    /// If you need all wheels, go to the target first. // TODO: Reference
    ///
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn next_sibling(self) -> Self {
        self.twin().next()
    }

    /// Moves the cursor to the previous sibling of the edge, i.e., the previous edge's twin.
    ///
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn prev_sibling(self) -> Self {
        self.prev().twin()
    }

    /// Moves the cursor to the face of the edge.
    /// Won't move if the edge is void.
    #[inline]
    #[must_use]
    fn face(self) -> Self::FC {
        let id = self.map_or(IndexType::max(), |e| e.face_id());
        self.move_to_face(id)
    }

    /// Returns the id of the face of the edge.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn face_id(&self) -> T::F {
        self.unwrap().face_id()
    }

    /// Returns whether the edge has a face.
    /// Panics if the edge is void.
    #[inline]
    #[must_use]
    fn has_face(&self) -> bool {
        self.face_id() != IndexType::max()
    }

    /// Runs some sanity checks on the edge, i.e., whether the origin and target vertices exist.
    /// Returns false if the edge is void or exists and is malformed.
    /// See [Mesh::validate_edge] for more information.
    #[inline]
    #[must_use]
    fn check(&self) -> Result<(), String> {
        self.map_or(Err(format!("Edge {} is invalid", self.try_id())), |e| {
            HalfEdge::check(e, self.mesh())
        })
    }

    /// Returns whether the edge is a boundary edge itself.
    /// Panics if the edge is void.
    /// See [HalfEdge::is_boundary_self] for more information.
    #[inline]
    #[must_use]
    fn is_boundary_self(&self) -> bool {
        self.unwrap().is_boundary_self()
    }

    /// Returns an outgoing edge from `v` that is part of the same boundary as the edge.
    /// Traverses the boundary forwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_boundary] for more information.
    #[inline]
    #[must_use]
    fn same_boundary(self, v: T::V) -> Option<Self> {
        let id = HalfEdge::same_boundary(self.get()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }

    /// Returns an outgoing edge from `v` that is part of the same boundary as the edge.
    /// Traverses the boundary backwards.
    /// Returns `None` if the edge is void or no matching edge was found.
    /// See [HalfEdge::same_boundary_back] for more information.
    #[inline]
    #[must_use]
    fn same_boundary_back(self, v: T::V) -> Option<Self> {
        let id = HalfEdge::same_boundary_back(self.get()?, self.mesh(), v)?;
        Some(self.move_to(id))
    }
}
