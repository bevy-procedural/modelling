use crate::{
    math::IndexType,
    mesh::{cursor::*, HalfEdge, MeshBasics, MeshType},
};

/// A face cursor pointing to a face of a mesh with an immutable reference to the mesh.
#[derive(Clone, Eq)]
pub struct FaceCursor<'a, T: MeshType> {
    mesh: &'a T::Mesh,
    face: T::F,
}

impl<'a, T: MeshType> PartialEq for FaceCursor<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // same face id and pointing to the same mesh instance
        self.face == other.face && self.mesh as *const _ == other.mesh as *const _
    }
}

impl<'a, T: MeshType> std::fmt::Debug for FaceCursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FaceCursor({:?})", self.face)
    }
}

impl<'a, T: MeshType> FaceCursor<'a, T> {
    /// Creates a new face cursor pointing to the given face.
    #[inline]
    #[must_use]
    pub fn new(mesh: &'a T::Mesh, face: T::F) -> Self {
        Self { mesh, face }
    }

    #[inline]
    #[must_use]
    pub fn new_void(mesh: &'a T::Mesh) -> Self {
        Self {
            mesh,
            face: IndexType::max(),
        }
    }
}

impl<'a, T: MeshType> ImmutableCursor for FaceCursor<'a, T>
where
    T: 'a,
{
    #[inline]
    fn fork(&self) -> Self {
        Self::new(self.mesh, self.face)
    }
}

impl<'a, T: MeshType> FaceCursorData<'a, T> for FaceCursor<'a, T>
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

impl<'a, T: MeshType> CursorData for FaceCursor<'a, T>
where
    T: 'a,
{
    type I = T::F;
    type S = T::Face;
    type T = T;
    type Payload = T::FP;
    type Maybe = Self;
    type Valid = ValidFaceCursor<'a, T>;

    #[inline]
    fn try_id(&self) -> T::F {
        self.face
    }

    #[inline]
    fn mesh<'b>(&'b self) -> &'b T::Mesh {
        self.mesh
    }

    #[inline]
    fn move_to(self, id: T::F) -> FaceCursor<'a, T> {
        Self::new(self.mesh, id)
    }

    #[inline]
    fn load(self) -> Option<Self::Valid> {
        if self.is_void() {
            None
        } else {
            Some(ValidFaceCursor::load_new(self.mesh, self.face))
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

impl<'a, T: MeshType> MaybeCursor for FaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorBasics<'a, T> for FaceCursor<'a, T> where T: 'a {}
impl<'a, T: MeshType> FaceCursorHalfedgeBasics<'a, T> for FaceCursor<'a, T>
where
    T::Edge: HalfEdge<T>,
    T: 'a,
{
}
impl<'a, T: MeshType> ImmutableFaceCursor<'a, T> for FaceCursor<'a, T> where T: 'a {}
