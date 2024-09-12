use crate::{
    math::Transform,
    representation::{
        payload::VertexPayload, DefaultEdgePayload, DefaultFacePayload, IndexType, Mesh, MeshType,
    },
};

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
    T::FP: DefaultFacePayload,
{
    /// Extrudes the given edge in the given direction.
    /// Returns the closing face if it was created.
    pub fn extrude(&mut self, e: T::E, direction: T::Vec, close: bool) -> T::F {
        self.extrude_ex(e, T::Trans::from_translation(direction), close, false)
    }

    /// Extrudes the given edge using the given transformation.
    /// Returns the closing face if it was created.
    pub fn extrude_ex(&mut self, e: T::E, transform: T::Trans, close: bool, curved: bool) -> T::F {
        assert!(self.edge(e).is_boundary_self());

        // TODO: use the hem function! 
        // TODO: Also, make an intermediate version that makes closed hems and version that maps the vertices instead of taking an iterator

        let first = self.edge(e).origin_id();
        let mut last = first;
        let mut second = first;
        let edges = self.edge(e).edges_face_back(self).collect::<Vec<_>>();
        for i in 0..edges.len() {
            let p = edges[i].origin(self).payload().transform(&transform);
            let curr = self.add_vertex_via_vertex_default(last, p).0;
            if i > 0 {
                self.close_face_vertices_default(last, curr, edges[i].origin_id(), curved);
            } else {
                second = curr;
            }
            if i == edges.len() - 1 {
                self.close_face_vertices_default(edges[i].origin_id(), curr, second, curved);
            }
            last = curr;
        }

        if close {
            return self.close_hole(
                self.shared_edge_id(second, last).unwrap(),
                Default::default(),
                curved,
            );
        }

        return IndexType::max();
    }

    /// Create a vertex at the given position and connect the given face to that vertex.
    pub fn extrude_to_point(&mut self, e: T::E, p: T::VP) -> T::V {
        // TODO: make this a similar api as fill_hole_with_vertex

        let mut curr = self.edge(e).origin_id();
        let mut last = self.edge(e).target_id(self);
        let edges = self.edge(e).edges_face_back(self).collect::<Vec<_>>();

        let point = self.add_vertex_via_vertex_default(last, p).0;

        for i in 1..edges.len() {
            self.close_face_vertices_default(last, point, curr, false);
            last = curr;
            curr = edges[i].origin_id();
        }
        self.close_hole(
            self.shared_edge_id(point, curr).unwrap(),
            Default::default(),
            false,
        );
        point
    }

    /// Create a vertex by translating the center of the given face and connect the given face to that vertex.
    pub fn extrude_to_center_point(&mut self, e: T::E, translation: T::Vec) -> T::V {
        let f = if self.edge(e).is_boundary_self() {
            self.close_hole(e, Default::default(), true)
        } else {
            self.edge(e).face_id()
        };
        let p = T::VP::from_pos(self.face(f).center(self) + translation);
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
    pub fn extrude_face_ex(
        &mut self,
        f: T::F,
        transform: T::Trans,
        close: bool,
        curved: bool,
    ) -> T::F {
        let e = self.face(f).edge_id();
        self.remove_face(f);
        return self.extrude_ex(e, transform, close, curved);
    }

    /// This will walk along the given boundary and add a "hem" made from triangles.
    /// The payloads are given using the iterator.
    ///
    /// `start` must be an edge on the boundary pointing to the first vertex to be connected with the hem.
    ///
    /// Returns the edge pointing from the first inserted vertex to the target of `start`.
    /// If the iterator is empty, return `start` instead.
    ///
    /// If `shift` is true, the first inserted triangle will be with the tip pointing to the target of `start`.
    /// Otherwise, the first triangle will include the edge `start`.
    /// This doesn't affect the number of triangles but shifts the hem by one.
    pub fn triangle_hem(
        &mut self,
        start: T::E,
        shift: bool,
        vp: impl IntoIterator<Item = T::VP>,
    ) -> T::E {
        // TODO: a more efficient implementation could bulk-insert everything at once
        // TODO: assertions

        let mut input = start;
        let mut first = true;
        let mut iter = vp.into_iter();
        let mut pos = iter.next();
        let mut ret = start;

        if shift && pos.is_some() {
            let output = self.edge(input).next_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());
            first = false;
            ret = self.edge(output).prev_id();
            pos = iter.next();
        }

        while pos.is_some() {
            let output = self.edge(input).next_id();
            self.add_vertex_via_edge_default(input, output, pos.unwrap());

            let ip = self.edge(input);
            input = ip.prev_id();
            let ip_next = ip.next_id();

            // the first one shouldn't connect to the previous
            if !first {
                self.close_face_default(output, ip_next, false);
            } else {
                ret = self.edge(output).prev_id();
            }

            pos = iter.next();
            // the last one also shouldn't connect to the next
            if pos.is_some() || shift {
                self.close_face_default(ip_next, input, false);
            }

            first = false;
        }

        ret
    }

    /// Like `triangle_hem` but for quad faces.
    pub fn quad_hem(&mut self, start: T::E, vp: impl IntoIterator<Item = T::VP>) -> T::E {
        // TODO: assertions
        // TODO: Can be written faster without the "smart" edge functions but by bulk inserting into the mesh

        let mut iter = vp.into_iter();

        let mut input = start;

        if let Some(vp) = iter.next() {
            self.add_vertex_via_edge_default(input, self.edge(start).next_id(), vp);
        }

        let mut ret = start;
        for vp in iter {
            input = self.edge(input).prev_id();
            let output = self.edge(input).next(self);
            self.add_vertex_via_edge_default(input, output.id(), vp);
            self.close_face_default(output.next_id(), self.edge(input).next_id(), false);
            if ret == start {
                ret = self.edge(input).next(self).next_id();
            }
        }

        ret
    }

    /// Assumes `start` is on the boundary of the edge.
    /// Will insert a vertex `center` with the given vp and fill the hole along the boundary with triangles connected to the center vertex.
    /// Returns the vertex.
    pub fn fill_hole_with_vertex(&mut self, start: T::E, center: T::VP) -> T::V {
        // TODO: find a better name!
        let e0 = self.edge(start);
        let origin = e0.origin_id();
        let mut input = self.edge(start).prev_id();
        let (v, _, _) = self.add_vertex_via_edge_default(input, start, center);
        loop {
            let e = self.edge(input);
            if e.origin_id() == origin {
                break;
            }
            input = e.prev_id();
            self.close_face_default(self.edge(input).next(&self).next_id(), input, false);
        }
        self.close_hole(input, Default::default(), false);

        v
    }
}
