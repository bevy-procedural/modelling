use crate::{
    math::{HasZero, Scalar, Vector},
    representation::{
        payload::VertexPayload, DefaultEdgePayload, HalfEdge, IndexType, Mesh, MeshType, Vertex,
    },
};

// The simplest non-empty mesh: a single edge with two vertices
impl<T: MeshType> From<(T::VP, T::EP, T::VP, T::EP)> for Mesh<T>
where
    T::EP: DefaultEdgePayload,
{
    fn from((a, epa, b, epb): (T::VP, T::EP, T::VP, T::EP)) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_isolated_edge(a, epa, b, epb);
        return mesh;
    }
}

impl<T: MeshType> Mesh<T> {
    /// Inserts vertices a and b and adds an isolated edge between a and b.
    pub fn add_isolated_edge(
        &mut self,
        a: T::VP,
        epa: T::EP,
        b: T::VP,
        epb: T::EP,
    ) -> (T::V, T::V) {
        let v0 = self.vertices.allocate();
        let v1 = self.vertices.allocate();
        let (e0, e1) = self.insert_edge_unsafe(
            (
                IndexType::max(),
                IndexType::max(),
                v0,
                IndexType::max(),
                epa,
            ),
            (
                IndexType::max(),
                IndexType::max(),
                v1,
                IndexType::max(),
                epb,
            ),
        );
        self.vertices.set(v0, Vertex::new(e0, a));
        self.vertices.set(v1, Vertex::new(e1, b));

        (v0, v1)
    }

    /// Connects the vertices v0 and v1 with an edge and returns the edge id.
    /// This will not close any face, i.e., v0 and v1 must be in different connected components.
    /// Hence, they must be also on the boundary of each connected components.
    pub fn insert_edge_between(
        &mut self,
        origin0: T::V,
        ep0: T::EP,
        origin1: T::V,
        ep1: T::EP,
    ) -> (T::E, T::E) {
        debug_assert!(
            self.has_vertex(origin0),
            "First Vertex {} does not exist",
            origin0
        );
        debug_assert!(
            self.has_vertex(origin1),
            "Second Vertex {} does not exist",
            origin1
        );
        debug_assert!(
            self.shared_edge(origin0, origin1).is_none(),
            "There is already an edge between first vertex {} and second vertex {}",
            origin0,
            origin1
        );
        debug_assert!(
            self.shared_edge(origin1, origin0).is_none(),
            "There is already an edge between second vertex {} and first vertex {}",
            origin1,
            origin0
        );
        debug_assert!(
            self.shortest_path(origin0, origin1).is_none(),
            "Vertices {} and {} must be in different connected components",
            origin0,
            origin1
        );

        // We are connecting two vertices at the boundary of two connected components.
        // Hence, the edge from v0 to v1 will come from the ingoing boundary
        // edge of v0 and go to the outgoing boundary edge of v1.

        // TODO: When allowing non-manifold meshes, they vertices might not be at boundary and in the same component, e.g., we could allow an edge from one interior point to another.

        let next0 = self
            .vertex(origin1)
            .outgoing_boundary_edge(self)
            .expect("There must be an outgoing boundary edge at vertex v0");
        let prev0 = self
            .vertex(origin0)
            .ingoing_boundary_edge(self)
            .expect("There must be an ingoing boundary edge at vertex v1");
        let next1 = self
            .vertex(origin0)
            .outgoing_boundary_edge(self)
            .expect("There must be an outgoing boundary edge at vertex v1");
        let prev1 = self
            .vertex(origin1)
            .ingoing_boundary_edge(self)
            .expect("There must be an ingoing boundary edge at vertex v0");

        let (e0, e1) = self.insert_edge_unsafe(
            (next0, prev0, origin0, IndexType::max(), ep0),
            (next1, prev1, origin1, IndexType::max(), ep1),
        );

        self.edge_mut(next0).set_prev(e0);
        self.edge_mut(prev0).set_next(e0);
        self.edge_mut(next1).set_prev(e1);
        self.edge_mut(prev1).set_next(e1);

        (e0, e1)
    }

