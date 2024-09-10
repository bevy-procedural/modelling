use crate::{
    representation::{Face, IndexType, Mesh, MeshType},
    util::iter::contains_exactly_one,
};

// TODO: Don't use a trait for this!
/// Close a face given some description. Might insert additional edges and vertices.
pub trait CloseFace<Input> {
    /// The type of the face indices
    type F: IndexType;

    /// Close the face and return the index of the new face
    fn close_face(&mut self, input: Input) -> Self::F;
}

impl<T: MeshType> CloseFace<(T::E, T::FP, bool)> for Mesh<T> {
    type F = T::F;

    /// Close the open boundary with a single face
    fn close_face(&mut self, (e, fp, curved): (T::E, T::FP, bool)) -> T::F {
        let f = self.faces.push(Face::new(e, curved, fp));
        self.edge(e)
            .clone()
            .edges_face_mut(self)
            .for_each(|e| e.set_face(f));
        return f;
    }
}

impl<T: MeshType> CloseFace<(T::E, T::EP, T::E, T::EP, T::FP, bool)> for Mesh<T> {
    type F = T::F;

    /// Close the face by connecting `inside` with the next edge to close the face and `outside` with the next edge to complete the outside
    fn close_face(
        &mut self,
        (inside, ep1, outside, ep2, fp, curved): (T::E, T::EP, T::E, T::EP, T::FP, bool),
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
}

impl<T: MeshType> CloseFace<(T::V, T::EP, T::V, T::EP, T::V, T::FP, bool)> for Mesh<T> {
    type F = T::F;

    /// Close the face by connecting the edge from v1 to v2 with vertex w.
    /// TODO: Docs
    fn close_face(
        &mut self,
        (prev, ep1, from, ep2, to, fp, curved): (T::V, T::EP, T::V, T::EP, T::V, T::FP, bool),
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

        return self.close_face((inside, ep1, outside, ep2, fp, curved));
    }
}
