use core::panic;

use super::Mesh;
use crate::representation::{payload::Payload, Face, HalfEdge, IndexType, Vertex};

// The simplest non-empty mesh: a single edge with two vertices
impl<E, V, F, P> From<(P, P)> for Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    fn from((a, b): (P, P)) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_isolated_edge(a, b);
        return mesh;
    }
}

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Inserts vertices a and b and adds an isolated edge between a and b.
    pub fn add_isolated_edge(&mut self, a: P, b: P) -> (V, V) {
        let e0 = E::new(self.edges.len());
        let e1 = E::new(self.edges.len() + 1);
        let v0 = V::new(self.vertices.len());
        let v1 = V::new(self.vertices.len() + 1);

        self.vertices.push(Vertex::new(v0, e0, v0, a));
        self.vertices.push(Vertex::new(v1, e1, v1, b));
        self.edges
            .push(HalfEdge::new(e0, e1, e1, e1, v0, IndexType::max()));
        self.edges
            .push(HalfEdge::new(e1, e0, e0, e0, v1, IndexType::max()));

        return (v0, v1);
    }

    /// Creates a new vertex based on p and connects it to vertex v
    pub fn add_vertex_auto(&mut self, v: V, payload: P) -> (V, E, E) {
        let (input, output) = if self.vertex(v).has_only_one_edge(self) {
            let e = self.vertex(v).edge(self);
            (e.twin_id(), e.id())
        } else {
            let Some(boundary) = self.vertex(v).edges(self).find(|e| e.is_boundary_self()) else {
                panic!("Vertex is not a boundary vertex");
            };
            assert!(
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

        return self.add_vertex(input, output, payload);
    }

    /// Adds a vertex with the given payload via a new edge starting in input and ending in output
    pub fn add_vertex(&mut self, input: E, output: E, payload: P) -> (V, E, E) {
        let new = V::new(self.vertices.len());
        let e1 = E::new(self.edges.len());
        let e2 = E::new(self.edges.len() + 1);

        let v = self.edge(input).target(self).id();
        assert!(self.edge(output).origin(self).id() == v);

        self.vertices.push(Vertex::new(new, e2, new, payload));
        self.edges
            .push(HalfEdge::new(e1, e2, e2, input, v, IndexType::max()));
        self.edges
            .push(HalfEdge::new(e2, output, e1, e1, new, IndexType::max()));

        self.edge_mut(input).set_next(e1);
        self.edge_mut(output).set_prev(e2);

        return (new, e1, e2);
    }

    /// Close the open boundary with a single face
    pub fn close_final(&mut self, e: E) -> F {
        let f = F::new(self.faces.len());
        self.faces.push(Face::new(f, e));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }

    /// Close the face by connecting the edge from v1 to v2 with vertex w.
    pub fn close_face3(&mut self, prev: V, from: V, to: V) -> (F, E, E) {
        self.close_auto(self.edge_between(prev, from).unwrap().id(), to)
    }

    /// Close the face by connecting the edge `inside` with vertex w.
    /// Will automatically find the outside edge.
    pub fn close_auto(&mut self, inside: E, w: V) -> (F, E, E) {
        assert!(
            self.vertex(w)
                .edges_in(self)
                .filter(|e| {
                    e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id())
                })
                .count()
                == 1,
        );

        let outside = self
            .vertex(w)
            .edges_in(self)
            .find(|e| e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id()))
            .unwrap()
            .id();

        return self.close_face(inside, outside);
    }

    /// Close the face by connecting `inside` with the next edge to close the face and `outside` with the next edge to complete the outside
    pub fn close_face(&mut self, inside: E, outside: E) -> (F, E, E) {
        let e1 = E::new(self.edges.len());
        let e2 = E::new(self.edges.len() + 1);
        let f = F::new(self.faces.len());
        let e_inside = self.edge(inside);
        let e_outside = self.edge(outside);
        let v = e_inside.target(self).id();
        let w = e_outside.target(self).id();

        assert!(e_inside.can_reach_back(self, w));
        assert!(e_outside.can_reach_back(self, v));

        let other_inside = self
            .edge(inside)
            .edges_face_back(self)
            .find(|e| e.origin_id() == w)
            .unwrap();

        let other_outside = self
            .edge(inside)
            .edges_face_back(self)
            .find(|e| e.origin_id() == v)
            .unwrap();

        self.edges.push(HalfEdge::new(
            e1,
            other_inside.id(),
            e2,
            inside,
            v,
            IndexType::max(),
        ));
        self.edges.push(HalfEdge::new(
            e2,
            other_outside.id(),
            e1,
            outside,
            w,
            IndexType::max(),
        ));

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        self.faces.push(Face::new(f, inside));

        self.edge(inside)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));

        return (f, e1, e2);
    }
}
