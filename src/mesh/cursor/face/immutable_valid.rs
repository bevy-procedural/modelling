use crate::mesh::{cursor::*, FaceBasics, HalfEdge, MeshBasics, MeshType};

/// A face cursor pointing to an existing non-deleted face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct ValidFaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: &'a T::Face,
}

impl<'a, T: MeshType> PartialEq for ValidFaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face.id() == other.face.id() && self.mesh as *const _ == other.mesh as *const _
    }
}

impl<'a, T: MeshType> std::fmt::Debug for ValidFaceCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidFaceCursor({:?})", self.face)
    }
}

impl<'a, T: MeshType> ValidFaceCursor<'a, T> {
    /// Creates a new face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, face: &'a T::Face) -> Self {
        Self { mesh, face }
    }

    /// Creates a new face cursor pointing to the given face.
    /// Panics if the face does not exist in the mesh.
    #[inline]
    #[must_use]
    pub fn load_new(mesh: &'a T::Mesh, face: T::F) -> Self {
        Self::new(mesh, mesh.face_ref(face))
    }
}
impl<'a, T: MeshType> ImmutableCursor for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.face)
    }
}

impl<'a, T: MeshType> FaceCursorData<'a, T> for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    type VC = VertexCursor<'a, T>;
    type EC = EdgeCursor<'a, T>;

    #[inline]
    fn move_to_vertex(self, id: T::V) -> VertexCursor<'a, T> {
        VertexCursor::new(self.mesh, id)
    }

    #[inline]
    fn move_to_edge(self, id: T::E) -> EdgeCursor<'a, T> {
        EdgeCursor::new(self.mesh, id)
    }
}

impl<'a, T: MeshType> CursorData for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    type I = T::F;
    type S = T::Face;
    type T = T;
    type Payload = T::FP;
    type Maybe = FaceCursor<'a, T>;
    type Valid = Self;

    #[inline]
    fn try_id(&self) -> T::F {
        self.face.id()
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::F) -> FaceCursor<'a, T> {
        FaceCursor::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        Some(self)
    }

    #[inline]
    fn try_inner<'b>(&'b self) -> Option<&'b Self::S> {
        Some(self.face)
    }

    #[inline]
    fn maybe(self) -> Self::Maybe {
        FaceCursor::new(self.mesh, self.face.id())
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

impl<'a, T: MeshType> ValidCursor for ValidFaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn id(&self) -> Self::I {
        self.face.id()
    }

    #[inline]
    fn inner<'b>(&'b self) -> &'b Self::S {
        self.face
    }

    #[inline]
    fn payload<'b>(&'b self) -> &'b Self::Payload {
        self.mesh.face_ref(self.try_id()).payload()
    }
}

impl<'a, T: MeshType> ValidFaceCursorBasics<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for ValidFaceCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> ImmutableFaceCursor<'a, T> for ValidFaceCursor<'a, T> where T: 'a {}
