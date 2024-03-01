use crate::representation::{
    builder::{AddVertex, CloseFace},
    payload::Payload,
    IndexType, Mesh,
};

impl<E, V, F, P> Mesh<E, V, F, P>
where
    E: IndexType,
    V: IndexType,
    F: IndexType,
    P: Payload,
{
    /// Extrudes the given edge in the given direction.
    /// Returns the closing face if it was created.
    pub fn extrude(&mut self, e: E, direction: P::Vec, close: bool) -> F{
        assert!(self.edge(e).is_boundary_self());

        let first = self.edge(e).origin_id();
        let mut last = first;
        let mut second = first;
        let edges = self.edge(e).edges_face_back(self).collect::<Vec<_>>();
        for i in 0..edges.len() {
            let p = edges[i].origin(self).payload().translate(&direction);

            let curr = self.add_vertex((last, p)).0;
            if i > 0 {
                self.close_face((last, curr, edges[i].origin_id()));
            } else {
                second = curr;
            }
            if i == edges.len() - 1 {
                self.close_face((edges[i].origin_id(), curr, second));
            }
            last = curr;
        }

        if close {
            return self.close_face(self.edge_id_between(second, last));
        }

        return IndexType::max();
    }

    /// Extrudes the given face in the given direction.
    pub fn extrude_face(&mut self, f: F, direction: P::Vec, close: bool) -> F {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude(e, direction, close);
    }
}
