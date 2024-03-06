use crate::representation::{payload::Payload, IndexType, Mesh, Vertex};

/// Trait for adding a vertex to a mesh and connecting it to the graph
pub trait AddVertex<Input> {
    /// The type of the edge indices
    type EdgeIndex: IndexType;

    /// The type of the vertex indices
    type VertexIndex: IndexType;

    /// Adds a vertex and connects it to the given graph with a single edge.
    /// The new vertex and the HalfEdges are returned.
    fn add_vertex(&mut self, input: Input)
        -> (Self::VertexIndex, Self::EdgeIndex, Self::EdgeIndex);
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> AddVertex<(V, P)> for Mesh<E, V, F, P> {
    type EdgeIndex = E;
    type VertexIndex = V;

    /// Creates a new vertex based on p and connects it to vertex v
    fn add_vertex(&mut self, (v, payload): (V, P)) -> (V, E, E) {
        let (input, output) = if self.vertex(v).has_only_one_edge(self) {
            let e = self.vertex(v).edge(self);
            (e.twin_id(), e.id())
        } else {
            let Some(boundary) = self.vertex(v).edges(self).find(|e| e.is_boundary_self()) else {
                panic!("Vertex is not a boundary vertex");
            };
            debug_assert!(
                self.vertex(v)
                    .edges(self)
                    .filter(|e| e.is_boundary_self())
                    .count()
                    == 1
            );
            (boundary.prev_id(), boundary.id())
        };

        assert!(self.edge(input).is_boundary_self());
        assert!(self.edge(output).is_boundary_self());

        return self.add_vertex((input, output, payload));
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> AddVertex<(E, E, P)>
    for Mesh<E, V, F, P>
{
    type EdgeIndex = E;
    type VertexIndex = V;

    /// Adds a vertex with the given payload via a new edge starting in input and ending in output
    fn add_vertex(&mut self, (input, output, payload): (E, E, P)) -> (V, E, E) {
        let v = self.edge(input).target(self).id();
        assert!(self.edge(output).origin(self).id() == v);

        let new = self.vertices.allocate();

        let (e1, e2) = self.insert_full_edge(
            (IndexType::max(), input, v, IndexType::max()),
            (output, IndexType::max(), new, IndexType::max()),
        );

        self.vertices.set(new, Vertex::new(e2, payload));

        self.edge_mut(input).set_next(e1);
        self.edge_mut(output).set_prev(e2);

        return (new, e1, e2);
    }
}
