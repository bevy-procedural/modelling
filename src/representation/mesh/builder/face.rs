use crate::representation::{payload::Payload, Face, HalfEdge, IndexType, Mesh};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Removes the provided face.
    pub fn remove_face(&mut self, f: F) {
        let face = self.face(f);

        let edge_ids: Vec<_> = face.edges(self).map(|e| e.id()).collect();
        for e in edge_ids {
            self.edge_mut(e).delete_face();
        }

        *self.face_mut(f) = Face::deleted();
        // TODO: add to deleted list for reallocation
    }
}

/// Close a phase given some description. Might insert additional edges and vertices.
pub trait CloseFace<Input> {
    /// The type of the face indices
    type FaceIndex: IndexType;

    /// Close the face and return the index of the new face
    fn close_face(&mut self, input: Input) -> Self::FaceIndex;
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<E> for Mesh<E, V, F, P> {
    type FaceIndex = F;

    /// Close the open boundary with a single face
    fn close_face(&mut self, e: E) -> F {
        let f = F::new(self.faces.len());
        self.faces.push(Face::new(f, e));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<(E, E)> for Mesh<E, V, F, P> {
    type FaceIndex = F;

    /// Close the face by connecting `inside` with the next edge to close the face and `outside` with the next edge to complete the outside
    fn close_face(&mut self, (inside, outside): (E, E)) -> F {
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

        return f;
    }
}

impl<E: IndexType, V: IndexType, F: IndexType, P: Payload> CloseFace<(V, V, V)>
    for Mesh<E, V, F, P>
{
    type FaceIndex = F;

    /// Close the face by connecting the edge from v1 to v2 with vertex w.
    fn close_face(&mut self, (prev, from, to): (V, V, V)) -> F {
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

        return self.close_face((inside, outside));
    }
}
