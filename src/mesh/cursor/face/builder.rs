use crate::{
    math::{IndexType, Transformable},
    mesh::{
        cursor::*, DefaultEdgePayload, DefaultFacePayload, EuclideanMeshType, FaceBasics,
        HasIslands, MeshBuilder, MeshType, MeshTypeHalfEdge,
    },
    operations::MeshExtrude,
};

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the face etc. using a chaining syntax.
pub trait FaceCursorBuilder<'a, T: MeshType + 'a>:
    FaceCursorData<'a, T> + MutableCursor<T = T, I = T::F, S = T::Face>
where
    T::Mesh: MeshBuilder<T>,
    Self::Valid: FaceCursorData<'a, T, EC = Self::EC, VC = Self::VC>
        + MutableCursor<T = T, I = T::F, S = T::Face>
        + ValidFaceCursorBasics<'a, T>
        + ValidCursorMut<T = T, I = T::F, S = T::Face>,
    Self::Maybe: FaceCursorData<'a, T, EC = Self::EC, VC = Self::VC>
        + MutableCursor<T = T, I = T::F, S = T::Face>,
{
    /// Removes the face the cursor is pointing to.
    /// Returns an empty cursor if the face was removed successfully or didn't exist.
    /// Returns the same cursor if the face couldn't be removed and still exists.
    #[inline]
    fn remove(self) -> Self::Maybe {
        self.load_move_or_void(|valid, id| {
            if let Some(_) = valid.mesh_mut().try_remove_face(id) {
                None
            } else {
                Some(id)
            }
        })
    }

    /// See [MeshExtrude::extrude_face].
    #[inline]
    #[must_use]
    fn extrude<const D: usize>(self, transform: &T::Trans) -> Self::EC
    where
        T: EuclideanMeshType<D> + MeshTypeHalfEdge,
        T::Mesh: MeshExtrude<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
    {
        // TODO: Return valid cursor?
        self.load_or_else(
            |c| c.move_to_edge(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                let e_id = valid
                    .mesh_mut()
                    .extrude_face(id, transform)
                    .unwrap_or(IndexType::max());
                valid.move_to_edge(e_id)
            },
        )
    }

    /// See [MeshExtrude::extrude_tri_face].
    #[inline]
    #[must_use]
    fn extrude_tri<const D: usize>(self, transform: &T::Trans) -> Self::EC
    where
        T: EuclideanMeshType<D> + MeshTypeHalfEdge,
        T::Mesh: MeshExtrude<T>,
        T::FP: DefaultFacePayload,
        T::EP: DefaultEdgePayload,
        T::VP: Transformable<D, Trans = T::Trans, S = T::S>,
    {
        // TODO: Return valid cursor?
        self.load_or_else(
            |c| c.move_to_edge(IndexType::max()),
            |mut valid| {
                let id = valid.id();
                let e_id = valid
                    .mesh_mut()
                    .extrude_tri_face(id, transform)
                    .unwrap_or(IndexType::max());
                valid.move_to_edge(e_id)
            },
        )
    }

    /// See [HasIslands::remove_next_island]
    /// Returns a void cursor if the island didn't exist or was removed successfully.
    /// Returns the same cursor if the island couldn't be removed and still exists.
    #[inline]
    #[must_use]
    fn remove_next_island(self) -> Self::Maybe
    where
        T::Face: HasIslands<T>,
    {
        self.load_move_or_void(|valid, id| {
            if let Some(_) = T::Face::remove_next_island(valid.mesh_mut(), id) {
                None
            } else {
                Some(id)
            }
        })
    }

    /// See [HasIslands::add_island]
    #[inline]
    #[must_use]
    fn add_island(self, _island: T::E) -> Self::EC
    where
        T::Face: HasIslands<T>,
    {
        todo!()
    }

    /// See [FaceBasics::add_quasi_island]
    #[inline]
    #[must_use]
    fn add_quasi_island(self, island: T::E) -> Self::EC {
        self.load_or_else(
            |c| c.move_to_edge(IndexType::max()),
            |mut valid| {
                // PERF: avoid clone
                let inner = valid.inner().clone();
                let e = FaceBasics::add_quasi_island(&inner, valid.mesh_mut(), island)
                    .unwrap_or(IndexType::max());
                valid.move_to_edge(e)
            },
        )
    }

    /// See [HasIslands::remove_island]
    #[inline]
    #[must_use]
    fn remove_island(self, _island: T::E) -> Self::EC
    where
        T::Face: HasIslands<T>,
    {
        todo!()
    }
}
