use super::FaceCursorData;
use crate::{
    mesh::{MeshBuilder, MeshType},
    prelude::MutableCursor,
};

/// This trait implements some shorthands to quickly modify a mesh without thinking about local variables,
/// i.e., you can quickly modify the mesh multiple times and change the face etc. using a chaining syntax.
pub trait FaceCursorBuilder<'a, T: MeshType>:
    FaceCursorData<'a, T> + MutableCursor<T = T, I = T::F, S = T::Face>
where
    T::Mesh: MeshBuilder<T>,
{
    /// Removes the face the cursor is pointing to.
    /// Returns an empty cursor if the face was removed successfully or didn't exist.
    /// Returns the same cursor if the face couldn't be removed and still exists.
    #[inline]
    fn remove(self) -> Self {
        if self.mesh.try_remove_face(self.face) {
            self.void()
        } else if self.mesh.has_face(self.face) {
            self
        } else {
            self.void()
        }
    }
}
