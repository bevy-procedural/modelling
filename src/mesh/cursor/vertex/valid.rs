use crate::mesh::{cursor::*, HalfEdge, HalfEdgeVertex, MeshType};

pub trait ValidVertexCursorBasics<'a, T: MeshType>: VertexCursorData<'a, T> + ValidCursor {
    fn shortest_path(self, other: T::V) -> Option<(T::E, T::E, usize)>
    where
        T::Edge: HalfEdge<T>,
        Self::S: HalfEdgeVertex<T>,
    {
        self.inner().shortest_path(self.mesh(), other)
    }
}