    /// Will allocate two edges and return them as a tuple.
    /// You can set next and prev to IndexType::max() to insert the id of the twin edge there.
    pub fn insert_edge(
        &mut self,
        (next1, prev1, origin1, face1, ep1): (T::E, T::E, T::V, T::F, T::EP),
        (next2, prev2, origin2, face2, ep2): (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E) {
        debug_assert!(
            self.has_vertex(origin1),
            "First Vertex {} does not exist",
            origin1
        );
        debug_assert!(
            self.has_vertex(origin2),
            "Second Vertex {} does not exist",
            origin2
        );
        debug_assert!(
            self.shared_edge(origin1, origin2).is_none(),
            "There is already an edge between first vertex {} and second vertex {}",
            origin1,
            origin2
        );
        debug_assert!(
            self.shared_edge(origin2, origin1).is_none(),
            "There is already an edge between second vertex {} and first vertex {}",
            origin2,
            origin1
        );

        // TODO: validate that the setting of IndexType::Max() is valid!

        self.insert_edge_unsafe(
            (next1, prev1, origin1, face1, ep1),
            (next2, prev2, origin2, face2, ep2),
        )
    }

    /// like insert_edge, but without assertions.
    /// You have to make sure that the vertices will not be deleted afterwards and that there is no halfedge between them yet.
    pub fn insert_edge_unsafe(
        &mut self,
        (next1, prev1, origin1, face1, ep1): (T::E, T::E, T::V, T::F, T::EP),
        (next2, prev2, origin2, face2, ep2): (T::E, T::E, T::V, T::F, T::EP),
    ) -> (T::E, T::E) {
        let e1 = self.halfedges.allocate();
        let e2 = self.halfedges.allocate();
        self.halfedges.set(
            e1,
            HalfEdge::new(
                if next1 == IndexType::max() { e2 } else { next1 },
                e2,
                if prev1 == IndexType::max() { e2 } else { prev1 },
                origin1,
                face1,
                ep1,
            ),
        );
        self.halfedges.set(
            e2,
            HalfEdge::new(
                if next2 == IndexType::max() { e1 } else { next2 },
                e1,
                if prev2 == IndexType::max() { e1 } else { prev2 },
                origin2,
                face2,
                ep2,
            ),
        );

        (e1, e2)
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Generate a straight line from `from` to `to` with `n` vertices and return the vertices.
    pub fn insert_line(self: &mut Self, from: T::Vec, to: T::Vec, n: usize) -> Vec<T::V> {
        assert!(n >= 2);
        let step = (to - from) / T::S::from_usize(n - 1);
        debug_assert!(step.length_squared() > T::S::ZERO);
        let nth_payload = |i: usize| from + step * T::S::from_usize(i);
        let mut vs: Vec<<T as MeshType>::V> = Vec::with_capacity(n);
        let (v0, mut v) =
            self.add_isolated_edge_default(T::VP::from_pos(from), T::VP::from_pos(nth_payload(1)));
        vs.push(v0);
        vs.push(v);
        for i in 2..n {
            v = self
                .add_vertex_via_vertex_default(v, T::VP::from_pos(nth_payload(i)))
                .0;
            vs.push(v);
        }
        vs
    }

    /// Generate a straight line from `from` to `to` with `n` additional vertices and return the added vertices. Assumes `from` already exists.
    pub fn append_line(self: &mut Self, from: T::V, to: T::Vec, n: usize) -> Vec<T::V> {
        // TODO: Modify this to be based on a iterator of payloads
        // TODO: insert_line should be based on that
        assert!(n >= 1);
        let p = *self.vertex(from).payload().pos();
        let step = (to - p) / T::S::from_usize(n);
        debug_assert!(step.length_squared() > T::S::ZERO);
        let nth_payload = |i: usize| p + step * T::S::from_usize(i);
        let mut vs: Vec<<T as MeshType>::V> = Vec::with_capacity(n);
        let mut v = from;
        for i in 1..=n {
            v = self
                .add_vertex_via_vertex_default(v, T::VP::from_pos(nth_payload(i)))
                .0;
            vs.push(v);
        }
        vs
    }

    /// Same as `add_isolated_edge` but with default edge payloads
    pub fn add_isolated_edge_default(&mut self, a: T::VP, b: T::VP) -> (T::V, T::V) {
        self.add_isolated_edge(a, T::EP::default(), b, T::EP::default())
    }
}
