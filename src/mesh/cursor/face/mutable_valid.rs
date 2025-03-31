use crate::mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType};

/// A face cursor pointing to a face of a mesh with a mutable reference to the mesh.
pub struct ValidFaceCursorMut<'a, T: MeshType> {
    mesh: &'a mut T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> std::fmt::Debug for ValidFaceCursorMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidFaceCursorMut({:?})", self.face)
    }
}

impl<'a, T: MeshType> ValidFaceCursorMut<'a, T> {
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

impl<'a, T: MeshType> FaceCursorData<'a, T> for ValidFaceCursorMut<'a, T>
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

impl<'a, T: MeshType> CursorData for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    type I = T::F;
    type S = T::Face;
    type T = T;
    type Payload = T::FP;
    type Maybe = FaceCursorMut<'a, T>;
    type Valid = Self;

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
        FaceCursorMut::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        self.mesh().get_face(self.try_id())
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        FaceCursorMut::new(self.mesh, self.face)
    }

    #[inline]
    fn from_maybe(from: Self::Maybe) -> Self {
        from.load().unwrap()
    }

    #[inline]
    fn from_valid(from: Self::Valid) -> Self {
        from
    }

    #[inline]
    fn is_void(&self) -> bool {
        false
    }
}

impl<'a, T: MeshType> ValidCursor for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.face
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.mesh.get_face(self.face).unwrap()
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.face_ref(self.try_id()).payload()
    }
}

impl<'a, T: MeshType> ValidCursorMut for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn payload_mut<'b>(&'b mut self) -> &'b mut Self::Payload {
        self.mesh.face_ref_mut(self.try_id()).payload_mut()
    }

    #[inline]
    fn inner_mut<'b>(&'b mut self) -> &'b mut Self::S {
        self.mesh.get_face_mut(self.face).unwrap()
    }
}

impl<'a, T: MeshType> MutableCursor for ValidFaceCursorMut<'a, T>
where
    T: 'a,
{
    #[inline]
    fn mesh_mut<'b>(&'b mut self) -> &'b mut <Self::T as MeshType>::Mesh {
        self.mesh
    }
}

impl<'a, T: MeshType> ValidFaceCursorMut<'a, T> {
    /// Updates the representative edge incident to the face in the mesh.
    /// Panics if the vertex is void.
    #[inline]
    pub fn set_edge(&mut self, edge: T::E) {
        self.mesh.face_ref_mut(self.try_id()).set_edge(edge);
    }
}

impl<'a, T: MeshType> FaceCursorBuilder<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> ValidFaceCursorBasics<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for ValidFaceCursorMut<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for ValidFaceCursorMut<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
