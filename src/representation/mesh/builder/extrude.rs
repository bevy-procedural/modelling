use crate::{
    math::Transform,
    representation::{
        builder::{AddVertex, CloseFace},
        payload::VertexPayload,
        IndexType, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T> {
    /// Extrudes the given edge in the given direction.
    /// Returns the closing face if it was created.
    pub fn extrude(&mut self, e: T::E, direction: T::Vec, close: bool) -> T::F {
        self.extrude_ex(e, T::Trans::from_translation(direction), close, false)
    }

    /// Extrudes the given edge using the given transformation.
    /// Returns the closing face if it was created.
    pub fn extrude_ex(&mut self, e: T::E, transform: T::Trans, close: bool, curved: bool) -> T::F {
        assert!(self.edge(e).is_boundary_self());

        let first = self.edge(e).origin_id();
        let mut last = first;
        let mut second = first;
        let edges = self.edge(e).edges_face_back(self).collect::<Vec<_>>();
        for i in 0..edges.len() {
            let p = edges[i].origin(self).payload().transform(&transform);
            let curr = self.add_vertex((last, p)).0;
            if i > 0 {
                self.close_face((last, curr, edges[i].origin_id(), curved));
            } else {
                second = curr;
            }
            if i == edges.len() - 1 {
                self.close_face((edges[i].origin_id(), curr, second, curved));
            }
            last = curr;
        }

        if close {
            return self.close_face((self.edge_id_between(second, last), curved));
        }

        return IndexType::max();
    }

    /// Create a vertex at the given position and connect the given face to that vertex.
    pub fn extrude_to_point(&mut self, e: T::E, p: T::VP) -> T::V {
        let mut curr = self.edge(e).origin_id();
        let mut last = self.edge(e).target_id(self);
        let edges = self.edge(e).edges_face_back(self).collect::<Vec<_>>();

        let point = self.add_vertex((last, p)).0;

        for i in 1..edges.len() {
            self.close_face((last, point, curr, false));
            last = curr;
            curr = edges[i].origin_id();
        }
        self.close_face((self.edge_id_between(point, curr), false));
        point
    }

    /// Create a vertex by translating the center of the given face and connect the given face to that vertex.
    pub fn extrude_to_center_point(&mut self, e: T::E, translation: T::Vec) -> T::V {
        let f = if self.edge(e).is_boundary_self() {
            self.close_face((e, true))
        } else {
            self.edge(e).face_id()
        };
        let p = T::VP::from_vec(self.face(f).center(self) + translation);
        self.remove_face(f);
        self.extrude_to_point(e, p)
    }

    /// Extrudes the given face in the given direction.
    pub fn extrude_face(&mut self, f: T::F, direction: T::Vec, close: bool) -> T::F {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude(e, direction, close);
    }

    /// Extrudes the given face in the given direction.
    pub fn extrude_face_ex(&mut self, f: T::F, transform: T::Trans, close: bool, curved: bool) -> T::F {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude_ex(e, transform, close, curved);
    }
}
