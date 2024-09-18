use crate::mesh::{
    payload::VertexInterpolator, DefaultEdgePayload, HalfEdge, Mesh, MeshType, Vertex,
};

/// Describes how to subdivide a mesh.
#[derive(Debug, Clone, Copy)]
pub struct SubdivisionDescription {
    b: usize,
    c: usize,
}

impl SubdivisionDescription {
    /// Create a new subdivision description.
    pub fn new(b: usize, c: usize) -> Self {
        assert!(b >= 1);
        Self { b, c }
    }

    /// Get the first number of subdivisions.
    pub fn b(&self) -> usize {
        self.b
    }

    /// Get the second number of subdivisions.
    pub fn c(&self) -> usize {
        self.c
    }

    /// Frequency $v = b + c$ of the subdivision.
    pub fn frequency(&self) -> usize {
        self.b + self.c
    }

    /// Triangulation number $T = b^2 + bc + c^2$ of the subdivision.
    pub fn triangulation_number(&self) -> usize {
        self.b * self.b + self.b * self.c + self.c * self.c
    }
}

impl<T: MeshType> Mesh<T>
where
    T::EP: DefaultEdgePayload,
{
    /// Will insert a new vertex inside this halfedge.
    /// After this, the mesh will be invalid since the twin is not updated!
    fn subdivide_unsafe(&mut self, e: T::E, vp: T::VP) -> T::E {
        let old_edge = self.edge(e).clone();

        let new_v = self.vertices.allocate();
        let new_edge = self.halfedges.allocate();

        self.halfedges.set(
            new_edge,
            HalfEdge::new(
                old_edge.next_id(),
                old_edge.twin_id(),
                old_edge.id(),
                new_v,
                old_edge.face_id(),
                Default::default(),
            ),
        );
        self.vertices.set(new_v, Vertex::new(new_edge, vp));

        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        /*println!(
            "insert\nnew {:?}\nold {:?}",
            self.edge(new_edge),
            self.edge(old_edge.id())
        );*/

        new_edge
    }

    /// Call this on the twin of an halfedge where `subdivide_unsafe` was called
    /// and it will apply the same subdivision on this halfedge making the mesh valid again.
    /// Returns the id of the new edge. If the twin was not subdivided, it will return `None`.
    fn subdivide_unsafe_try_fixup(&mut self, e: T::E) -> Option<T::E> {
        let old_edge = self.edge(e).clone();
        let other_old = old_edge.twin(self);

        // find the "other_new". It has the characteristic property of sharing the same twin with the old edge.
        let mut other_new = other_old.next(self);
        let first_other_new_origin = other_new.origin_id();
        loop {
            if other_new.twin_id() == e {
                break;
            }
            other_new = other_new.twin(self).next(self);
            if other_new.origin_id() != first_other_new_origin {
                // Not a valid wheel
                return None;
            }
            if other_new.prev_id() == other_old.id() {
                // Went a full round
                return None;
            }
        }

        // Insert the new edge
        let new_edge = self.halfedges.allocate();
        self.halfedges.set(
            new_edge,
            HalfEdge::new(
                old_edge.next_id(),
                other_old.id(),
                old_edge.id(),
                other_new.origin_id(),
                old_edge.face_id(),
                Default::default(),
            ),
        );

        // update the neighbors
        self.edge_mut(old_edge.id()).set_twin(other_new.id());
        self.edge_mut(other_old.id()).set_twin(new_edge);
        self.edge_mut(old_edge.next_id()).set_prev(new_edge);
        self.edge_mut(old_edge.id()).set_next(new_edge);

        /*println!(
            "fixup\nnew {:?}\nold {:?}\non  {:?}\noo  {:?}",
            self.edge(new_edge),
            self.edge(old_edge.id()),
            self.edge(other_new.id()),
            self.edge(other_old.id())
        );*/

        Some(new_edge)
    }

    /// Subdivides the mesh with frequency (2,0).
    /// Uses the `vp_builder` to create the new vertex payloads.
    /// Returns a new mesh.
    ///
    /// based on an algorithm developed by Charles Loop in 1987
    pub fn loop_subdivision(&mut self, vp_builder: &impl VertexInterpolator<3, T>) -> &mut Self {
        let fs = self.faces().map(|f| f.id()).collect::<Vec<_>>();
        for face in &fs {
            // get the edge chain
            let edges = self.face(*face).edges(self).collect::<Vec<_>>();
            let vs = edges.iter().map(|e| e.origin_id()).collect::<Vec<_>>();
            assert!(edges.len() == 3);

            // insert an additional vertex for each edge
            for i in 0..3 {
                if self.subdivide_unsafe_try_fixup(edges[i].id()).is_some() {
                    // edge is already subdivided
                    continue;
                }
                let vp = vp_builder.call(
                    self,
                    [
                        (
                            if vs[0] == edges[i].origin_id() || vs[0] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[0],
                        ),
                        (
                            if vs[1] == edges[i].origin_id() || vs[1] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[1],
                        ),
                        (
                            if vs[2] == edges[i].origin_id() || vs[2] == edges[i].target_id(self) {
                                1
                            } else {
                                0
                            },
                            vs[2],
                        ),
                    ],
                );
                self.subdivide_unsafe(edges[i].id(), vp);
            }

            // remove the original face
            let fp = self.remove_face(*face);

            // TODO: cannot clone fp like that!

            // insert the new edges and faces
            for e in &edges {
                self.insert_edge_no_check(
                    e.id(),
                    Default::default(),
                    self.edge(e.id()).prev(self).prev_id(),
                    Default::default(),
                );
                self.close_hole(e.id(), fp, false);
            }
            // fill the center hole
            self.close_hole(self.edge(edges[0].id()).next(self).twin_id(), fp, false);
        }

        self
    }

    /// Subdivides the mesh with frequency (n,m).
    /// Uses the `vp_builder` to create the new vertex payloads.
    /// Returns a new mesh.
    pub fn subdivision_frequency(
        &mut self,
        des: SubdivisionDescription,
        vp_builder: impl VertexInterpolator<3, T>,
    ) -> &mut Self {
        // TODO: for c != 0 we have to shift the triangle. This means we have to build a completely new graph and things become much more complicated
        assert!(des.c == 0);

        // TODO: Apply this to meshes with non-triangular faces by triangulating them. Usually, you want to insert Center points / Steiner points to get nearly equilateral triangles.

        assert!(des.b & (des.b - 1) == 0, "todo: odd subdivision frequency");
        let num_faces = self.num_faces();

        let mut b = des.b;
        while b > 1 {
            self.loop_subdivision(&vp_builder);
            if b == 1 {
                break;
            }
            assert!(b % 2 == 0, "todo: odd subdivision frequency");
            b = b / 2
        }

        debug_assert_eq!(self.num_faces(), num_faces * des.triangulation_number());

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::{bevy::BevyMesh3d, payload::{vertex_payload::BevyVertexPayload, HasPosition}};

    #[test]
    fn subdivide_and_fixup() {
        let mut mesh = BevyMesh3d::regular_polygon(1.0, 3);

        let e = mesh.edge(0).clone();
        let vp = BevyVertexPayload::from_pos(
            *e.origin(&mesh).pos() * 0.5 + *e.origin(&mesh).pos() * 0.5,
        );

        mesh.subdivide_unsafe(e.id(), vp);
        assert!(mesh.subdivide_unsafe_try_fixup(e.twin_id()).is_some());

        println!("mesh: {}", mesh);

        assert!(mesh.check().is_ok());
    }
}
