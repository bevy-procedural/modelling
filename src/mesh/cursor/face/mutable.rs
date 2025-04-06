use crate::{
    math::IndexType,
    mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
pub struct FaceCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> std::fmt::Debug for FaceCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FaceCursorMut({:?})", self.face)
    }
}

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Creates a new mutable face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a mut T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    /// Converts the mutable cursor to an immutable cursor.
    #[inline]
    #[must_use]
    pub fn into_immutable(self) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, self.try_id())
    }
}

impl<'a, T: MeshType> FaceCursorData<'a, T> for FaceCursorMut<'a, T>
where
    T: 'a,
{
    type VC = VertexCursorMut<'a, T>;
    type EC = EdgeCursorMut<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursorMut<'a, T> {
        VertexCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursorMut<'a, T> {
        EdgeCursorMut::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for FaceCursorMut<'a, T>
where
    T: 'a,
{
    type I = T::F;
    type S = T::Face;
    type T = T;
    type Payload = T::FP;
    type Maybe = Self;
    type Valid = ValidFaceCursorMut<'a, T>;

    #[inline]
    fn try_id(&self) -> T::F {
        self.face
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::F) -> FaceCursorMut<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        if self.is_void() {
            None
        } else {
            Some(ValidFaceCursorMut::new(self.mesh, self.try_id()))
        }
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b T::Face> {
        self.mesh().get_face(self.try_id())
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        self
    }

    #[inline]
    fn from_maybe(from: Self::Maybe) -> Self {
        from
    }

    #[inline]
    fn from_valid(from: Self::Valid) -> Self {
        from.maybe()
    }

    #[inline]
    fn is_void(&self) -> bool {
        self.try_id() == IndexType::max() || !self.mesh().has_face(self.try_id())
    }
}

impl<'a, T: MeshType> FaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType> MutableCursor for FaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> FaceCursorBuilder<'a, T> for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> MaybeCursor for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for FaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for FaceCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
