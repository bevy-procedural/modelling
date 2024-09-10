use crate::representation::{IndexType, Mesh, MeshType, Vertex};

// TODO: Don't use a trait for this!
/// Trait for adding a vertex to a mesh and connecting it to the graph
pub trait AddVertex<Input> {
    /// The type of the edge indices
    type E: IndexType;

    /// The type of the vertex indices
    type V: IndexType;

    /// Adds a vertex and connects it to the given graph with a single edge.
    /// The new vertex and the HalfEdges are returned.
    fn add_vertex(&mut self, input: Input) -> (Self::V, Self::E, Self::E);
}

impl<T: MeshType> AddVertex<(T::V, T::VP, T::EP, T::EP)> for Mesh<T> {
    type E = T::E;
    type V = T::V;

    /// Creates a new vertex based on p and connects it to vertex v
    /// TODO: Docs
    fn add_vertex(&mut self, (v, vp, ep1, ep2): (T::V, T::VP, T::EP, T::EP)) -> (T::V, T::E, T::E) {
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

        return self.add_vertex((input, output, vp, ep1, ep2));
    }
}

impl<T: MeshType> AddVertex<(T::E, T::E, T::VP, T::EP, T::EP)> for Mesh<T> {
    type E = T::E;
    type V = T::V;

    /// Adds a vertex with the given payload via a new edge starting in input and ending in output
    fn add_vertex(
        &mut self,
        (input, output, vp, ep1, ep2): (T::E, T::E, T::VP, T::EP, T::EP),
    ) -> (T::V, T::E, T::E) {
        let v = self.edge(input).target(self).id();
        assert!(self.edge(output).origin(self).id() == v);

        let new = self.vertices.allocate();

        let (e1, e2) = self.insert_full_edge(
            (IndexType::max(), input, v, IndexType::max(), ep1),
            (output, IndexType::max(), new, IndexType::max(), ep2),
        );

        self.vertices.set(new, Vertex::new(e2, vp));

        self.edge_mut(input).set_next(e1);
        self.edge_mut(output).set_prev(e2);

        return (new, e1, e2);
    }
}
