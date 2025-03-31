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

use super::{CursorData, EdgeCursorData, VertexCursorData};
use crate::mesh::MeshType;

/// This trait defines the basic functionality for accessing the data fields of a face cursor.
pub trait FaceCursorData<'a, T: MeshType>:
    CursorData<T = T, I = T::F, S = T::Face, Payload = T::FP>
{
    /// The associated vertex cursor type
    type VC: VertexCursorData<
        'a,
        T,
        EC = Self::EC,
        FC = Self::Maybe,
        Payload = T::VP,
        Maybe = Self::VC,
    >;

    /// The associated edge cursor type
    type EC: EdgeCursorData<
        'a,
        T,
        VC = Self::VC,
        FC = Self::Maybe,
        Payload = T::EP,
        Maybe = Self::EC,
    >;

    /// Derives a new vertex cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_vertex(self, id: T::V) -> Self::VC;

    /// Derives a new edge cursor pointing to the given vertex id.
    #[must_use]
    fn move_to_edge(self, id: T::E) -> Self::EC;
}
