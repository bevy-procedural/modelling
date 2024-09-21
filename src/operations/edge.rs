use crate::mesh::{DefaultEdgePayload, HalfEdge, IndexType, Mesh, MeshType, Vertex};

// The simplest non-empty mesh: a single edge with two vertices
impl<T: MeshType> From<(T::VP, T::EP, T::VP, T::EP)> for T::Mesh
where
    T::EP: DefaultEdgePayload,
{
    fn from((a, epa, b, epb): (T::VP, T::EP, T::VP, T::EP)) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_isolated_edge(a, epa, b, epb);
        return mesh;
    }
}
