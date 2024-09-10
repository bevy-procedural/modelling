use crate::representation::{IndexType, Mesh, MeshType, Vertex};

impl<T: MeshType> Mesh<T> {
    /// Creates a new vertex based on `vp` and connects it to vertex `v` with a pair of halfedges 
    /// TODO: Docs
    pub fn add_vertex_via_vertex(
        &mut self,
        v: T::V,
        vp: T::VP,
        ep1: T::EP,
        ep2: T::EP,
    ) -> (T::V, T::E, T::E) {
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

        return self.add_vertex_via_edge(input, output, vp, ep1, ep2);
    }

    /// Adds a vertex with the given payload via a new edge starting in input and ending in output
    pub fn add_vertex_via_edge(
        &mut self,
        input: T::E,
        output: T::E,
        vp: T::VP,
        ep1: T::EP,
        ep2: T::EP,
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
