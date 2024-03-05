use crate::representation::{payload::Payload, Face, IndexType, Mesh};

/// Close a phase given some description. Might insert additional edges and vertices.
pub trait CloseFace<Input> {
    /// The type of the face indices
    type FaceIndex: IndexType;

    /// Close the face and return the index of the new face
    fn close_face(&mut self, input: Input) -> Self::FaceIndex;
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<(E, bool)>
    for Mesh<E, V, F, P>
{
    type FaceIndex = F;

    /// Close the open boundary with a single face
    fn close_face(&mut self, (e, curved): (E, bool)) -> F {
        let f = self.faces.push(Face::new(e, curved));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<(E, E, bool)>
    for Mesh<E, V, F, P>
{
    type FaceIndex = F;

    /// Close the face by connecting `inside` with the next edge to close the face and `outside` with the next edge to complete the outside
    fn close_face(&mut self, (inside, outside, curved): (E, E, bool)) -> F {
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

        let (e1, e2) = self.insert_full_edge(
            (other_inside.id(), inside, v, IndexType::max()),
            (other_outside.id(), outside, w, IndexType::max()),
        );

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        let f = self.faces.push(Face::new(inside, curved));

        self.edge(inside)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));

        return f;
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<(V, V, V, bool)>
    for Mesh<E, V, F, P>
{
    type FaceIndex = F;

    /// Close the face by connecting the edge from v1 to v2 with vertex w.
    fn close_face(&mut self, (prev, from, to, curved): (V, V, V, bool)) -> F {
        let inside = self.edge_between(prev, from).unwrap().id();

        assert!(
            self.vertex(to)
                .edges_in(self)
                .filter(|e| {
                    e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id())
                })
                .count()
                == 1,
        );

        let outside = self
            .vertex(to)
            .edges_in(self)
            .find(|e| e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id()))
            .unwrap()
            .id();

        return self.close_face((inside, outside, curved));
    }
}
