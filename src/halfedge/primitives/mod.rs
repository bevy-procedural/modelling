use super::{HalfEdgeMesh, HalfEdgeMeshType};
use crate::{
    mesh::{DefaultEdgePayload, DefaultFacePayload, Halfedge, MeshBasics, MeshBuilder},
    primitives::Make2dShape,
};

impl<T: HalfEdgeMeshType<Mesh = Self>> Make2dShape<T> for HalfEdgeMesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    fn insert_polygon(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        let first = self.insert_loop(vp);
        self.close_hole(first, Default::default(), false);
        self.edge(first).twin_id()
    }

    fn insert_dihedron(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        let first = self.insert_polygon(vp);
        self.close_hole(self.edge(first).twin_id(), Default::default(), false);
        first
    }
}
