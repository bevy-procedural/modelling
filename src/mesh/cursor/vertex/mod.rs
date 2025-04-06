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

use super::{CursorData, EdgeCursorData, FaceCursorData};
use crate::mesh::MeshType;

/// This trait defines the basic functionality for accessing the data fields of a vertex cursor.
pub trait VertexCursorData<'a, T: MeshType>:
    CursorData<T = T, I = T::V, S = T::Vertex, Payload = T::VP>
{
    /// The associated face cursor type
    type FC: FaceCursorData<
        'a,
        T,
        EC = Self::EC,
        VC = Self::Maybe,
        Payload = T::FP,
        Maybe = Self::FC,
    >;

    /// The associated edge cursor type
    type EC: EdgeCursorData<
        'a,
        T,
        FC = Self::FC,
        VC = Self::Maybe,
        Payload = T::EP,
        Maybe = Self::EC,
    >;

    /// Derives a new face cursor pointing to the given face id.
    #[must_use]
    fn move_to_face(self, id: T::F) -> Self::FC;

    /// Derives a new edge cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_edge(self, id: T::E) -> Self::EC;

    /// Destructures the cursor into a tuple of the mesh reference and the vertex id.
    /// Useful for passing ownership of the mesh reference to other structures.
    #[must_use]
    fn destructure(self) -> (&'a <Self::T as MeshType>::Mesh, Self::I);
}
