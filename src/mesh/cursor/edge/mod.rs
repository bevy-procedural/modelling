mod basics;
mod builder;
mod immutable;
mod immutable_valid;
mod mutable;
mod mutable_valid;
mod valid;

pub use basics::*;
pub use builder::*;
pub use immutable::*;
pub use immutable_valid::*;
pub use mutable::*;
pub use mutable_valid::*;
pub use valid::*;

use super::{CursorData, FaceCursorData, VertexCursorData};
use crate::mesh::MeshType;

/// This trait defines the basic functionality for accessing the data fields of an edge cursor.
pub trait EdgeCursorData<'a, T: MeshType>:
    CursorData<T = T, I = T::E, S = T::Edge, Payload = T::EP>
{
    /// The associated vertex cursor type
    type VC: VertexCursorData<
        'a,
        T,
        FC = Self::FC,
        EC = Self::Maybe,
        Payload = T::VP,
        Maybe = Self::VC,
    >;

    /// The associated face cursor type
    type FC: FaceCursorData<
        'a,
        T,
        VC = Self::VC,
        EC = Self::Maybe,
        Payload = T::FP,
        Maybe = Self::FC,
    >;

    /// Derives a new vertex cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new face cursor pointing to the given face id.
    #[must_use]
    fn move_to_face(self, id: T::F) -> Self::FC;

    /// Destructures the cursor into a tuple of the mesh reference and the edge id.
    /// Useful for passing ownership of the mesh reference to other structures.
    #[must_use]
    fn destructure(self) -> (&'a <Self::T as MeshType>::Mesh, Self::I);
}
