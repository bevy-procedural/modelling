use crate::{
    halfedge::{HalfEdgeMesh, HalfEdgeMeshType},
    mesh::{DefaultEdgePayload, EdgeBasics, MeshBasics, MeshBuilder},
};

impl<T: HalfEdgeMeshType> MeshBuilder<T> for HalfEdgeMesh<T> {
    /// Generate a path from the finite iterator of positions and return the halfedges pointing to the first and last vertex.
    fn insert_path(&mut self, vp: impl IntoIterator<Item = T::VP>) -> (T::E, T::E)
    where
        T::EP: DefaultEdgePayload,
    {
        // TODO: create this directly without the builder functions

        let mut iter = vp.into_iter();
        let p0 = iter.next().expect("Path must have at least one vertex");
        let p1 = iter.next().expect("Path must have at least two vertices");
        let (v0, v) = self.add_isolated_edge_default(p0, p1);
        let first = self.shared_edge(v0, v).unwrap();
        let mut input = first.id();
        let mut output = first.twin_id();
        for pos in iter {
            self.add_vertex_via_edge_default(input, output, pos);
            let n = self.edge(input).next(self);
            input = n.id();
            output = n.twin_id();
        }

        (first.twin_id(), input)
    }

    fn add_isolated_edge_default(&mut self, a: T::VP, b: T::VP) -> (T::V, T::V)
    where
        T::EP: DefaultEdgePayload,
    {
        self.add_isolated_edge(a, T::EP::default(), b, T::EP::default())
    }

    fn insert_loop(&mut self, vp: impl IntoIterator<Item = T::VP>) -> T::E
    where
        T::EP: DefaultEdgePayload,
    {
        let (first, last) = self.insert_path(vp);
        self.insert_edge(first, Default::default(), last, Default::default());
        return first;
    }
}
