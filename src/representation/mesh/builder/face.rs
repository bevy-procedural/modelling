use crate::{
    representation::{Face, IndexType, Mesh, MeshType},
    util::iter::contains_exactly_one,
};

impl<T: MeshType> Mesh<T> {
    /// Close the open boundary with a single face. Doesn't create new edges or vertices.
    pub fn close_hole(&mut self, e: T::E, fp: T::FP, curved: bool) -> T::F {
        let f = self.faces.push(Face::new(e, curved, fp));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }

    /// Close the face by inserting a pair of halfedges, i.e.,
    /// connecting `inside` with the next halfedge to close the face and `outside`
    /// with the next halfedge to complete the outside.
    pub fn close_face(
        &mut self,
        inside: T::E,
        ep1: T::EP,
        outside: T::E,
        ep2: T::EP,
        fp: T::FP,
        curved: bool,
    ) -> T::F {
        let e_inside = self.edge(inside);
        let e_outside = self.edge(outside);
        let v = e_inside.target(self).id();
        let w = e_outside.target(self).id();

        debug_assert!(e_inside.can_reach_back(self, w));
        debug_assert!(e_outside.can_reach_back(self, v));

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
            (other_inside.id(), inside, v, IndexType::max(), ep1),
            (other_outside.id(), outside, w, IndexType::max(), ep2),
        );

        self.edge_mut(other_inside.id()).set_prev(e1);
        self.edge_mut(other_outside.id()).set_prev(e2);
        self.edge_mut(inside).set_next(e1);
        self.edge_mut(outside).set_next(e2);

        let f = self.faces.push(Face::new(inside, curved, fp));

        self.edge(inside)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));

        return f;
    }

    /// Close the face by connecting vertex `from` (coming from `prev`) with vertex `to`.
    /// Inserts a pair of halfedges between these two vertices.
    pub fn close_face_vertices(
        &mut self,
        prev: T::V,
        ep1: T::EP,
        from: T::V,
        ep2: T::EP,
        to: T::V,
        fp: T::FP,
        curved: bool,
    ) -> T::F {
        let inside = self.edge_between(prev, from).unwrap().id();

        debug_assert!(
            contains_exactly_one(self.vertex(to).edges_in(self), |e| {
                e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id())
            }),
            "There mus be exactly one ingoing edge to {} that can reach edge {} but there were {:?}",
            to,
            inside,
            self.vertex(to).edges_in(self).filter(|e| {
                e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id())
            }).collect::<Vec<_>>()
        );

        let outside = self
            .vertex(to)
            .edges_in(self)
            .find(|e| e.is_boundary_self() && e.can_reach(self, self.edge(inside).origin_id()))
            .unwrap()
            .id();

        return self.close_face(inside, ep1, outside, ep2, fp, curved);
    }
}
