use super::{FaceCursorData, ValidFaceCursorBasics};
use crate::{
    mesh::{MeshBuilder, MeshType},
    prelude::{MutableCursor, ValidCursorMut},
};
use std::ops::Not;
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
        self.load_move_or_void(|valid, id| valid.mesh_mut().try_remove_face(id).not().then(|| id))
    }
}
