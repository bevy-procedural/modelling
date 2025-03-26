use super::{
    CursorData, EdgeCursorHalfedgeBasics, EdgeCursorMut, FaceCursorMut, VertexCursor, VertexCursorBasics, VertexCursorData, VertexCursorHalfedgeBasics
};
use crate::{
    math::IndexType,
    mesh::{EdgeBasics, HalfEdge, HalfEdgeVertex, MeshBasics, MeshBuilder, MeshType, VertexBasics},
};

/// A vertex cursor pointing to a vertex of a mesh with a mutable reference to the mesh.
pub struct VertexCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    vertex: T::V,
}

impl<'a, T: MeshType> std::fmt::Debug for VertexCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexCursorMut({:?})", self.vertex)
    }
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    /// Creates a new mutable vertex cursor pointing to the given vertex.
    pub fn new(mesh: &'a mut T::Mesh, vertex: T::V) -> Self {
        Self { mesh, vertex }
    }

    /// Creates a new void vertex cursor.
    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a mut T::Mesh) -> Self {
        Self::new(mesh, IndexType::max())
    }

    /// Returns a mutable reference to the payload of the vertex.
    /// Panics if the vertex is void.
    #[inline]
    #[must_use]
    pub fn payload(&mut self) -> &mut T::VP {
        VertexBasics::payload_mut(self.mesh.vertex_ref_mut(self.try_id()))
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, self.try_id())
    }

    /// Appends multiple edges to the current vertex given by the iterator.
    /// Each edge payload will be used for the edge leading to the given vertex payload.
    ///
    /// Moves the cursor to the last inserted vertex.
    /// If the iterator is empty, don't move the cursor.
    /// Panics if the cursor is void.
    ///
    #[inline]
    pub fn append_path(self, iter: impl IntoIterator<Item = (T::EP, T::VP)>) -> Self {
        if let Some((_first_e, last_e)) = self.mesh.append_path(self.id(), iter) {
            let id = self.mesh.edge_ref(last_e).target_id(self.mesh());
            self.move_to(id)
        } else {
            self
        }
    }

    /// Inserts a new vertex and a edge connecting the current vertex and the new vertex.
    /// Move to the new vertex.
    ///
    /// Returns `None` if the insertion wasn't successful.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_vertex_v] for more information.
    #[inline]
    #[must_use]
    pub fn insert_vertex(self, vp: T::VP, ep: T::EP) -> Option<Self> {
        let (_e, v) = self.mesh.insert_vertex_v(self.id(), vp, ep)?;
        Some(self.move_to(v))
    }

    /// Connects the current vertex and the given vertex with an edge.
    ///
    /// Moves the cursor to the newly inserted edge.
    /// Returns `None` if the connectivity wasn't clear.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_edge_vv] for more information.
    #[inline]
    #[must_use]
    pub fn connect(self, target: T::V, ep: T::EP) -> Option<EdgeCursorMut<'a, T>> {
        let e = self.mesh.insert_edge_vv(self.id(), target, ep)?;
        Some(self.move_to_edge(e))
    }

    /// Connects the current vertex and the given edge's origin with an edge.
    ///
    /// Moves the cursor to the newly inserted edge leading from the current vertex to the edge's origin.
    /// Returns `None` if the connectivity wasn't clear.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::insert_edge_ev] for more information.
    #[inline]
    #[must_use]
    pub fn connect_e(self, target: T::E, ep: T::EP) -> Option<EdgeCursorMut<'a, T>>
    where
        T::Edge: HalfEdge<T>,
    {
        let target_twin = self.mesh.edge_ref(target).twin_id();
        let e = self.mesh.insert_edge_ev(target_twin, self.id(), ep)?;
        Some(self.move_to_edge(e).twin())
    }

    /// Subdivide the given face by inserting an edge from the current vertex to the given vertex.
    ///
    /// Moves the cursor to the new edge. The new face will be that edge's face.
    /// Returns `None` if the resulting faces would've been invalid or the vertices don't share the given face.
    /// Panics if the cursor is void.
    ///
    /// See [MeshBuilder::subdivide_face_v] for more information.
    #[inline]
    #[must_use]
    pub fn subdivide_face(
        self,
        f: T::F,
        other: T::V,
        ep: T::EP,
        fp: T::FP,
    ) -> Option<EdgeCursorMut<'a, T>> {
        let e = self.mesh.subdivide_face_v(f, self.id(), other, ep, fp)?;
        Some(self.move_to_edge(e))
    }
}

impl<'a, T: MeshType> VertexCursorData<'a, T> for VertexCursorMut<'a, T> {
    type EC = EdgeCursorMut<'a, T>;
    type FC = FaceCursorMut<'a, T>;

    #[inline]
    fn move_to_face(self, id: T::F) -> FaceCursorMut<'a, T> {
        FaceCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for VertexCursorMut<'a, T> {
    type I = T::V;
    type S = T::Vertex;
    type T = T;

    #[inline]
    fn try_id(&self) -> T::V {
        self.vertex
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::V) -> VertexCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_vertex(self.try_id())
    }

    #[inline]
    fn inner<'b>(&'b self) -> Option<&'b T::Vertex> {
        self.mesh().get_vertex(self.try_id())
    }
}

impl<'a, T: MeshType> VertexCursorMut<'a, T> {
    /// Updates the representative edge incident to the vertex in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E)
    where
        T::Edge: HalfEdge<T>,
        T::Vertex: HalfEdgeVertex<T>,
    {
        self.mesh.vertex_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType> VertexCursorBasics<'a, T> for VertexCursorMut<'a, T> {}
impl<'a, T: MeshType> VertexCursorHalfedgeBasics<'a, T> for VertexCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T::Vertex: HalfEdgeVertex<T>,
{
}
